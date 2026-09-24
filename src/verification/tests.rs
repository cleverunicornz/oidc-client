use crate::core::{
    CoreEdDsaPrivateSigningKey, CoreHmacKey, CoreIdToken, CoreIdTokenClaims, CoreIdTokenVerifier,
    CoreJsonWebKey, CoreJsonWebKeySet, CoreJsonWebKeyType, CoreJsonWebKeyUse,
    CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm, CoreRsaPrivateSigningKey,
    CoreUserInfoClaims, CoreUserInfoJsonWebToken, CoreUserInfoVerifier,
};
use crate::helpers::{Base64UrlEncodedBytes, Timestamp};
use crate::jwt::tests::{TEST_RSA_PRIV_KEY, TEST_RSA_PUB_KEY};
use crate::jwt::{
    JsonWebToken, JsonWebTokenHeader, JsonWebTokenJsonPayloadSerde, JsonWebTokenType,
};
use crate::verification::{AudiencesClaim, IssuerClaim, JwtClaimsVerifier};
use crate::{
    AccessToken, AccessTokenHash, Audience, AuthenticationContextClass, AuthorizationCode,
    ClaimsVerificationError, ClientId, ClientSecret, EndUserName, IssuerUrl, JsonWebKey,
    JsonWebKeyId, Nonce, PrivateSigningKey, SignatureVerificationError, StandardClaims,
    SubjectIdentifier, UserInfoError,
};

use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

type CoreJsonWebTokenHeader =
    JsonWebTokenHeader<CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm>;

type CoreJwtClaimsVerifier<'a> = JwtClaimsVerifier<'a, CoreJsonWebKey>;

fn assert_unsupported<T>(result: Result<T, ClaimsVerificationError>, expected_substr: &str) {
    match result {
        Err(ClaimsVerificationError::Unsupported(msg)) => {
            assert!(msg.contains(expected_substr))
        }
        Err(err) => panic!("unexpected error: {:?}", err),
        Ok(_) => panic!("validation should fail"),
    }
}

#[test]
fn test_jose_header() {
    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let verifier = CoreJwtClaimsVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
    );

    // Happy path JWT
    verifier
        .validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>("{\"alg\":\"RS256\", \"typ\":\"JWT\"}")
                .expect("failed to deserialize"),
        )
        .expect("JWT type should be allowed but is not");

    verifier
        .validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>("{\"alg\":\"RS256\", \"typ\":\"jwt\"}")
                .expect("failed to deserialize"),
        )
        .expect("JWT type should be allowed but is not");

    verifier
        .validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>(
                "{\"alg\":\"RS256\", \"typ\":\"application/JWT\"}",
            )
            .expect("failed to deserialize"),
        )
        .expect("JWT type should be allowed but is not");

    // Happy path JOSE
    verifier
        .validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>(
                "{\"alg\":\"RS256\", \"typ\":\"jose\"}",
            )
            .expect("failed to deserialize"),
        )
        .expect("JWT type should be allowed but is not");

    verifier
        .validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>(
                "{\"alg\":\"RS256\", \"typ\":\"JOSE\"}",
            )
            .expect("failed to deserialize"),
        )
        .expect("JWT type should be allowed but is not");

    // Unexpected JWT type.
    assert_unsupported(
        verifier.validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>(
                "{\"alg\":\"RS256\",\"typ\":\"NOT_A_JWT\"}",
            )
            .expect("failed to deserialize"),
        ),
        "unsupported JWT type",
    );

    // No typ at all.
    verifier
        .validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>("{\"alg\":\"RS256\"}")
                .expect("failed to deserialize"),
        )
        .expect("JWT type should be allowed but is not");

    // Specific JWT type from list.
    {
        let custom_verifier = CoreJwtClaimsVerifier::new(
            client_id.clone(),
            issuer.clone(),
            CoreJsonWebKeySet::new(vec![]),
        )
        .set_allowed_jose_types(vec![
            JsonWebTokenType::new("application/NOT_A_JWT".to_string())
                .normalize()
                .unwrap(),
            JsonWebTokenType::new("APPLICATION/AT+jwt".to_string())
                .normalize()
                .unwrap(),
            JsonWebTokenType::new("X-special-app/jwt;param=some".to_string())
                .normalize()
                .unwrap(),
            JsonWebTokenType::new("X-special-app/jwt".to_string())
                .normalize()
                .unwrap(),
        ]);

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"NOT_A_JWT\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"application/NOT_A_JWT\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");

        assert_unsupported(
            custom_verifier.validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"application/NOT_A_JWT;bla=test\"}",
                )
                .expect("failed to deserialize"),
            ),
            "unsupported JWT type",
        );

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"application/at+jwt\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");

        assert_unsupported(
            custom_verifier.validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"NOT_A_JWT_REALLY\"}",
                )
                .expect("failed to deserialize"),
            ),
            "unsupported JWT type",
        );

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"X-special-app/jwt\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"X-special-app/jwt;param=some\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");

        assert_unsupported(
            custom_verifier.validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"X-special-app/jwt;param=other\"}",
                )
                .expect("failed to deserialize"),
            ),
            "unsupported JWT type",
        );
    }

    // Allow all JWT types.
    {
        let custom_verifier = CoreJwtClaimsVerifier::new(
            client_id.clone(),
            issuer.clone(),
            CoreJsonWebKeySet::new(vec![]),
        )
        .allow_all_jose_types();

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"NOT_A_JWT\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"application/at+jwt\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"application/at+jwt;oidc=cool\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");

        custom_verifier
            .validate_jose_header(
                &serde_json::from_str::<CoreJsonWebTokenHeader>(
                    "{\"alg\":\"RS256\",\"typ\":\"NOT_A_JWT_REALLY\"}",
                )
                .expect("failed to deserialize"),
            )
            .expect("JWT type should be allowed but is not");
    }

    // Nested JWTs.
    assert_unsupported(
        verifier.validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>("{\"alg\":\"RS256\",\"cty\":\"JWT\"}")
                .expect("failed to deserialize"),
        ),
        "nested JWT",
    );
    assert_unsupported(
        verifier.validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>(
                "{\"alg\":\"RS256\",\"cty\":\"NOT_A_JWT\"}",
            )
            .expect("failed to deserialize"),
        ),
        "unsupported JWT content type",
    );

    // Critical fields. Adapted from https://tools.ietf.org/html/rfc7515#appendix-E
    assert_unsupported(
        verifier.validate_jose_header(
            &serde_json::from_str::<CoreJsonWebTokenHeader>(
                "{\
                     \"alg\":\"RS256\",\
                     \"crit\":[\"http://example.invalid/UNDEFINED\"],\
                     \"http://example.invalid/UNDEFINED\":true\
                     }",
            )
            .expect("failed to deserialize"),
        ),
        "critical JWT header fields are unsupported",
    );
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct TestClaims {
    aud: Option<Vec<Audience>>,
    iss: Option<IssuerUrl>,
    payload: String,
}
impl AudiencesClaim for TestClaims {
    fn audiences(&self) -> Option<&Vec<Audience>> {
        self.aud.as_ref()
    }
}
impl IssuerClaim for TestClaims {
    fn issuer(&self) -> Option<&IssuerUrl> {
        self.iss.as_ref()
    }
}
type TestClaimsJsonWebToken = JsonWebToken<
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
    TestClaims,
    JsonWebTokenJsonPayloadSerde,
>;

#[test]
fn test_jwt_verified_claims() {
    let rsa_key =
        serde_json::from_str::<CoreJsonWebKey>(TEST_RSA_PUB_KEY).expect("deserialization failed");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let verifier = CoreJwtClaimsVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
    );

    // Invalid JOSE header.
    assert_unsupported(
        verifier.verified_claims(
            serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
                "eyJhbGciOiJBMjU2R0NNIiwiY3R5IjoiSldUIn0.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Im\
                     h0dHBzOi8vZXhhbXBsZS5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.YmFkX2hhc2g"
                    .to_string(),
            ))
            .expect("failed to deserialize"),
        ),
        "nested JWT",
    );

    // JWE-encrypted JWT.
    assert_unsupported(
        verifier.verified_claims(
            serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
                "eyJhbGciOiJBMjU2R0NNIn0.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbX\
                     BsZS5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.YmFkX2hhc2g"
                    .to_string(),
            ))
            .expect("failed to deserialize"),
        ),
        "JWE encryption",
    );

    // Wrong issuer.
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vYXR0YWNrZXIuY\
                 29tIiwicGF5bG9hZCI6ImhlbGxvIHdvcmxkIn0.YmFkX2hhc2g"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::InvalidIssuer(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Missing issuer.
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sInBheWxvYWQiOiJoZWxsbyB3b3JsZCJ9.\
                 YmFkX2hhc2g"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::InvalidIssuer(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Ignore missing issuer.
    verifier
            .clone()
            .require_issuer_match(false)
            .verified_claims(
            serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
                "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sInBheWxvYWQiOiJoZWxsbyB3b3JsZCJ9.\
                 nv09al63NNDfb8cF3IozegXKbPaUC08zknRPKmQ5qKgXv80hjVxknkpRz7BxocB3JYTBjhYd0gyN9wAuJj\
                 byZ1QaUC14HOB83awAGbehy5yFLkLadTfPT7-siBCvE2V7AF73a_21YvwdkKmJ-RaKWHzFnG8CDmioma3X\
                 cWyrsdRLgvUkrWllajLRo8DCIXQ8OuZo1_o4n17PSlPxSkhKIrgaWCvG6tan40Y_1DZOFv47bx4hQUGd-J\
                 h2aEjiwn65WV3M_Xb2vQMP7VgYNVaNlfxzpL4yDASItbPMWaXBt3ZUa_IOGoSx2GMnPkrQ4xp56qUth6U7\
                 esWPqRSqqolnHg"
                    .to_string(),
            )).expect("failed to deserialize"),
        ).expect("verification should succeed");

    // Wrong audience.
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsib3RoZXJfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                 S5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.YmFkX2hhc2g"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::InvalidAudience(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Missing audience.
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJodHRwczovL2V4YW1wbGUuY29tIiwicGF5bG9hZCI6ImhlbGxvI\
                 HdvcmxkIn0.YmFkX2hhc2g"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::InvalidAudience(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Ignore missing audience.
    verifier
        .clone()
        .require_audience_match(false)
        .verified_claims(
            serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
                "eyJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJodHRwczovL2V4YW1wbGUuY29tIiwicGF5bG9hZCI6Imhlb\
                     GxvIHdvcmxkIn0.lP-Z_zGPNoKIbLQsnrZc2LAc5qJrKyb7t07ZtJUKVhcwHiCUou4bBhq5RHlElCh\
                     0ElRRP6I25lp6UszkRvIC46UV3GVze0x73kVkHSvCVI7MO75LbL9BRqrm5b4CN2zCiFBY8-EwTXnJd\
                     Ri0d_U8K29TV24L2I-Z5ZILebwUue1N59AGDjx2yYLFx5NOw3TUsPyscG62aZAT321pL_jcYwTWTWw\
                     2FYm07zguwx-PUTZwGXlJiOgXQqRIbY_1bS3I_D8UWsmEB3DmV0f9z-iklgIPFawa4wHaE-hpzBAEx\
                     pSieyOavA5pl0Se3XRYA-CkdDVgzG0Pt4IdnxFanfUXTw"
                    .to_string(),
            ))
            .expect("failed to deserialize"),
        )
        .expect("verification should succeed");

    // Multiple audiences, where one is a match (default = reject)
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsiYXVkMSIsIm15X2NsaWVudCIsImF1ZDIiXSwiaXNzIjoia\
                 HR0cHM6Ly9leGFtcGxlLmNvbSIsInBheWxvYWQiOiJoZWxsbyB3b3JsZCJ9.N9ibisEe0kKLe1GDWM\
                 ON3PmYqbL73dag-loM8pjKJNinF9SB7n4JuSu4FrNkeW4F1Cz8MIbLuWfKvDa_4v_3FstMA3GODZWH\
                 BVIiuNFay2ovCfGFyykwe47dF_47g_OM5AkJc_teE5MN8lPh9V5zYCy3ON3zZ3acFPJMOPTdbU56xD\
                 eFe7lil6DmV4JU9A52t5ZkJILFaIuxxXJUIDmqpPTvHkggh_QOj9C2US9bgg5b543JwT4j-HbDp51L\
                 dDB4k3azOssT1ddtoAuuDOctnraMKUtqffJXexxfwA1uM6EIofSrK5v11xwgTciL9xDXAvav_G2buP\
                 ol1bjGLa2t0Q"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::InvalidAudience(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Multiple audiences, where one is a match (allowed)
    verifier
        .clone()
        .set_other_audience_verifier_fn(|aud| **aud == "aud1" || **aud == "aud2")
        .verified_claims(
            serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
                "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsiYXVkMSIsIm15X2NsaWVudCIsImF1ZDIiXSwiaXNzIjoia\
                 HR0cHM6Ly9leGFtcGxlLmNvbSIsInBheWxvYWQiOiJoZWxsbyB3b3JsZCJ9.N9ibisEe0kKLe1GDWM\
                 ON3PmYqbL73dag-loM8pjKJNinF9SB7n4JuSu4FrNkeW4F1Cz8MIbLuWfKvDa_4v_3FstMA3GODZWH\
                 BVIiuNFay2ovCfGFyykwe47dF_47g_OM5AkJc_teE5MN8lPh9V5zYCy3ON3zZ3acFPJMOPTdbU56xD\
                 eFe7lil6DmV4JU9A52t5ZkJILFaIuxxXJUIDmqpPTvHkggh_QOj9C2US9bgg5b543JwT4j-HbDp51L\
                 dDB4k3azOssT1ddtoAuuDOctnraMKUtqffJXexxfwA1uM6EIofSrK5v11xwgTciL9xDXAvav_G2buP\
                 ol1bjGLa2t0Q"
                    .to_string(),
            ))
            .expect("failed to deserialize"),
        )
        .expect("verification should succeed");

    // Multiple audiences, where none is a match
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsiYXVkMSIsImF1ZDIiXSwiaXNzIjoiaHR0cHM6Ly9leGFtcGxlL\
                 mNvbSIsInBheWxvYWQiOiJoZWxsbyB3b3JsZCJ9.YmFkX2hhc2g"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::InvalidAudience(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Disable signature check.
    verifier
        .clone()
        .require_signature_check(false)
        .verified_claims(
            serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
                "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                     S5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.YmFkX2hhc2g"
                    .to_string(),
            ))
            .expect("failed to deserialize"),
        )
        .expect("verification should succeed");

    // "none" algorithm (unsigned JWT).
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJub25lIn0.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                 S5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ."
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::NoSignature,
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    let valid_rs256_jwt =
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                 S5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.UZ7vmAsDmOBzeB6e2_0POUfyhMRZKM6WSKz3\
                 jB2QdmO-eZ9605EzhkJufJQ8515ryWnHv-gUHtZHQi3zilrzhBwvE2cVP83Gv2XIL1EKaMMmfISeEB\
                 ShWez_FvqxN_bamh5yTROhWmoZTmof-MweBCHgINcsEd7K4e_BHHgq3aaRBpvSFlL_z4l_1NwNcTBo\
                 kqjNScKZITk42AbsSuGR39L94BWLhz6WXQZ_Sn6R1Ro6roOm1b7E82jJiQEtlseQiCCvPR2JJ6LgW6\
                 XTMzQ0vCqSh1A7U_IBDsjY_yag8_X3xxFh2URCtHJ47ZSjqfv6hq7OAq8tmVecOVgfIvABOg"
                .to_string(),
        ))
        .expect("failed to deserialize");
    // Default algs + RS256 -> allowed
    verifier
        .verified_claims(valid_rs256_jwt.clone())
        .expect("verification should succeed");

    let verifier_with_client_secret = CoreJwtClaimsVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
    )
    .set_client_secret(ClientSecret::new("my_secret".to_string()));
    let valid_hs256_jwt =
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                 S5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.dTXvSWen74_rC4oiWw0ziLZNe4KZk8Jw2VZe\
                 N6vLCDo"
                .to_string(),
        ))
        .expect("failed to deserialize");

    // Default algs + HS256 -> disallowed
    match verifier_with_client_secret.verified_claims(valid_hs256_jwt.clone()) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::DisallowedAlg(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // none algs + RS256 -> allowed
    verifier
        .clone()
        .allow_any_alg()
        .verified_claims(valid_rs256_jwt.clone())
        .expect("verification should succeed");

    // none algs + HS256 -> allowed
    verifier_with_client_secret
        .clone()
        .allow_any_alg()
        .verified_claims(valid_hs256_jwt.clone())
        .expect("verification should succeed");

    // none algs + none -> disallowed
    match verifier.clone().allow_any_alg().verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJub25lIn0.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                 S5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ."
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::NoSignature,
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // HS256 + no client secret -> disallowed
    match verifier
        .clone()
        .allow_any_alg()
        .verified_claims(valid_hs256_jwt.clone())
    {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::DisallowedAlg(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // HS256 + valid signature
    verifier_with_client_secret
        .clone()
        .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256])
        .verified_claims(valid_hs256_jwt)
        .expect("verification should succeed");

    // HS256 + invalid signature
    match verifier_with_client_secret
        .clone()
        .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256])
        .verified_claims(
            serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
                "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                     S5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.dTXvSWen74_rC4oiWw0ziLZNe4KZk8Jw2VZe\
                     N6vLCEo"
                    .to_string(),
            ))
            .expect("failed to deserialize"),
        ) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::CryptoError(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // No public keys
    match CoreJwtClaimsVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
    )
    .verified_claims(valid_rs256_jwt.clone())
    {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::NoMatchingKey,
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    let kid = JsonWebKeyId::new("bilbo.baggins@hobbiton.example".to_string());
    let n = Base64UrlEncodedBytes::new(vec![
        159, 129, 15, 180, 3, 130, 115, 208, 37, 145, 228, 7, 63, 49, 210, 182, 0, 27, 130, 206,
        219, 77, 146, 240, 80, 22, 93, 71, 207, 202, 184, 163, 196, 28, 183, 120, 172, 117, 83,
        121, 63, 142, 249, 117, 118, 141, 26, 35, 116, 216, 113, 37, 100, 195, 188, 215, 123, 158,
        164, 52, 84, 72, 153, 64, 124, 255, 0, 153, 146, 10, 147, 26, 36, 196, 65, 72, 82, 171, 41,
        189, 176, 169, 92, 6, 83, 243, 108, 96, 230, 11, 249, 11, 98, 88, 221, 165, 111, 55, 4,
        123, 165, 194, 209, 208, 41, 175, 156, 157, 64, 186, 199, 170, 65, 199, 138, 13, 209, 6,
        138, 221, 105, 158, 128, 143, 234, 1, 30, 161, 68, 29, 138, 79, 123, 180, 233, 123, 227,
        159, 85, 241, 221, 212, 78, 156, 75, 163, 53, 21, 151, 3, 212, 211, 75, 96, 62, 101, 20,
        122, 79, 35, 214, 211, 192, 153, 108, 117, 237, 238, 132, 106, 130, 209, 144, 174, 16, 120,
        60, 150, 28, 240, 56, 122, 237, 33, 6, 210, 208, 85, 91, 111, 217, 55, 250, 213, 83, 83,
        135, 224, 255, 114, 255, 190, 120, 148, 20, 2, 176, 184, 34, 234, 42, 116, 182, 5, 140, 29,
        171, 249, 179, 74, 118, 203, 99, 184, 127, 170, 44, 104, 71, 184, 226, 131, 127, 255, 145,
        24, 110, 107, 28, 20, 145, 28, 249, 137, 168, 144, 146, 168, 28, 230, 1, 221, 172, 211,
        249, 207,
    ]);
    let e = Base64UrlEncodedBytes::new(vec![1, 0, 1]);

    // Wrong key type (symmetric key)
    match CoreJwtClaimsVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![CoreJsonWebKey {
            kty: CoreJsonWebKeyType::Symmetric,
            use_: Some(CoreJsonWebKeyUse::Signature),
            kid: Some(kid.clone()),
            n: None,
            e: None,
            k: Some(Base64UrlEncodedBytes::new(vec![1, 2, 3, 4])),
            crv: None,
            x: None,
            y: None,
            d: None,
            alg: None,
        }]),
    )
    .verified_claims(valid_rs256_jwt.clone())
    {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::NoMatchingKey,
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Correct public key, but with signing disallowed
    match CoreJwtClaimsVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![CoreJsonWebKey {
            kty: CoreJsonWebKeyType::RSA,
            use_: Some(CoreJsonWebKeyUse::Encryption),
            kid: Some(kid),
            n: Some(n),
            e: Some(e),
            k: None,
            crv: None,
            x: None,
            y: None,
            d: None,
            alg: None,
        }]),
    )
    .verified_claims(valid_rs256_jwt.clone())
    {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::NoMatchingKey,
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Wrong key ID
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiIsImtpZCI6Indyb25nX2tleSJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6I\
                 mh0dHBzOi8vZXhhbXBsZS5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.lVLomyIyO8WmyS1VZWPu\
                 cGhRTUyK9RCw90fJC5CfDWUCgt1CBn-aP_ieWWBGfjb4ccR4dl57OYxdLl0Day8QN5pTCBud9QKpQ0rKQX\
                 K8eBlOW8uSosx8q5pwU_bRyy-XuKJiPlDCOwTEHOp_hOgZFGjoN27MH3Xm8kc0iT3PgyqQ46-wsqHY9S02\
                 hdJORX7vqYwQLZF8_k_L8K0IG_dC-1Co0g5oAf37oVSdl8hE-ScQ9K-AiSpS-cGYyldbMhyKNDL3ry2cuI\
                 EUgYSIznkVFuM7RrEdNK222z5PF11ijYx-TM7BIDggbcIyJm-UqpmvVaJImmj5FNkMzuHYznLtdg"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::NoMatchingKey,
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Client secret + public key
    verifier
        .clone()
        .set_client_secret(ClientSecret::new("my_secret".to_string()))
        .verified_claims(valid_rs256_jwt.clone())
        .expect("verification should succeed");

    // Multiple matching public keys: no KID specified
    match CoreJwtClaimsVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![rsa_key.clone(), rsa_key.clone()]),
    )
    .verified_claims(valid_rs256_jwt.clone())
    {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::AmbiguousKeyId(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Multiple matching public keys: KID specified
    match CoreJwtClaimsVerifier::new(
        client_id,
        issuer,
        CoreJsonWebKeySet::new(vec![rsa_key.clone(), rsa_key]),
    )
    .verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiIsImtpZCI6ImJpbGJvLmJhZ2dpbnNAaG9iYml0b24uZXhhbXBsZSJ9.eyJhdWQiO\
                 lsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZS5jb20iLCJwYXlsb2FkIjoiaGVsbG8gd29\
                 ybGQifQ.jH0v2fQGvH2MD0jn5pQP6W6AF5rJlizyofdyRUIt7E3GraGA1LYDiLAVIfhST3uwJopP-TgtBk\
                 zc-zyJSvgTR63S8iI1YlHypItpx7r4I9ydzo8GSN5RrZudcU2esY4uEnLbVl17ZVNu4IyTExeKJ0sPM0Hj\
                 qkOA4XaP2cJwsK-bookNHSA8NRE6adRMrHAKJbor5jrGjpkZAKHbnQFK-wu-nEV_OjS9jpN_FboRZVcDTZ\
                 GFzeFbqFqHdRn6UWPFnVpVnUhih16UjNH1om6gwc0uFoPWTDxJlXQCFbHMhZtgCbUkXQBH7twPMc4YUziw\
                 S8GIRKCcXjdrP5oyxmcitQ"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::AmbiguousKeyId(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // RS256 + valid signature
    verifier
        .verified_claims(valid_rs256_jwt)
        .expect("verification should succeed");

    // RS256 + invalid signature
    match verifier.verified_claims(
        serde_json::from_value::<TestClaimsJsonWebToken>(serde_json::Value::String(
            "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZS5jb\
                 20iLCJwYXlsb2FkIjoiaGVsbG8gd29ybGQifQ.YmFkX2hhc2g"
                .to_string(),
        ))
        .expect("failed to deserialize"),
    ) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::CryptoError(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }
}

type CoreIdTokenJwt = JsonWebToken<
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreIdTokenClaims,
    JsonWebTokenJsonPayloadSerde,
>;

#[test]
fn test_id_token_verified_claims() {
    let rsa_key =
        serde_json::from_str::<CoreJsonWebKey>(TEST_RSA_PUB_KEY).expect("deserialization failed");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let mock_current_time = AtomicUsize::new(1544932149);
    let mock_is_valid_issue_time = AtomicBool::new(true);
    // Extra scope needed to ensure closures are destroyed before the values they borrow.
    {
        let public_client_verifier = CoreIdTokenVerifier::new_public_client(
            client_id.clone(),
            issuer.clone(),
            CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
        )
        .set_time_fn(|| {
            Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
                .to_utc()
                .unwrap()
        })
        .set_issue_time_verifier_fn(|_| {
            if mock_is_valid_issue_time.load(Ordering::Relaxed) {
                Ok(())
            } else {
                Err("Invalid iat claim".to_string())
            }
        });

        let insecure_verifier = CoreIdTokenVerifier::new_insecure_without_verification()
            .set_time_fn(|| {
                Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
                    .to_utc()
                    .unwrap()
            });

        // This JWTs below have an issue time of 1544928549 and an expiration time of 1544932149.

        let test_jwt_without_nonce =
            serde_json::from_value::<CoreIdTokenJwt>(serde_json::Value::String(
                "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                     S5jb20iLCJzdWIiOiJzdWJqZWN0IiwiZXhwIjoxNTQ0OTMyMTQ5LCJpYXQiOjE1NDQ5Mjg1NDl9.nN\
                     aTxNwclnTHd1Q9POkddm5wB1w3wJ-gwQWHomhimttk3SWQTLhxI0SSjWrHahGxlfkjufJlSyt-t_VO\
                     SdcROvIYZTDznDfFZz3oSOev-p9XiZ-EZTS-U6N11Y923sDQjbTMeukz1F3ZFEfn5Mv2xjdEoJccCe\
                     7SaGuDmVqMqTLXMtsw9NCE_KDd0oKSwDzbJIBBPEfG3JjbKg0Dln7ENHg9wzoNFQzPXrkKzjneBgD3\
                     vuwFCV5y-e8xUBdLaLZF1kdkDZJIA48uRROLlWjsM8pEptosA5QK07luQCZNqcaZWEczoGXeQs8PyA\
                     zkNV7JEmti3bJnWSN-ud4cFU0LiQ"
                    .to_string(),
            ))
            .expect("failed to deserialize");

        // Invalid JWT claims
        match public_client_verifier.verified_claims(
                &serde_json::from_value::<CoreIdTokenJwt>(serde_json::Value::String(
                    "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vYXR0YWNrZ\
                     XIuY29tIiwic3ViIjoic3ViamVjdCIsImV4cCI6MTU0NDkzMjE0OSwiaWF0IjoxNTQ0OTI4NTQ5LCJ\
                     ub25jZSI6InRoZV9ub25jZSIsImFjciI6InRoZV9hY3IifQ.Pkicxk0dTU5BkSxgqTON6lE7A7ir3l\
                     aADRyoeRoCNDX3AOx7BXCbfzbda6HJiPskN2nu56w0q-0OdkDSIHls-2xTUlLEJv2Bv0BLYwV5ZVJ8\
                     hoc-rTd0_oLUb5NzyD80RyVByjVMK8bh6cwysTnr8QDxsEiFZbFo3mVJob2yjPZnNOdcNJWPcVVueP\
                     8vqMJnx5kHih1gKZpWj_dMN9b2AW6zVLOInW3Ox__gx6fsFFz7rjxItG-PTY_OQMzthqeHUyq4o9y7\
                     Jv8mB_jFkTZGVKHTPpObHV-qptJ_rnlwvF_mP5GARBLng-4Yd7nmSr31onYL48QDjGOrwPqQ-IyaCQ"
                        .to_string(),
                ))
                    .expect("failed to deserialize"), |_: Option<&Nonce>| Ok(())) {
                Err(ClaimsVerificationError::InvalidIssuer(_)) => {}
                other => panic!("unexpected result: {:?}", other),
            }

        // TODO: disallowed algs

        // Expired token
        mock_current_time.store(1544928549 + 3600, Ordering::Relaxed);
        match public_client_verifier
            .verified_claims(&test_jwt_without_nonce, |_: Option<&Nonce>| Ok(()))
        {
            Err(ClaimsVerificationError::Expired(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }
        mock_current_time.store(1544928549 + 1, Ordering::Relaxed);

        // Invalid issue time
        mock_is_valid_issue_time.store(false, Ordering::Relaxed);
        match public_client_verifier
            .verified_claims(&test_jwt_without_nonce, |_: Option<&Nonce>| Ok(()))
        {
            Err(ClaimsVerificationError::Expired(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }
        mock_is_valid_issue_time.store(true, Ordering::Relaxed);

        let valid_nonce = Nonce::new("the_nonce".to_string());

        // Successful verification w/o checking nonce
        public_client_verifier
            .verified_claims(&test_jwt_without_nonce, |_: Option<&Nonce>| Ok(()))
            .expect("verification should succeed");

        // Missing nonce
        match public_client_verifier.verified_claims(&test_jwt_without_nonce, &valid_nonce) {
            Err(ClaimsVerificationError::InvalidNonce(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        // Missing nonce w/ closure
        match public_client_verifier.verified_claims(
            &test_jwt_without_nonce,
            |nonce: Option<&Nonce>| {
                if nonce.iter().any(|n| n.secret() == valid_nonce.secret()) {
                    Ok(())
                } else {
                    Err("invalid nonce".to_string())
                }
            },
        ) {
            Err(ClaimsVerificationError::InvalidNonce(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        let test_jwt_with_nonce =
            serde_json::from_value::<CoreIdTokenJwt>(serde_json::Value::String(
                "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                     S5jb20iLCJzdWIiOiJzdWJqZWN0IiwiZXhwIjoxNTQ0OTMyMTQ5LCJpYXQiOjE1NDQ5Mjg1NDksIm5\
                     vbmNlIjoidGhlX25vbmNlIiwiYWNyIjoidGhlX2FjciIsImF1dGhfdGltZSI6MTU0NDkyODU0OH0.W\
                     XA7SS9aMh_6rvBEgQce5D2J84OqphmmnCLGgEKRTN5G-UuQTNOBp8VS5_4f3xgzMEEMvGJJauJoALk\
                     muUeHB-N_ESrkmB3tgDzBSYBa7kuYPHUPYpdjZM2UVolqI9RYyHaWwKjL_Io5YyAazB5lH5ibPaiBl\
                     UNKGs3cmVsEB22UGMFKM6cek7GinrHQe_aJQsMU839-c2zzlEyFSeI8QBphQtG6AN82IPkNRv8QWmw\
                     ZjUiB5a-W73Z3gURYMNs7f32BjAUNoJzW0Qj34vzD2djoSHhltE0wHKBzPqGhUM1Y3A-a3q-LS2g1h\
                     6qgXb_KQ_Mmok8v8ld0cW_aYRLfNg"
                    .to_string(),
            ))
            .expect("failed to deserialize");

        // Invalid nonce
        match public_client_verifier.verified_claims(
            &test_jwt_with_nonce,
            &Nonce::new("different_nonce".to_string()),
        ) {
            Err(ClaimsVerificationError::InvalidNonce(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        let verified_claims = public_client_verifier
            .clone()
            .set_auth_context_verifier_fn(|acr| {
                assert_eq!(**acr.unwrap(), "the_acr");
                Err("Invalid acr claim".to_string())
            })
            .verified_claims(&test_jwt_with_nonce, &valid_nonce);

        // Invalid AuthenticationContextClass reference
        match verified_claims {
            Err(ClaimsVerificationError::InvalidAuthContext(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        let test_jwt_without_auth_time =
            serde_json::from_value::<CoreIdTokenJwt>(serde_json::Value::String(
                "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                     S5jb20iLCJzdWIiOiJzdWJqZWN0IiwiZXhwIjoxNTQ0OTMyMTQ5LCJpYXQiOjE1NDQ5Mjg1NDksIm5\
                     vbmNlIjoidGhlX25vbmNlIiwiYWNyIjoidGhlX2FjciJ9.c_lU1VRasTg0mB4lwdOzbzvFS_XShMLN\
                     lAPUpHBaMtCSPtI71L2x3hIByfkqIrAED-Qc_am2gNJ20bifidlkTOO6nyaBrJuaSjwT8aqajEbXon\
                     5JFswwPvqCIWjd0eV5dXC1MZunpd7ANXSC7Qw16v3m_crc9wcI_fLFCzuAKrWYokGvNy0gr1CxcgVg\
                     aE9qR0eqaatetzCuaOJhYOq4njrRlGZWtbj5Q56q3zhxJ_yS8K8gv1QcB4sHjUyXIj21jzjUD87zVG\
                     dJsn8E-nFJSltBdQhEaLksTBH6ZZhkeGicQ8cEPnNeS4L1vfVyAd_cjl64JHLmzw8RUp8XuoF9nA"
                    .to_string(),
            ))
            .expect("failed to deserialize");

        // Missing auth_time (ok)
        public_client_verifier
            .verified_claims(&test_jwt_without_auth_time, |_: Option<&Nonce>| Ok(()))
            .expect("verification should succeed");

        let verified_claims = public_client_verifier
            .clone()
            .set_auth_time_verifier_fn(|auth_time| {
                assert!(auth_time.is_none());
                Err("Invalid auth_time claim".to_string())
            })
            .verified_claims(&test_jwt_without_auth_time, |_: Option<&Nonce>| Ok(()));

        // Missing auth_time (error)
        match verified_claims {
            Err(ClaimsVerificationError::InvalidAuthTime(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        let verified_claims = public_client_verifier
            .clone()
            .set_auth_time_verifier_fn(|auth_time| {
                assert_eq!(
                    auth_time.unwrap(),
                    Timestamp::Seconds(1544928548.into()).to_utc().unwrap(),
                );
                Err("Invalid auth_time claim".to_string())
            })
            .verified_claims(&test_jwt_with_nonce, &valid_nonce);

        // Invalid auth_time
        match verified_claims {
            Err(ClaimsVerificationError::InvalidAuthTime(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        // Successful verification with nonce, acr, and auth_time specified (no expected Nonce)
        public_client_verifier
            .verified_claims(&test_jwt_with_nonce, |_: Option<&Nonce>| Ok(()))
            .expect("verification should succeed");
        insecure_verifier
            .verified_claims(&test_jwt_with_nonce, |_: Option<&Nonce>| Ok(()))
            .expect("verification should succeed");

        // Successful verification with nonce, acr, and auth_time specified (w/ expected Nonce)
        public_client_verifier
            .verified_claims(&test_jwt_with_nonce, &valid_nonce)
            .expect("verification should succeed");
        insecure_verifier
            .verified_claims(&test_jwt_with_nonce, &valid_nonce)
            .expect("verification should succeed");

        // Successful verification with nonce, acr, and auth_time specified (w/ closure)
        public_client_verifier
            .verified_claims(&test_jwt_with_nonce, |nonce: Option<&Nonce>| {
                if nonce.iter().any(|n| n.secret() == valid_nonce.secret()) {
                    Ok(())
                } else {
                    Err("invalid nonce".to_string())
                }
            })
            .expect("verification should succeed");
        insecure_verifier
            .verified_claims(&test_jwt_with_nonce, |nonce: Option<&Nonce>| {
                if nonce.iter().any(|n| n.secret() == valid_nonce.secret()) {
                    Ok(())
                } else {
                    Err("invalid nonce".to_string())
                }
            })
            .expect("verification should succeed");

        // HS256 w/ default algs
        let test_jwt_hs256 = serde_json::from_value::<CoreIdTokenJwt>(serde_json::Value::String(
            "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhbXBsZ\
                     S5jb20iLCJzdWIiOiJzdWJqZWN0IiwiZXhwIjoxNTQ0OTMyMTQ5LCJpYXQiOjE1NDQ5Mjg1NDksIm5\
                     vbmNlIjoidGhlX25vbmNlIn0.xUnSwSbcHsHWyJxwKGg69BIo_CktcyN5BVulGDb_QzE"
                .to_string(),
        ))
        .expect("failed to deserialize");
        let private_client_verifier = CoreIdTokenVerifier::new_confidential_client(
            client_id.clone(),
            ClientSecret::new("my_secret".to_string()),
            issuer.clone(),
            CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
        )
        .set_time_fn(|| {
            Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
                .to_utc()
                .unwrap()
        });
        match private_client_verifier.verified_claims(&test_jwt_hs256, &valid_nonce) {
            Err(ClaimsVerificationError::SignatureVerification(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }
        insecure_verifier
            .clone()
            .verified_claims(&test_jwt_hs256, &valid_nonce)
            .expect("verification should succeed");

        // HS256 w/ set_allowed_algs
        private_client_verifier
            .clone()
            .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256])
            .verified_claims(&test_jwt_hs256, &valid_nonce)
            .expect("verification should succeed");

        // HS256 w/ allow_any_alg
        private_client_verifier
            .clone()
            .allow_any_alg()
            .verified_claims(&test_jwt_hs256, &valid_nonce)
            .expect("verification should succeed");

        // Invalid signature
        let private_client_verifier_with_other_secret =
            CoreIdTokenVerifier::new_confidential_client(
                client_id,
                ClientSecret::new("other_secret".to_string()),
                issuer,
                CoreJsonWebKeySet::new(vec![rsa_key]),
            )
            .allow_any_alg()
            .set_time_fn(|| {
                Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
                    .to_utc()
                    .unwrap()
            });
        match private_client_verifier_with_other_secret
            .verified_claims(&test_jwt_hs256, &valid_nonce)
        {
            Err(ClaimsVerificationError::SignatureVerification(_)) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        // Invalid signature w/ signature check disabled
        private_client_verifier_with_other_secret
            .clone()
            .insecure_disable_signature_check()
            .verified_claims(&test_jwt_hs256, &valid_nonce)
            .expect("verification should succeed");
    };
}

#[test]
fn test_new_id_token() {
    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let nonce = Nonce::new("the_nonce".to_string());
    let rsa_priv_key = CoreRsaPrivateSigningKey::from_pem(TEST_RSA_PRIV_KEY, None).unwrap();

    let id_token = CoreIdToken::new(
        CoreIdTokenClaims::new(
            issuer.clone(),
            vec![Audience::new((*client_id).clone())],
            Utc.timestamp_opt(1544932149, 0)
                .single()
                .expect("valid timestamp"),
            Utc.timestamp_opt(1544928549, 0)
                .single()
                .expect("valid timestamp"),
            StandardClaims::new(SubjectIdentifier::new("subject".to_string())),
            Default::default(),
        )
        .set_nonce(Some(nonce.clone()))
        .set_auth_context_ref(Some(AuthenticationContextClass::new("the_acr".to_string())))
        .set_auth_time(Some(
            Utc.timestamp_opt(1544928548, 0)
                .single()
                .expect("valid timestamp"),
        )),
        &rsa_priv_key,
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
        Some(&AccessToken::new("the_access_token".to_string())),
        Some(&AuthorizationCode::new(
            "the_authorization_code".to_string(),
        )),
    )
    .unwrap();

    let serialized_jwt: serde_json::Value = serde_json::to_value(&id_token).unwrap();
    let expected_serialized_jwt =
        "eyJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJodHRwczovL2V4YW1wbGUuY29tIiwiYXVkIjpbIm15X2NsaWVudCJdL\
             CJleHAiOjE1NDQ5MzIxNDksImlhdCI6MTU0NDkyODU0OSwiYXV0aF90aW1lIjoxNTQ0OTI4NTQ4LCJub25jZSI\
             6InRoZV9ub25jZSIsImFjciI6InRoZV9hY3IiLCJhdF9oYXNoIjoiWjNJQUNVR00tbXhIV3lZUXZpSzhFUSIsI\
             mNfaGFzaCI6Imo2OW1CZmFIbmRMM1Y1RmNoak9LVXciLCJzdWIiOiJzdWJqZWN0In0.CHCWFcIqbCZhZwZH4oY\
             _mlcRy5aUQQtlNI0VHNYxiILn9ppRHLL4Bn_LMn9VP8tGXkfZWxCgP25ZTyBXXKfk0fQvnukVdyM0bCOpQbiBg\
             5gB9c46l_f-ZznDoHWonpnKky2Gmzk3ocb3TCUQ9GSeRXAzRdRNWTT0ElWNBsLWU4j2IIdnghM78gkXwOC76Rk\
             pshgB73ubtuHGdIf5L9Ec3hifHlVjzKuvedAM4SIOjdBOelgtBlF3463ufX_Ut91CjP5TzLMsuK3Lh_vyo8ttn\
             S41rBDuetR2ENvR0yj5RjkX_SPY3V0yCW8_NPPu1CHu_1oL0Nma0ohCbF3vnUJcwg";
    assert_eq!(expected_serialized_jwt, serialized_jwt.as_str().unwrap());

    let rsa_pub_key =
        serde_json::from_str::<CoreJsonWebKey>(TEST_RSA_PUB_KEY).expect("deserialization failed");

    let mock_current_time = AtomicUsize::new(1544932148);
    let time_fn = || {
        Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
            .to_utc()
            .unwrap()
    };
    let verifier = CoreIdTokenVerifier::new_public_client(
        client_id,
        issuer,
        CoreJsonWebKeySet::new(vec![rsa_pub_key]),
    )
    .set_time_fn(time_fn);
    let claims = id_token.claims(&verifier, &nonce).unwrap();
    let unverified = id_token
        .claims(
            &CoreIdTokenVerifier::new_insecure_without_verification().set_time_fn(time_fn),
            &nonce,
        )
        .unwrap();
    assert_eq!(claims, unverified);
}

#[test]
fn test_user_info_verified_claims() {
    let rsa_key =
        serde_json::from_str::<CoreJsonWebKey>(TEST_RSA_PUB_KEY).expect("deserialization failed");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let sub = SubjectIdentifier::new("the_subject".to_string());

    let verifier = CoreUserInfoVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
        Some(sub.clone()),
    );

    let json_claims = "{\
                           \"sub\": \"the_subject\",\
                           \"name\": \"Jane Doe\"\
                           }";

    // JSON response (default args)
    assert_eq!(
        CoreUserInfoClaims::from_json::<crate::reqwest::Error>(json_claims.as_bytes(), Some(&sub))
            .expect("verification should succeed")
            .name()
            .unwrap()
            .iter()
            .collect::<Vec<_>>(),
        vec![(None, &EndUserName::new("Jane Doe".to_string()))],
    );

    // Invalid subject
    match CoreUserInfoClaims::from_json::<crate::reqwest::Error>(
        json_claims.as_bytes(),
        Some(&SubjectIdentifier::new("wrong_subject".to_string())),
    ) {
        Err(UserInfoError::ClaimsVerification(ClaimsVerificationError::InvalidSubject(_))) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    let jwt_claims = serde_json::from_value::<CoreUserInfoJsonWebToken>(serde_json::Value::String(
        "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhb\
                 XBsZS5jb20iLCJzdWIiOiJ0aGVfc3ViamVjdCIsIm5hbWUiOiJKYW5lIERvZSJ9.aX7VpexLAd\
                 43HtC1cFTot3jmqsr105rB50mzTcS1TXzWcxLbqYf1K7Kf-S1oP-ZCL_dnL9-nu3iDK_vRa6xT\
                 nGGt3I1JwhoIv6znSS3JOPT1wtekyD-sLcUwqsJHWBBiTSBwlmGG_kVRuGkBtXgVZ9aGlqg9u1\
                 FlxvyGUJ5q1o9gdb8mKql5ojgsThTNo9qdW3lPIVsiDO-n4mMp4HuOp1re4ZDDkHxiExjtLQAV\
                 kR4q3SlhJC2mkr4mw3_0a2AW52ocWDiwY_lPcdmohmwFaB8aHlivYLFnmKGQIatEW-KDaW5fFo\
                 JYreNkplo4FvzXYyxgxAsqHjHMI8MZVEa1IA"
            .to_string(),
    ))
    .expect("failed to deserialize");

    // Valid JWT response (default args)
    jwt_claims
        .clone()
        .claims(&verifier)
        .expect("verification should succeed");

    // JWT response with invalid signature
    match serde_json::from_value::<CoreUserInfoJsonWebToken>(serde_json::Value::String(
        "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhb\
             XBsZS5jb20iLCJzdWIiOiJ0aGVfc3ViamVjdCIsIm5hbWUiOiJKYW5lIERvZSJ9.bX7VpexLAd\
             43HtC1cFTot3jmqsr105rB50mzTcS1TXzWcxLbqYf1K7Kf-S1oP-ZCL_dnL9-nu3iDK_vRa6xT\
             nGGt3I1JwhoIv6znSS3JOPT1wtekyD-sLcUwqsJHWBBiTSBwlmGG_kVRuGkBtXgVZ9aGlqg9u1\
             FlxvyGUJ5q1o9gdb8mKql5ojgsThTNo9qdW3lPIVsiDO-n4mMp4HuOp1re4ZDDkHxiExjtLQAV\
             kR4q3SlhJC2mkr4mw3_0a2AW52ocWDiwY_lPcdmohmwFaB8aHlivYLFnmKGQIatEW-KDaW5fFo\
             JYreNkplo4FvzXYyxgxAsqHjHMI8MZVEa1IA"
            .to_string(),
    ))
    .expect("failed to deserialize")
    .claims(&verifier)
    {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::CryptoError(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // JWT response with invalid issuer claim (error)
    match jwt_claims.clone().claims(&CoreUserInfoVerifier::new(
        client_id.clone(),
        IssuerUrl::new("https://attacker.com".to_string()).unwrap(),
        CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
        Some(sub.clone()),
    )) {
        Err(ClaimsVerificationError::InvalidIssuer(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // JWT response with invalid issuer claim (allowed)
    jwt_claims
        .clone()
        .claims(
            &CoreUserInfoVerifier::new(
                client_id,
                IssuerUrl::new("https://attacker.com".to_string()).unwrap(),
                CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
                Some(sub.clone()),
            )
            .require_issuer_match(false),
        )
        .expect("verification should succeed");

    // JWT response with invalid audience claim (error)
    match jwt_claims.clone().claims(&CoreUserInfoVerifier::new(
        ClientId::new("wrong_client".to_string()),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
        Some(sub.clone()),
    )) {
        Err(ClaimsVerificationError::InvalidAudience(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // JWT response with invalid audience claim (allowed)
    jwt_claims
        .claims(
            &CoreUserInfoVerifier::new(
                ClientId::new("wrong_client".to_string()),
                issuer,
                CoreJsonWebKeySet::new(vec![rsa_key]),
                Some(sub),
            )
            .require_audience_match(false),
        )
        .expect("verification should succeed");
}

#[test]
fn test_new_user_info_claims() {
    let claims = CoreUserInfoClaims::new(
        StandardClaims {
            sub: SubjectIdentifier::new("the_subject".to_string()),
            name: Some(EndUserName::new("John Doe".to_string()).into()),
            given_name: None,
            family_name: None,
            middle_name: None,
            nickname: None,
            preferred_username: None,
            profile: None,
            picture: None,
            website: None,
            email: None,
            email_verified: None,
            gender: None,
            birthday: None,
            birthdate: None,
            zoneinfo: None,
            locale: None,
            phone_number: None,
            phone_number_verified: None,
            address: None,
            updated_at: Some(
                Utc.timestamp_opt(1544928548, 0)
                    .single()
                    .expect("valid timestamp"),
            ),
        },
        Default::default(),
    );

    assert_eq!(
        "{\"sub\":\"the_subject\",\"name\":\"John Doe\",\"updated_at\":1544928548}",
        serde_json::to_string(&claims).unwrap()
    );

    let rsa_priv_key = CoreRsaPrivateSigningKey::from_pem(TEST_RSA_PRIV_KEY, None).unwrap();
    let claims_jwt = CoreUserInfoJsonWebToken::new(
        claims,
        &rsa_priv_key,
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
    )
    .unwrap();
    assert_eq!(
        "eyJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJ0aGVfc3ViamVjdCIsIm5hbWUiOiJKb2huIERvZSIsInVwZGF0ZWRfY\
             XQiOjE1NDQ5Mjg1NDh9.nJ7Buckt_p_ACXkyVRCQLqyaW8KhDsk5H9Nu7PdNf4daEcEWm-lGjoSTAfAbDPgHAZ\
             78knomgLgDxiGWrj1qdFTIEFep32I3q18VBP_DcMdyuQafipK6T98RgZFWP8YnxlxLPHeJQlRsdMpemHK4vxas\
             ZD4A4aIn0K7z5J9RvrR3L7DWnc3fJQ0VU2v5QLePyqNWnFxks5eyl8Ios8JrZhwr4Q8GES8Q4Iw8Sz6W9vYpHK\
             2r1YdaACMM4g_TTtV91lpjn-Li2-HxW9NERdLvYvF6HwGIwbss26trp2yjNTARlxBUT6LR7y82oPIJKXIKL1GD\
             YeSLeErhb6oTQ0a5gQ",
        serde_json::to_value(claims_jwt).unwrap().as_str().unwrap()
    );
}

#[test]
fn test_es256_id_token_verified_claims() {
    use base64::Engine;
    use p256::ecdsa::signature::Signer;
    use p256::elliptic_curve::sec1::ToEncodedPoint;

    let b64 = crate::core::base64_url_safe_no_pad();

    // Self-generated, deterministic P-256 fixture keys: fixed scalar seeds, so
    // the tokens signed below are reproducible on every run.
    let seed: [u8; 32] = [0x37; 32];
    let signing_key = p256::ecdsa::SigningKey::from_bytes(&seed.into())
        .expect("fixture scalar is a valid P-256 secret key");
    let other_seed: [u8; 32] = [0x5a; 32];
    let other_signing_key = p256::ecdsa::SigningKey::from_bytes(&other_seed.into())
        .expect("fixture scalar is a valid P-256 secret key");

    let public_point = p256::PublicKey::from(signing_key.verifying_key()).to_encoded_point(false);
    let public_bytes = public_point.as_bytes();
    let es256_key: CoreJsonWebKey = serde_json::from_value(serde_json::json!({
        "kty": "EC",
        "kid": "test-es256-key",
        "crv": "P-256",
        "x": b64.encode(&public_bytes[1..33]),
        "y": b64.encode(&public_bytes[33..65]),
        "use": "sig",
    }))
    .expect("deserialization failed");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let mock_current_time = AtomicUsize::new(1544932148);
    let verifier = CoreIdTokenVerifier::new_public_client(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![es256_key.clone()]),
    )
    // The default allowed-algorithm set is RS256-only; a relying party that
    // consumes ES256 providers selects the algorithm through this public
    // builder, mirroring `id_token_signed_response_alg` registration.
    .set_allowed_algs(vec![CoreJwsSigningAlgorithm::EcdsaP256Sha256])
    .set_time_fn(|| {
        Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
            .to_utc()
            .unwrap()
    });

    let nonce = Nonce::new("the_nonce".to_string());

    let sign_es256 = |key: &p256::ecdsa::SigningKey, payload: &str| -> String {
        let header = "{\"alg\":\"ES256\",\"typ\":\"JWT\",\"kid\":\"test-es256-key\"}";
        let signing_input = format!(
            "{}.{}",
            b64.encode(header.as_bytes()),
            b64.encode(payload.as_bytes())
        );
        let signature: p256::ecdsa::Signature = key.sign(signing_input.as_bytes());
        format!("{}.{}", signing_input, b64.encode(signature.to_bytes()))
    };

    // Claims are valid relative to the mocked current time 1544932148.
    let valid_payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"subject\",\"exp\":1544932149,\"iat\":1544928549,\"nonce\":\"the_nonce\"}";

    // Parse through the public consumer entry point (`IdToken: FromStr`) and
    // verify via the public `CoreIdToken::claims`, exactly as a relying party
    // would.
    let id_token_valid: CoreIdToken = sign_es256(&signing_key, valid_payload)
        .parse()
        .expect("failed to deserialize");

    // Valid ES256 ID token: signature, issuer, audience, nonce, and expiry all
    // verify through the public verifier.
    let claims = id_token_valid
        .claims(&verifier, &nonce)
        .expect("verification should succeed");
    assert_eq!(claims.issuer().as_str(), "https://example.com");
    assert!(claims.audiences().iter().any(|aud| **aud == *"my_client"));
    assert_eq!(&**claims.subject(), "subject");

    // Wrong issuer claim.
    let wrong_issuer_payload = "{\"iss\":\"https://attacker.com\",\"aud\":[\"my_client\"],\
\"sub\":\"subject\",\"exp\":1544932149,\"iat\":1544928549,\"nonce\":\"the_nonce\"}";
    let id_token_wrong_issuer: CoreIdToken = sign_es256(&signing_key, wrong_issuer_payload)
        .parse()
        .expect("failed to deserialize");
    match id_token_wrong_issuer.claims(&verifier, &nonce) {
        Err(ClaimsVerificationError::InvalidIssuer(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Wrong audience claim.
    let wrong_audience_payload = "{\"iss\":\"https://example.com\",\"aud\":[\"other_client\"],\
\"sub\":\"subject\",\"exp\":1544932149,\"iat\":1544928549,\"nonce\":\"the_nonce\"}";
    let id_token_wrong_audience: CoreIdToken = sign_es256(&signing_key, wrong_audience_payload)
        .parse()
        .expect("failed to deserialize");
    match id_token_wrong_audience.claims(&verifier, &nonce) {
        Err(ClaimsVerificationError::InvalidAudience(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Wrong nonce expectation.
    match id_token_valid.claims(&verifier, &Nonce::new("different_nonce".to_string())) {
        Err(ClaimsVerificationError::InvalidNonce(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Expired token.
    let expired_payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"subject\",\"exp\":1544928600,\"iat\":1544928549,\"nonce\":\"the_nonce\"}";
    let id_token_expired: CoreIdToken = sign_es256(&signing_key, expired_payload)
        .parse()
        .expect("failed to deserialize");
    match id_token_expired.claims(&verifier, &nonce) {
        Err(ClaimsVerificationError::Expired(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Tampered signature.
    let token_valid = sign_es256(&signing_key, valid_payload);
    let (signing_input, signature) = token_valid.rsplit_once('.').unwrap();
    let mut tampered_signature = signature.to_string();
    let replacement = if tampered_signature.ends_with('A') {
        'B'
    } else {
        'A'
    };
    tampered_signature.replace_range(tampered_signature.len() - 1.., &replacement.to_string());
    let id_token_tampered: CoreIdToken = format!("{}.{}", signing_input, tampered_signature)
        .parse()
        .expect("failed to deserialize");
    match id_token_tampered.claims(&verifier, &nonce) {
        Err(ClaimsVerificationError::SignatureVerification(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Signature by the wrong key (key set contains only the matching key).
    let id_token_wrong_key: CoreIdToken = sign_es256(&other_signing_key, valid_payload)
        .parse()
        .expect("failed to deserialize");
    match id_token_wrong_key.claims(&verifier, &nonce) {
        Err(ClaimsVerificationError::SignatureVerification(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }
}

/// Signed user info responses are accepted only for explicitly allowed algorithms; the default
/// allowlist is RS256-only and there is no way to allow any algorithm.
#[test]
fn test_user_info_signed_response_es256() {
    use base64::Engine;
    use p256::ecdsa::signature::Signer;
    use p256::elliptic_curve::sec1::ToEncodedPoint;

    let b64 = crate::core::base64_url_safe_no_pad();

    // Deterministic P-256 fixture keys (same derivation as test_es256_id_token_verified_claims).
    let seed: [u8; 32] = [0x37; 32];
    let signing_key = p256::ecdsa::SigningKey::from_bytes(&seed.into())
        .expect("fixture scalar is a valid P-256 secret key");
    let other_seed: [u8; 32] = [0x5a; 32];
    let other_signing_key = p256::ecdsa::SigningKey::from_bytes(&other_seed.into())
        .expect("fixture scalar is a valid P-256 secret key");

    let public_point = p256::PublicKey::from(signing_key.verifying_key()).to_encoded_point(false);
    let public_bytes = public_point.as_bytes();
    let es256_key: CoreJsonWebKey = serde_json::from_value(serde_json::json!({
        "kty": "EC",
        "kid": "test-es256-key",
        "crv": "P-256",
        "x": b64.encode(&public_bytes[1..33]),
        "y": b64.encode(&public_bytes[33..65]),
        "use": "sig",
    }))
    .expect("deserialization failed");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let sub = SubjectIdentifier::new("the_subject".to_string());

    let sign_es256 = |key: &p256::ecdsa::SigningKey, payload: &str| -> String {
        let header = "{\"alg\":\"ES256\",\"typ\":\"JWT\",\"kid\":\"test-es256-key\"}";
        let signing_input = format!(
            "{}.{}",
            b64.encode(header.as_bytes()),
            b64.encode(payload.as_bytes())
        );
        let signature: p256::ecdsa::Signature = key.sign(signing_input.as_bytes());
        format!("{}.{}", signing_input, b64.encode(signature.to_bytes()))
    };

    let valid_payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"the_subject\",\"name\":\"Jane Doe\"}";
    let user_info_jwt: CoreUserInfoJsonWebToken = serde_json::from_value(
        serde_json::Value::String(sign_es256(&signing_key, valid_payload)),
    )
    .expect("failed to deserialize");

    // ES256 is accepted only once explicitly allowed.
    let verifier = CoreUserInfoVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![es256_key.clone()]),
        Some(sub.clone()),
    )
    .set_allowed_algs(vec![CoreJwsSigningAlgorithm::EcdsaP256Sha256]);
    let claims = user_info_jwt
        .clone()
        .claims(&verifier)
        .expect("verification should succeed");
    assert_eq!(*claims.subject(), sub);

    // The default verifier rejects ES256 with DisallowedAlg (RS256-only default).
    let default_verifier = CoreUserInfoVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![es256_key.clone()]),
        Some(sub.clone()),
    );
    match user_info_jwt.clone().claims(&default_verifier) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::DisallowedAlg(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Altered signature is rejected.
    let token_valid = sign_es256(&signing_key, valid_payload);
    let (signing_input, signature) = token_valid.rsplit_once('.').unwrap();
    let mut tampered_signature = signature.to_string();
    // Flip the first signature character: it always encodes six real bits (the final
    // base64 character of an unpadded encoding may only carry padding bits).
    let replacement = if tampered_signature.starts_with('A') {
        'B'
    } else {
        'A'
    };
    tampered_signature.replace_range(0..1, &replacement.to_string());
    let tampered: CoreUserInfoJsonWebToken = serde_json::from_value(serde_json::Value::String(
        format!("{}.{}", signing_input, tampered_signature),
    ))
    .expect("failed to deserialize");
    match tampered.claims(&verifier) {
        Err(ClaimsVerificationError::SignatureVerification(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Signature by the wrong key is rejected.
    let wrong_key: CoreUserInfoJsonWebToken = serde_json::from_value(serde_json::Value::String(
        sign_es256(&other_signing_key, valid_payload),
    ))
    .expect("failed to deserialize");
    match wrong_key.claims(&verifier) {
        Err(ClaimsVerificationError::SignatureVerification(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }
}

/// HS256-signed user info responses verify through a confidential verifier with an explicitly
/// allowed algorithm using the client secret; a public verifier must reject them even when the
/// algorithm is allowed.
#[test]
fn test_user_info_signed_response_hs256() {
    use base64::Engine;

    let b64 = crate::core::base64_url_safe_no_pad();

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let sub = SubjectIdentifier::new("the_subject".to_string());
    let client_secret = ClientSecret::new("the_client_secret".to_string());

    let payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"the_subject\",\"name\":\"Jane Doe\"}";
    let sign_hs256 = |secret: &str| -> String {
        let header = "{\"alg\":\"HS256\",\"typ\":\"JWT\"}";
        let signing_input = format!(
            "{}.{}",
            b64.encode(header.as_bytes()),
            b64.encode(payload.as_bytes())
        );
        let hmac_key = CoreHmacKey::new(secret);
        let signature = hmac_key
            .sign(
                &CoreJwsSigningAlgorithm::HmacSha256,
                signing_input.as_bytes(),
            )
            .expect("HS256 signing should succeed");
        format!("{}.{}", signing_input, b64.encode(signature))
    };

    let user_info_jwt: CoreUserInfoJsonWebToken =
        serde_json::from_value(serde_json::Value::String(sign_hs256("the_client_secret")))
            .expect("failed to deserialize");

    // Accepted via the confidential verifier with the matching secret (empty JWKS: the
    // verification key is derived from the client secret). HS256 requires explicit
    // configuration in addition to the confidential verifier.
    let verifier = CoreUserInfoVerifier::new_confidential_client(
        client_id.clone(),
        client_secret.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
        Some(sub.clone()),
    )
    .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256]);
    let claims = user_info_jwt
        .clone()
        .claims(&verifier)
        .expect("verification should succeed");
    assert_eq!(*claims.subject(), sub);

    // Wrong secret is rejected.
    let wrong_secret_verifier = CoreUserInfoVerifier::new_confidential_client(
        client_id.clone(),
        ClientSecret::new("the_wrong_secret".to_string()),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
        Some(sub.clone()),
    )
    .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256]);
    match user_info_jwt.clone().claims(&wrong_secret_verifier) {
        Err(ClaimsVerificationError::SignatureVerification(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // HS256 additionally requires a confidential verifier: a public verifier rejects the
    // response even when HS256 is explicitly allowed.
    let public_verifier =
        CoreUserInfoVerifier::new(client_id, issuer, CoreJsonWebKeySet::new(vec![]), Some(sub))
            .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256]);
    match user_info_jwt.claims(&public_verifier) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::DisallowedAlg(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }
}

/// The UserInfo subject binding rejects mismatched `sub` claims on both the JSON and signed
/// response paths, independent of the optional `at_hash` check.
#[test]
fn test_user_info_subject_binding() {
    let rsa_key =
        serde_json::from_str::<CoreJsonWebKey>(TEST_RSA_PUB_KEY).expect("deserialization failed");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let sub = SubjectIdentifier::new("the_subject".to_string());

    let json_claims = "{\
                           \"sub\": \"the_subject\",\
                           \"name\": \"Jane Doe\"\
                           }";

    // JSON path: matching subject accepted.
    CoreUserInfoClaims::from_json::<crate::reqwest::Error>(json_claims.as_bytes(), Some(&sub))
        .expect("verification should succeed");

    // JSON path: mismatched subject rejected.
    match CoreUserInfoClaims::from_json::<crate::reqwest::Error>(
        json_claims.as_bytes(),
        Some(&SubjectIdentifier::new("wrong_subject".to_string())),
    ) {
        Err(UserInfoError::ClaimsVerification(ClaimsVerificationError::InvalidSubject(_))) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Signed path: matching subject accepted.
    let signed_verifier = CoreUserInfoVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
        Some(sub.clone()),
    );
    let jwt_claims: CoreUserInfoJsonWebToken = serde_json::from_value(serde_json::Value::String(
        "eyJhbGciOiJSUzI1NiJ9.eyJhdWQiOlsibXlfY2xpZW50Il0sImlzcyI6Imh0dHBzOi8vZXhhb\
                 XBsZS5jb20iLCJzdWIiOiJ0aGVfc3ViamVjdCIsIm5hbWUiOiJKYW5lIERvZSJ9.aX7VpexLAd\
                 43HtC1cFTot3jmqsr105rB50mzTcS1TXzWcxLbqYf1K7Kf-S1oP-ZCL_dnL9-nu3iDK_vRa6xT\
                 nGGt3I1JwhoIv6znSS3JOPT1wtekyD-sLcUwqsJHWBBiTSBwlmGG_kVRuGkBtXgVZ9aGlqg9u1\
                 FlxvyGUJ5q1o9gdb8mKql5ojgsThTNo9qdW3lPIVsiDO-n4mMp4HuOp1re4ZDDkHxiExjtLQAV\
                 kR4q3SlhJC2mkr4mw3_0a2AW52ocWDiwY_lPcdmohmwFaB8aHlivYLFnmKGQIatEW-KDaW5fFo\
                 JYreNkplo4FvzXYyxgxAsqHjHMI8MZVEa1IA"
            .to_string(),
    ))
    .expect("failed to deserialize");
    jwt_claims
        .clone()
        .claims(&signed_verifier)
        .expect("verification should succeed");

    // Signed path: mismatched expected subject is rejected after successful signature
    // verification.
    let mismatched_verifier = CoreUserInfoVerifier::new(
        client_id,
        issuer,
        CoreJsonWebKeySet::new(vec![rsa_key]),
        Some(SubjectIdentifier::new("wrong_subject".to_string())),
    );
    match jwt_claims.claims(&mismatched_verifier) {
        Err(ClaimsVerificationError::InvalidSubject(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }
}

/// The documented `at_hash` flow resolves the effective verification key for both shared-secret
/// (client-secret-derived) and asymmetric (JWKS) ID tokens.
#[test]
fn test_id_token_verification_key_at_hash() {
    use base64::Engine;
    use p256::ecdsa::signature::Signer;
    use p256::elliptic_curve::sec1::ToEncodedPoint;

    let b64 = crate::core::base64_url_safe_no_pad();

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let nonce = Nonce::new("the_nonce".to_string());
    let access_token = AccessToken::new("the_access_token".to_string());
    let secret = "the_client_secret".to_string();

    let mock_current_time = AtomicUsize::new(1544932148);
    let time_fn = || {
        Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
            .to_utc()
            .unwrap()
    };

    let id_claims = CoreIdTokenClaims::new(
        issuer.clone(),
        vec![Audience::new((*client_id).clone())],
        Utc.timestamp_opt(1544932149, 0)
            .single()
            .expect("valid timestamp"),
        Utc.timestamp_opt(1544928549, 0)
            .single()
            .expect("valid timestamp"),
        StandardClaims::new(SubjectIdentifier::new("subject".to_string())),
        Default::default(),
    )
    .set_nonce(Some(nonce.clone()));

    // HS256 ID token with an at_hash computed over the access token.
    let hmac_key = CoreHmacKey::new(secret.clone());
    let id_token_hs256 = CoreIdToken::new(
        id_claims,
        &hmac_key,
        CoreJwsSigningAlgorithm::HmacSha256,
        Some(&access_token),
        None,
    )
    .unwrap();

    // The documented flow works with an EMPTY provider JWKS: the confidential verifier derives
    // the symmetric verification key from the client secret. HS256 requires explicit
    // configuration in addition to the confidential verifier.
    let confidential_verifier = CoreIdTokenVerifier::new_confidential_client(
        client_id.clone(),
        ClientSecret::new(secret.clone()),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
    )
    .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256])
    .set_time_fn(time_fn);
    let hs256_claims = id_token_hs256
        .claims(&confidential_verifier, &nonce)
        .expect("verification should succeed");
    let expected_access_token_hash = hs256_claims.access_token_hash().unwrap().clone();
    let verification_key = id_token_hs256
        .verification_key(&confidential_verifier)
        .expect("verification key should resolve from the client secret");
    let actual_access_token_hash = AccessTokenHash::from_token(
        &access_token,
        id_token_hs256.signing_alg().unwrap(),
        &verification_key,
    )
    .unwrap();
    assert_eq!(actual_access_token_hash, expected_access_token_hash);

    // A substituted access token fails the comparison.
    let substituted_hash = AccessTokenHash::from_token(
        &AccessToken::new("substituted_access_token".to_string()),
        id_token_hs256.signing_alg().unwrap(),
        &verification_key,
    )
    .unwrap();
    assert_ne!(substituted_hash, expected_access_token_hash);

    // A shared-secret algorithm with a verifier that holds no client secret is an error, not a
    // panic.
    let public_verifier = CoreIdTokenVerifier::new_public_client(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
    )
    .set_time_fn(time_fn);
    match id_token_hs256.verification_key(&public_verifier) {
        Err(SignatureVerificationError::DisallowedAlg(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // ES256 at_hash still resolves through the provider JWKS via the same documented call.
    let seed: [u8; 32] = [0x37; 32];
    let es256_signing_key = p256::ecdsa::SigningKey::from_bytes(&seed.into())
        .expect("fixture scalar is a valid P-256 secret key");
    let public_point =
        p256::PublicKey::from(es256_signing_key.verifying_key()).to_encoded_point(false);
    let public_bytes = public_point.as_bytes();
    let es256_key: CoreJsonWebKey = serde_json::from_value(serde_json::json!({
        "kty": "EC",
        "kid": "test-es256-key",
        "crv": "P-256",
        "x": b64.encode(&public_bytes[1..33]),
        "y": b64.encode(&public_bytes[33..65]),
        "use": "sig",
    }))
    .expect("deserialization failed");

    let header = "{\"alg\":\"ES256\",\"typ\":\"JWT\",\"kid\":\"test-es256-key\"}";
    let payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"subject\",\"exp\":1544932149,\"iat\":1544928549,\"nonce\":\"the_nonce\"}";
    let signing_input = format!(
        "{}.{}",
        b64.encode(header.as_bytes()),
        b64.encode(payload.as_bytes())
    );
    let signature: p256::ecdsa::Signature = es256_signing_key.sign(signing_input.as_bytes());
    let id_token_es256: CoreIdToken =
        format!("{}.{}", signing_input, b64.encode(signature.to_bytes()))
            .parse()
            .expect("failed to deserialize");

    let es256_verifier = CoreIdTokenVerifier::new_public_client(
        client_id,
        issuer,
        CoreJsonWebKeySet::new(vec![es256_key.clone()]),
    )
    .set_allowed_algs(vec![CoreJwsSigningAlgorithm::EcdsaP256Sha256])
    .set_time_fn(time_fn);
    id_token_es256
        .claims(&es256_verifier, &nonce)
        .expect("verification should succeed");
    let es256_verification_key = id_token_es256
        .verification_key(&es256_verifier)
        .expect("verification key should resolve from the JWKS");
    // The resolved key is the provider's ES256 JWK: it hashes exactly like the fixture key.
    let from_resolved = es256_verification_key
        .hash_bytes(
            access_token.secret().as_bytes(),
            &CoreJwsSigningAlgorithm::EcdsaP256Sha256,
        )
        .unwrap();
    let from_fixture = es256_key
        .hash_bytes(
            access_token.secret().as_bytes(),
            &CoreJwsSigningAlgorithm::EcdsaP256Sha256,
        )
        .unwrap();
    assert_eq!(from_resolved, from_fixture);
    let es256_at_hash = AccessTokenHash::from_token(
        &access_token,
        id_token_es256.signing_alg().unwrap(),
        &es256_verification_key,
    )
    .unwrap();
    let es256_expected = AccessTokenHash::new(b64.encode(&from_fixture[0..from_fixture.len() / 2]));
    assert_eq!(es256_at_hash, es256_expected);
}

/// Signed user info responses are accepted only for explicitly allowed algorithms; the default
/// allowlist is RS256-only and there is no way to allow any algorithm. ES384 responses verify
/// against a matching P-384 JWK once ES384 is explicitly allowed.
#[test]
fn test_user_info_signed_response_es384() {
    use base64::Engine;
    use p384::ecdsa::signature::Signer;
    use p384::elliptic_curve::sec1::ToEncodedPoint;

    let b64 = crate::core::base64_url_safe_no_pad();

    // Deterministic P-384 fixture keys (same derivation as the P-256 fixtures above).
    let seed: [u8; 48] = [0x3b; 48];
    let signing_key = p384::ecdsa::SigningKey::from_bytes(&seed.into())
        .expect("fixture scalar is a valid P-384 secret key");
    let other_seed: [u8; 48] = [0x5c; 48];
    let other_signing_key = p384::ecdsa::SigningKey::from_bytes(&other_seed.into())
        .expect("fixture scalar is a valid P-384 secret key");

    let public_point = p384::PublicKey::from(signing_key.verifying_key()).to_encoded_point(false);
    let public_bytes = public_point.as_bytes();
    let es384_key: CoreJsonWebKey = serde_json::from_value(serde_json::json!({
        "kty": "EC",
        "kid": "test-es384-key",
        "crv": "P-384",
        "x": b64.encode(&public_bytes[1..49]),
        "y": b64.encode(&public_bytes[49..97]),
        "use": "sig",
    }))
    .expect("deserialization failed");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let sub = SubjectIdentifier::new("the_subject".to_string());

    let sign_es384 = |key: &p384::ecdsa::SigningKey, payload: &str| -> String {
        let header = "{\"alg\":\"ES384\",\"typ\":\"JWT\",\"kid\":\"test-es384-key\"}";
        let signing_input = format!(
            "{}.{}",
            b64.encode(header.as_bytes()),
            b64.encode(payload.as_bytes())
        );
        let signature: p384::ecdsa::Signature = key.sign(signing_input.as_bytes());
        format!("{}.{}", signing_input, b64.encode(signature.to_bytes()))
    };

    let valid_payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"the_subject\",\"name\":\"Jane Doe\"}";
    let user_info_jwt: CoreUserInfoJsonWebToken = serde_json::from_value(
        serde_json::Value::String(sign_es384(&signing_key, valid_payload)),
    )
    .expect("failed to deserialize");

    // ES384 is accepted only once explicitly allowed.
    let verifier = CoreUserInfoVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![es384_key.clone()]),
        Some(sub.clone()),
    )
    .set_allowed_algs(vec![CoreJwsSigningAlgorithm::EcdsaP384Sha384]);
    let claims = user_info_jwt
        .clone()
        .claims(&verifier)
        .expect("verification should succeed");
    assert_eq!(*claims.subject(), sub);

    // The default verifier rejects ES384 with DisallowedAlg (RS256-only default).
    let default_verifier = CoreUserInfoVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![es384_key.clone()]),
        Some(sub.clone()),
    );
    match user_info_jwt.clone().claims(&default_verifier) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::DisallowedAlg(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Altered signature is rejected.
    let token_valid = sign_es384(&signing_key, valid_payload);
    let (signing_input, signature) = token_valid.rsplit_once('.').unwrap();
    let mut tampered_signature = signature.to_string();
    // Flip the first signature character: it always encodes six real bits (the final
    // base64 character of an unpadded encoding may only carry padding bits).
    let replacement = if tampered_signature.starts_with('A') {
        'B'
    } else {
        'A'
    };
    tampered_signature.replace_range(0..1, &replacement.to_string());
    let tampered: CoreUserInfoJsonWebToken = serde_json::from_value(serde_json::Value::String(
        format!("{}.{}", signing_input, tampered_signature),
    ))
    .expect("failed to deserialize");
    match tampered.claims(&verifier) {
        Err(ClaimsVerificationError::SignatureVerification(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Signature by the wrong key is rejected.
    let wrong_key: CoreUserInfoJsonWebToken = serde_json::from_value(serde_json::Value::String(
        sign_es384(&other_signing_key, valid_payload),
    ))
    .expect("failed to deserialize");
    match wrong_key.claims(&verifier) {
        Err(ClaimsVerificationError::SignatureVerification(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }
}

/// Exercises the signed user info policy for one shared-secret algorithm: an explicitly allowed
/// confidential verifier accepts the response using the client secret, a wrong secret is
/// rejected, and a public verifier rejects the response even when the algorithm is allowed.
fn assert_hs_user_info_signed_response_policy(alg: CoreJwsSigningAlgorithm) {
    use base64::Engine;

    let b64 = crate::core::base64_url_safe_no_pad();
    let alg_name = serde_plain::to_string(&alg).expect("alg should serialize to its JOSE name");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let sub = SubjectIdentifier::new("the_subject".to_string());
    let client_secret = ClientSecret::new("the_client_secret".to_string());

    let payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"the_subject\",\"name\":\"Jane Doe\"}";
    let header = format!("{{\"alg\":\"{alg_name}\",\"typ\":\"JWT\"}}");
    let signing_input = format!(
        "{}.{}",
        b64.encode(header.as_bytes()),
        b64.encode(payload.as_bytes())
    );
    let hmac_key = CoreHmacKey::new("the_client_secret");
    let signature = hmac_key
        .sign(&alg, signing_input.as_bytes())
        .expect("HS signing should succeed");
    let user_info_jwt: CoreUserInfoJsonWebToken = serde_json::from_value(
        serde_json::Value::String(format!("{}.{}", signing_input, b64.encode(signature))),
    )
    .expect("failed to deserialize");

    // Accepted via the confidential verifier with the matching secret (empty JWKS: the
    // verification key is derived from the client secret). The shared-secret algorithm requires
    // explicit configuration in addition to the confidential verifier.
    let verifier = CoreUserInfoVerifier::new_confidential_client(
        client_id.clone(),
        client_secret.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
        Some(sub.clone()),
    )
    .set_allowed_algs(vec![alg.clone()]);
    let claims = user_info_jwt
        .clone()
        .claims(&verifier)
        .expect("verification should succeed");
    assert_eq!(*claims.subject(), sub);

    // Wrong secret is rejected.
    let wrong_secret_verifier = CoreUserInfoVerifier::new_confidential_client(
        client_id.clone(),
        ClientSecret::new("the_wrong_secret".to_string()),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![]),
        Some(sub.clone()),
    )
    .set_allowed_algs(vec![alg.clone()]);
    match user_info_jwt.clone().claims(&wrong_secret_verifier) {
        Err(ClaimsVerificationError::SignatureVerification(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }

    // Shared-secret algorithms additionally require a confidential verifier: a public verifier
    // rejects the response even when the algorithm is explicitly allowed.
    let public_verifier =
        CoreUserInfoVerifier::new(client_id, issuer, CoreJsonWebKeySet::new(vec![]), Some(sub))
            .set_allowed_algs(vec![alg.clone()]);
    match user_info_jwt.claims(&public_verifier) {
        Err(ClaimsVerificationError::SignatureVerification(
            SignatureVerificationError::DisallowedAlg(_),
        )) => {}
        other => panic!("unexpected result: {:?}", other),
    }
}

/// HS384-signed user info responses verify through a confidential verifier with an explicitly
/// allowed algorithm using the client secret; a public verifier must reject them even when the
/// algorithm is allowed.
#[test]
fn test_user_info_signed_response_hs384() {
    assert_hs_user_info_signed_response_policy(CoreJwsSigningAlgorithm::HmacSha384);
}

/// HS512-signed user info responses verify through a confidential verifier with an explicitly
/// allowed algorithm using the client secret; a public verifier must reject them even when the
/// algorithm is allowed.
#[test]
fn test_user_info_signed_response_hs512() {
    assert_hs_user_info_signed_response_policy(CoreJwsSigningAlgorithm::HmacSha512);
}

/// With no expected subject (`Client::user_info(..., None)` mode), both the JSON and signed
/// UserInfo paths accept any response subject rather than applying a comparison; the bound
/// mode's mismatch rejection is covered by `test_user_info_subject_binding`.
#[test]
fn test_user_info_subject_binding_none_mode() {
    let rsa_key =
        serde_json::from_str::<CoreJsonWebKey>(TEST_RSA_PUB_KEY).expect("deserialization failed");

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let other_sub = SubjectIdentifier::new("some_other_subject".to_string());

    // JSON path: a response subject that would fail any bound comparison is accepted when no
    // expected subject is supplied.
    let json_claims = "{\
                       \"sub\": \"some_other_subject\",\
                       \"name\": \"Jane Doe\"\
                       }";
    let json_user_info =
        CoreUserInfoClaims::from_json::<crate::reqwest::Error>(json_claims.as_bytes(), None)
            .expect("verification should succeed");
    assert_eq!(*json_user_info.subject(), other_sub);

    // Signed path: the same None mode reaches the signed verifier, which verifies the signature
    // and then accepts any response subject.
    let none_verifier = CoreUserInfoVerifier::new(
        client_id.clone(),
        issuer.clone(),
        CoreJsonWebKeySet::new(vec![rsa_key.clone()]),
        None,
    );
    let claims =
        CoreUserInfoClaims::new(StandardClaims::new(other_sub.clone()), Default::default())
            .set_issuer(Some(issuer.clone()))
            .set_audiences(Some(vec![Audience::new((*client_id).clone())]));
    let rsa_priv_key = CoreRsaPrivateSigningKey::from_pem(TEST_RSA_PRIV_KEY, None).unwrap();
    let jwt_claims: CoreUserInfoJsonWebToken = CoreUserInfoJsonWebToken::new(
        claims,
        &rsa_priv_key,
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
    )
    .unwrap();
    let signed_user_info = jwt_claims
        .clone()
        .claims(&none_verifier)
        .expect("verification should succeed");
    assert_eq!(*signed_user_info.subject(), other_sub);

    // Contrast: with a bound expected subject, the same response is rejected after signature
    // verification.
    let bound_verifier = CoreUserInfoVerifier::new(
        client_id,
        issuer,
        CoreJsonWebKeySet::new(vec![rsa_key]),
        Some(SubjectIdentifier::new("the_subject".to_string())),
    );
    match jwt_claims.claims(&bound_verifier) {
        Err(ClaimsVerificationError::InvalidSubject(_)) => {}
        other => panic!("unexpected result: {:?}", other),
    }
}

/// The documented `at_hash` flow works for the remaining shared-secret algorithms: HS384 and
/// HS512 ID tokens verify through a confidential verifier with an empty JWKS, and the
/// client-secret-derived verification key reproduces the access-token hash.
#[test]
fn test_id_token_verification_key_at_hash_hs384_hs512() {
    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let nonce = Nonce::new("the_nonce".to_string());
    let access_token = AccessToken::new("the_access_token".to_string());
    let secret = "the_client_secret".to_string();

    let mock_current_time = AtomicUsize::new(1544932148);
    let time_fn = || {
        Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
            .to_utc()
            .unwrap()
    };

    let id_claims = CoreIdTokenClaims::new(
        issuer.clone(),
        vec![Audience::new((*client_id).clone())],
        Utc.timestamp_opt(1544932149, 0)
            .single()
            .expect("valid timestamp"),
        Utc.timestamp_opt(1544928549, 0)
            .single()
            .expect("valid timestamp"),
        StandardClaims::new(SubjectIdentifier::new("subject".to_string())),
        Default::default(),
    )
    .set_nonce(Some(nonce.clone()));

    let hmac_key = CoreHmacKey::new(secret);
    for alg in [
        CoreJwsSigningAlgorithm::HmacSha384,
        CoreJwsSigningAlgorithm::HmacSha512,
    ] {
        // Shared-secret ID token with an at_hash computed over the access token.
        let id_token = CoreIdToken::new(
            id_claims.clone(),
            &hmac_key,
            alg.clone(),
            Some(&access_token),
            None,
        )
        .unwrap();

        // The documented flow works with an EMPTY provider JWKS: the confidential verifier
        // derives the symmetric verification key from the client secret.
        let confidential_verifier = CoreIdTokenVerifier::new_confidential_client(
            client_id.clone(),
            ClientSecret::new("the_client_secret".to_string()),
            issuer.clone(),
            CoreJsonWebKeySet::new(vec![]),
        )
        .set_allowed_algs(vec![alg.clone()])
        .set_time_fn(time_fn);
        let claims = id_token
            .claims(&confidential_verifier, &nonce)
            .expect("verification should succeed");
        let expected_access_token_hash = claims.access_token_hash().unwrap().clone();
        let verification_key = id_token
            .verification_key(&confidential_verifier)
            .expect("verification key should resolve from the client secret");
        let actual_access_token_hash = AccessTokenHash::from_token(
            &access_token,
            id_token.signing_alg().unwrap(),
            &verification_key,
        )
        .unwrap();
        assert_eq!(actual_access_token_hash, expected_access_token_hash);

        // A substituted access token fails the comparison.
        let substituted_hash = AccessTokenHash::from_token(
            &AccessToken::new("substituted_access_token".to_string()),
            id_token.signing_alg().unwrap(),
            &verification_key,
        )
        .unwrap();
        assert_ne!(substituted_hash, expected_access_token_hash);
    }
}

/// The documented `at_hash` flow resolves the matching P-384 provider JWK for ES384 ID tokens
/// and reproduces the access-token hash.
#[test]
fn test_id_token_verification_key_at_hash_es384() {
    use base64::Engine;
    use p384::ecdsa::signature::Signer;
    use p384::elliptic_curve::sec1::ToEncodedPoint;

    let b64 = crate::core::base64_url_safe_no_pad();

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let nonce = Nonce::new("the_nonce".to_string());
    let access_token = AccessToken::new("the_access_token".to_string());

    let mock_current_time = AtomicUsize::new(1544932148);
    let time_fn = || {
        Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
            .to_utc()
            .unwrap()
    };

    // Deterministic P-384 fixture key (same derivation as the P-256 fixtures above).
    let seed: [u8; 48] = [0x3b; 48];
    let es384_signing_key = p384::ecdsa::SigningKey::from_bytes(&seed.into())
        .expect("fixture scalar is a valid P-384 secret key");
    let public_point =
        p384::PublicKey::from(es384_signing_key.verifying_key()).to_encoded_point(false);
    let public_bytes = public_point.as_bytes();
    let es384_key: CoreJsonWebKey = serde_json::from_value(serde_json::json!({
        "kty": "EC",
        "kid": "test-es384-key",
        "crv": "P-384",
        "x": b64.encode(&public_bytes[1..49]),
        "y": b64.encode(&public_bytes[49..97]),
        "use": "sig",
    }))
    .expect("deserialization failed");

    let header = "{\"alg\":\"ES384\",\"typ\":\"JWT\",\"kid\":\"test-es384-key\"}";
    let payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"subject\",\"exp\":1544932149,\"iat\":1544928549,\"nonce\":\"the_nonce\"}";
    let signing_input = format!(
        "{}.{}",
        b64.encode(header.as_bytes()),
        b64.encode(payload.as_bytes())
    );
    let signature: p384::ecdsa::Signature = es384_signing_key.sign(signing_input.as_bytes());
    let id_token_es384: CoreIdToken =
        format!("{}.{}", signing_input, b64.encode(signature.to_bytes()))
            .parse()
            .expect("failed to deserialize");

    let es384_verifier = CoreIdTokenVerifier::new_public_client(
        client_id,
        issuer,
        CoreJsonWebKeySet::new(vec![es384_key.clone()]),
    )
    .set_allowed_algs(vec![CoreJwsSigningAlgorithm::EcdsaP384Sha384])
    .set_time_fn(time_fn);
    id_token_es384
        .claims(&es384_verifier, &nonce)
        .expect("verification should succeed");
    let es384_verification_key = id_token_es384
        .verification_key(&es384_verifier)
        .expect("verification key should resolve from the JWKS");
    // The resolved key is the provider's ES384 JWK: it hashes exactly like the fixture key.
    let from_resolved = es384_verification_key
        .hash_bytes(
            access_token.secret().as_bytes(),
            &CoreJwsSigningAlgorithm::EcdsaP384Sha384,
        )
        .unwrap();
    let from_fixture = es384_key
        .hash_bytes(
            access_token.secret().as_bytes(),
            &CoreJwsSigningAlgorithm::EcdsaP384Sha384,
        )
        .unwrap();
    assert_eq!(from_resolved, from_fixture);
    let es384_at_hash = AccessTokenHash::from_token(
        &access_token,
        id_token_es384.signing_alg().unwrap(),
        &es384_verification_key,
    )
    .unwrap();
    let es384_expected = AccessTokenHash::new(b64.encode(&from_fixture[0..from_fixture.len() / 2]));
    assert_eq!(es384_at_hash, es384_expected);
}

/// The documented `at_hash` flow resolves the matching provider JWK for the remaining asymmetric
/// signature families: RSA PKCS#1 v1.5 (`RS256`/`RS384`/`RS512`), RSA-PSS (`PS256`/`PS384`/
/// `PS512`), and EdDSA (Ed25519) ID tokens verify through a public verifier against the provider
/// JWKS, `IdToken::verification_key` resolves that JWK, and `AccessTokenHash::from_token` over
/// the resolved key reproduces the embedded at_hash.
#[test]
fn test_id_token_verification_key_at_hash_rsa_pss_eddsa() {
    // Test-only Ed25519 signing key, mirroring TEST_ED25519_KEY in src/core/jwk/tests.rs (that
    // constant is private to the JWK test module). The matching public key is published as an
    // OKP JWK by `CoreEdDsaPrivateSigningKey::as_verification_key`.
    const TEST_ED25519_KEY: &str = "\
        -----BEGIN PRIVATE KEY-----\n\
        MC4CAQAwBQYDK2VwBCIEICWeYPLxoZKHZlQ6rkBi11E9JwchynXtljATLqym/XS9\n\
        -----END PRIVATE KEY-----\
        ";

    // Test-fixture plumbing that names every input explicitly for the seven
    // algorithm-family legs; a parameter struct would obscure the table rows.
    #[allow(clippy::too_many_arguments)]
    fn assert_jwks_resolved_at_hash<S>(
        signing_key: &S,
        alg: CoreJwsSigningAlgorithm,
        verification_key: CoreJsonWebKey,
        id_claims: &CoreIdTokenClaims,
        access_token: &AccessToken,
        nonce: &Nonce,
        client_id: &ClientId,
        issuer: &IssuerUrl,
        mock_current_time: &AtomicUsize,
    ) where
        S: PrivateSigningKey,
        S::VerificationKey: JsonWebKey<SigningAlgorithm = CoreJwsSigningAlgorithm>,
    {
        let time_fn = || {
            Timestamp::Seconds(mock_current_time.load(Ordering::Relaxed).into())
                .to_utc()
                .unwrap()
        };

        // The provider signs the ID token with this family's private key;
        // `CoreIdToken::new` embeds the at_hash over the access token for the same algorithm.
        let id_token = CoreIdToken::new(
            id_claims.clone(),
            signing_key,
            alg.clone(),
            Some(access_token),
            None,
        )
        .expect("ID token signing should succeed");

        let verifier = CoreIdTokenVerifier::new_public_client(
            client_id.clone(),
            issuer.clone(),
            CoreJsonWebKeySet::new(vec![verification_key.clone()]),
        )
        .set_allowed_algs(vec![alg.clone()])
        .set_time_fn(time_fn);
        let claims = id_token
            .claims(&verifier, nonce)
            .expect("verification should succeed");
        let expected_access_token_hash = claims.access_token_hash().unwrap().clone();

        // The documented flow resolves the matching provider JWK from the JWKS.
        let resolved_key = id_token
            .verification_key(&verifier)
            .expect("verification key should resolve from the JWKS");

        // The resolved key is the provider's JWK for this family: it hashes exactly like the
        // fixture key.
        let from_resolved = resolved_key
            .hash_bytes(access_token.secret().as_bytes(), &alg)
            .unwrap();
        let from_fixture = verification_key
            .hash_bytes(access_token.secret().as_bytes(), &alg)
            .unwrap();
        assert_eq!(from_resolved, from_fixture);

        let actual_access_token_hash = AccessTokenHash::from_token(
            access_token,
            id_token.signing_alg().unwrap(),
            &resolved_key,
        )
        .unwrap();
        assert_eq!(actual_access_token_hash, expected_access_token_hash);

        // A substituted access token fails the comparison.
        let substituted_hash = AccessTokenHash::from_token(
            &AccessToken::new("substituted_access_token".to_string()),
            id_token.signing_alg().unwrap(),
            &resolved_key,
        )
        .unwrap();
        assert_ne!(substituted_hash, expected_access_token_hash);
    }

    let client_id = ClientId::new("my_client".to_string());
    let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
    let nonce = Nonce::new("the_nonce".to_string());
    let access_token = AccessToken::new("the_access_token".to_string());

    let mock_current_time = AtomicUsize::new(1544932148);

    let id_claims = CoreIdTokenClaims::new(
        issuer.clone(),
        vec![Audience::new((*client_id).clone())],
        Utc.timestamp_opt(1544932149, 0)
            .single()
            .expect("valid timestamp"),
        Utc.timestamp_opt(1544928549, 0)
            .single()
            .expect("valid timestamp"),
        StandardClaims::new(SubjectIdentifier::new("subject".to_string())),
        Default::default(),
    )
    .set_nonce(Some(nonce.clone()));

    // One RSA fixture key signs every RSA family (PKCS#1 v1.5 and PSS); its public JWK (n, e)
    // is published in the verifier's JWKS.
    let rsa_priv_key = CoreRsaPrivateSigningKey::from_pem(
        TEST_RSA_PRIV_KEY,
        Some(JsonWebKeyId::new("test-rsa-key".to_string())),
    )
    .expect("key parsing failed");
    let rsa_verification_key = rsa_priv_key.as_verification_key();
    for alg in [
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha384,
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha512,
        CoreJwsSigningAlgorithm::RsaSsaPssSha256,
        CoreJwsSigningAlgorithm::RsaSsaPssSha384,
        CoreJwsSigningAlgorithm::RsaSsaPssSha512,
    ] {
        assert_jwks_resolved_at_hash(
            &rsa_priv_key,
            alg,
            rsa_verification_key.clone(),
            &id_claims,
            &access_token,
            &nonce,
            &client_id,
            &issuer,
            &mock_current_time,
        );
    }

    // EdDSA (Ed25519) through the crate's Ed25519 private signing key.
    let eddsa_priv_key = CoreEdDsaPrivateSigningKey::from_ed25519_pem(
        TEST_ED25519_KEY,
        Some(JsonWebKeyId::new("test-eddsa-key".to_string())),
    )
    .expect("key parsing failed");
    assert_jwks_resolved_at_hash(
        &eddsa_priv_key,
        CoreJwsSigningAlgorithm::EdDsa,
        eddsa_priv_key.as_verification_key(),
        &id_claims,
        &access_token,
        &nonce,
        &client_id,
        &issuer,
        &mock_current_time,
    );
}
