use crate::core::{
    CoreApplicationType, CoreClientAuthMethod, CoreClientMetadata, CoreClientRegistrationResponse,
    CoreGrantType, CoreJweContentEncryptionAlgorithm, CoreJweKeyManagementAlgorithm,
    CoreJwsSigningAlgorithm, CoreResponseType, CoreSubjectIdentifierType,
};
use crate::jwt::tests::TEST_RSA_PUB_KEY;
use crate::registration::ClientSecretExpiration;
use crate::{
    AuthenticationContextClass, ClientConfigUrl, ClientContactEmail, ClientName, ClientUrl,
    JsonWebKeySet, JsonWebKeySetUrl, LanguageTag, LogoUrl, PolicyUrl, RequestUrl, ResponseTypes,
    SectorIdentifierUrl, ToSUrl,
};
use crate::{ClientId, RedirectUrl};

use chrono::{TimeZone, Utc};
use itertools::sorted;

use std::time::Duration;

#[test]
fn test_metadata_serialization() {
    // `jwks_uri` and `jwks` aren't supposed to be used together, but this test is just for
    // serialization/deserialization.
    let json_response = format!("{{
            \"redirect_uris\": [\"https://example.com/redirect-1\", \"https://example.com/redirect-2\"],
            \"response_types\": [\"code\", \"code token id_token\"],
            \"grant_types\": [\"authorization_code\", \"client_credentials\", \"implicit\", \
                \"password\", \"refresh_token\"],
            \"application_type\": \"web\",
            \"contacts\": [\"user@example.com\", \"admin@openidconnect.local\"],
            \"client_name\": \"Example\",
            \"client_name#es\": \"Ejemplo\",
            \"logo_uri\": \"https://example.com/logo.png\",
            \"logo_uri#fr\": \"https://example.com/logo-fr.png\",
            \"client_uri\": \"https://example.com/client-app\",
            \"client_uri#de\": \"https://example.com/client-app-de\",
            \"policy_uri\": \"https://example.com/policy\",
            \"policy_uri#sr-Latn\": \"https://example.com/policy-sr-latin\",
            \"tos_uri\": \"https://example.com/tos\",
            \"tos_uri#sr-Cyrl\": \"https://example.com/tos-sr-cyrl\",
            \"jwks_uri\": \"https://example.com/jwks\",
            \"jwks\": {{\"keys\": [{}]}},
            \"sector_identifier_uri\": \"https://example.com/sector\",
            \"subject_type\": \"pairwise\",
            \"id_token_signed_response_alg\": \"HS256\",
            \"id_token_encrypted_response_alg\": \"RSA1_5\",
            \"id_token_encrypted_response_enc\": \"A128CBC-HS256\",
            \"userinfo_signed_response_alg\": \"RS384\",
            \"userinfo_encrypted_response_alg\": \"RSA-OAEP\",
            \"userinfo_encrypted_response_enc\": \"A256CBC-HS512\",
            \"request_object_signing_alg\": \"ES512\",
            \"request_object_encryption_alg\": \"ECDH-ES+A128KW\",
            \"request_object_encryption_enc\": \"A256GCM\",
            \"token_endpoint_auth_method\": \"client_secret_basic\",
            \"token_endpoint_auth_signing_alg\": \"PS512\",
            \"default_max_age\": 3600,
            \"require_auth_time\": true,
            \"default_acr_values\": [\"0\", \"urn:mace:incommon:iap:silver\", \
                \"urn:mace:incommon:iap:bronze\"],
            \"initiate_login_uri\": \"https://example.com/login\",
            \"request_uris\": [\"https://example.com/request-1\", \"https://example.com/request-2\"]
        }}", TEST_RSA_PUB_KEY);

    let client_metadata: CoreClientMetadata = serde_json::from_str(&json_response).unwrap();

    assert_eq!(
        *client_metadata.redirect_uris(),
        vec![
            RedirectUrl::new("https://example.com/redirect-1".to_string()).unwrap(),
            RedirectUrl::new("https://example.com/redirect-2".to_string()).unwrap(),
        ]
    );
    assert_eq!(
        *client_metadata.response_types().unwrap(),
        vec![
            ResponseTypes::new(vec![CoreResponseType::Code]),
            ResponseTypes::new(vec![
                CoreResponseType::Code,
                CoreResponseType::Token,
                CoreResponseType::IdToken,
            ]),
        ]
    );
    assert_eq!(
        client_metadata.grant_types().unwrap(),
        &vec![
            CoreGrantType::AuthorizationCode,
            CoreGrantType::ClientCredentials,
            CoreGrantType::Implicit,
            CoreGrantType::Password,
            CoreGrantType::RefreshToken,
        ]
    );
    assert_eq!(
        *client_metadata.application_type().unwrap(),
        CoreApplicationType::Web
    );
    assert_eq!(
        *client_metadata.contacts().unwrap(),
        vec![
            ClientContactEmail::new("user@example.com".to_string()),
            ClientContactEmail::new("admin@openidconnect.local".to_string()),
        ]
    );
    assert_eq!(
        sorted(client_metadata.client_name().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, ClientName)>>(),
        vec![
            (None, ClientName::new("Example".to_string())),
            (
                Some(LanguageTag::new("es".to_string())),
                ClientName::new("Ejemplo".to_string()),
            ),
        ]
    );
    assert_eq!(
        sorted(client_metadata.logo_uri().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, LogoUrl)>>(),
        vec![
            (
                None,
                LogoUrl::new("https://example.com/logo.png".to_string()).unwrap(),
            ),
            (
                Some(LanguageTag::new("fr".to_string())),
                LogoUrl::new("https://example.com/logo-fr.png".to_string()).unwrap(),
            ),
        ]
    );
    assert_eq!(
        sorted(client_metadata.client_uri().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, ClientUrl)>>(),
        vec![
            (
                None,
                ClientUrl::new("https://example.com/client-app".to_string()).unwrap(),
            ),
            (
                Some(LanguageTag::new("de".to_string())),
                ClientUrl::new("https://example.com/client-app-de".to_string()).unwrap(),
            ),
        ]
    );
    assert_eq!(
        sorted(client_metadata.policy_uri().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, PolicyUrl)>>(),
        vec![
            (
                None,
                PolicyUrl::new("https://example.com/policy".to_string()).unwrap(),
            ),
            (
                Some(LanguageTag::new("sr-Latn".to_string())),
                PolicyUrl::new("https://example.com/policy-sr-latin".to_string()).unwrap(),
            ),
        ]
    );
    assert_eq!(
        sorted(client_metadata.tos_uri().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, ToSUrl)>>(),
        vec![
            (
                None,
                ToSUrl::new("https://example.com/tos".to_string()).unwrap(),
            ),
            (
                Some(LanguageTag::new("sr-Cyrl".to_string())),
                ToSUrl::new("https://example.com/tos-sr-cyrl".to_string()).unwrap(),
            ),
        ]
    );
    assert_eq!(
        *client_metadata.jwks_uri().unwrap(),
        JsonWebKeySetUrl::new("https://example.com/jwks".to_string()).unwrap()
    );
    assert_eq!(
        client_metadata.jwks(),
        Some(&JsonWebKeySet::new(vec![serde_json::from_str(
            TEST_RSA_PUB_KEY
        )
        .unwrap()],))
    );
    assert_eq!(
        *client_metadata.sector_identifier_uri().unwrap(),
        SectorIdentifierUrl::new("https://example.com/sector".to_string()).unwrap()
    );
    assert_eq!(
        *client_metadata.subject_type().unwrap(),
        CoreSubjectIdentifierType::Pairwise
    );
    assert_eq!(
        *client_metadata.id_token_signed_response_alg().unwrap(),
        CoreJwsSigningAlgorithm::HmacSha256
    );
    assert_eq!(
        *client_metadata.id_token_encrypted_response_alg().unwrap(),
        CoreJweKeyManagementAlgorithm::RsaPkcs1V15
    );
    assert_eq!(
        *client_metadata.id_token_encrypted_response_enc().unwrap(),
        CoreJweContentEncryptionAlgorithm::Aes128CbcHmacSha256
    );
    assert_eq!(
        *client_metadata.userinfo_signed_response_alg().unwrap(),
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha384
    );
    assert_eq!(
        *client_metadata.userinfo_encrypted_response_alg().unwrap(),
        CoreJweKeyManagementAlgorithm::RsaOaep
    );
    assert_eq!(
        *client_metadata.userinfo_encrypted_response_enc().unwrap(),
        CoreJweContentEncryptionAlgorithm::Aes256CbcHmacSha512
    );
    assert_eq!(
        *client_metadata.request_object_signing_alg().unwrap(),
        CoreJwsSigningAlgorithm::EcdsaP521Sha512
    );
    assert_eq!(
        *client_metadata.request_object_encryption_alg().unwrap(),
        CoreJweKeyManagementAlgorithm::EcdhEsAesKeyWrap128
    );
    assert_eq!(
        *client_metadata.request_object_encryption_enc().unwrap(),
        CoreJweContentEncryptionAlgorithm::Aes256Gcm
    );
    assert_eq!(
        *client_metadata.token_endpoint_auth_method().unwrap(),
        CoreClientAuthMethod::ClientSecretBasic
    );
    assert_eq!(
        *client_metadata.token_endpoint_auth_signing_alg().unwrap(),
        CoreJwsSigningAlgorithm::RsaSsaPssSha512
    );
    assert_eq!(
        *client_metadata.default_max_age().unwrap(),
        Duration::from_secs(3600)
    );
    assert!(client_metadata.require_auth_time().unwrap());
    assert_eq!(
        *client_metadata.default_acr_values().unwrap(),
        vec![
            AuthenticationContextClass::new("0".to_string()),
            AuthenticationContextClass::new("urn:mace:incommon:iap:silver".to_string()),
            AuthenticationContextClass::new("urn:mace:incommon:iap:bronze".to_string()),
        ]
    );
    assert_eq!(
        *client_metadata.sector_identifier_uri().unwrap(),
        SectorIdentifierUrl::new("https://example.com/sector".to_string()).unwrap()
    );
    assert_eq!(
        *client_metadata.request_uris().unwrap(),
        vec![
            RequestUrl::new("https://example.com/request-1".to_string()).unwrap(),
            RequestUrl::new("https://example.com/request-2".to_string()).unwrap(),
        ]
    );
    let serialized_json = serde_json::to_string(&client_metadata).unwrap();

    assert_eq!(
        client_metadata,
        serde_json::from_str(&serialized_json).unwrap()
    );
}

#[test]
fn test_metadata_serialization_minimal() {
    let json_response = "{\"redirect_uris\": [\"https://example.com/redirect-1\"]}";

    let client_metadata: CoreClientMetadata = serde_json::from_str(json_response).unwrap();

    assert_eq!(
        *client_metadata.redirect_uris(),
        vec![RedirectUrl::new("https://example.com/redirect-1".to_string()).unwrap(),]
    );
    assert_eq!(client_metadata.response_types(), None);
    assert_eq!(client_metadata.grant_types(), None);
    assert_eq!(client_metadata.application_type(), None);
    assert_eq!(client_metadata.contacts(), None);
    assert_eq!(client_metadata.client_name(), None);
    assert_eq!(client_metadata.logo_uri(), None);
    assert_eq!(client_metadata.client_uri(), None);
    assert_eq!(client_metadata.policy_uri(), None);
    assert_eq!(client_metadata.tos_uri(), None);
    assert_eq!(client_metadata.jwks_uri(), None);
    assert_eq!(client_metadata.jwks(), None);
    assert_eq!(client_metadata.sector_identifier_uri(), None);
    assert_eq!(client_metadata.subject_type(), None);
    assert_eq!(client_metadata.id_token_signed_response_alg(), None);
    assert_eq!(client_metadata.id_token_encrypted_response_alg(), None);
    assert_eq!(client_metadata.id_token_encrypted_response_enc(), None);
    assert_eq!(client_metadata.userinfo_signed_response_alg(), None);
    assert_eq!(client_metadata.userinfo_encrypted_response_alg(), None);
    assert_eq!(client_metadata.userinfo_encrypted_response_enc(), None);
    assert_eq!(client_metadata.request_object_signing_alg(), None);
    assert_eq!(client_metadata.request_object_encryption_alg(), None);
    assert_eq!(client_metadata.request_object_encryption_enc(), None);
    assert_eq!(client_metadata.token_endpoint_auth_method(), None);
    assert_eq!(client_metadata.token_endpoint_auth_signing_alg(), None);
    assert_eq!(client_metadata.default_max_age(), None);
    assert_eq!(client_metadata.require_auth_time(), None);
    assert_eq!(client_metadata.default_acr_values(), None);
    assert_eq!(client_metadata.sector_identifier_uri(), None);
    assert_eq!(client_metadata.request_uris(), None);

    let serialized_json = serde_json::to_string(&client_metadata).unwrap();

    assert_eq!(
        client_metadata,
        serde_json::from_str(&serialized_json).unwrap()
    );
}

#[test]
fn test_response_serialization() {
    let json_response = format!("{{
            \"client_id\": \"abcdefgh\",
            \"client_secret\": \"shhhh\",
            \"registration_access_token\": \"use_me_to_update_registration\",
            \"registration_client_uri\": \"https://example-provider.com/registration\",
            \"client_id_issued_at\": 1523953306,
            \"client_secret_expires_at\": 1526545306,
            \"redirect_uris\": [\"https://example.com/redirect-1\", \"https://example.com/redirect-2\"],
            \"response_types\": [\"code\", \"code token id_token\"],
            \"grant_types\": [\"authorization_code\", \"client_credentials\", \"implicit\", \
                \"password\", \"refresh_token\"],
            \"application_type\": \"web\",
            \"contacts\": [\"user@example.com\", \"admin@openidconnect.local\"],
            \"client_name\": \"Example\",
            \"client_name#es\": \"Ejemplo\",
            \"logo_uri\": \"https://example.com/logo.png\",
            \"logo_uri#fr\": \"https://example.com/logo-fr.png\",
            \"client_uri\": \"https://example.com/client-app\",
            \"client_uri#de\": \"https://example.com/client-app-de\",
            \"policy_uri\": \"https://example.com/policy\",
            \"policy_uri#sr-Latn\": \"https://example.com/policy-sr-latin\",
            \"tos_uri\": \"https://example.com/tos\",
            \"tos_uri#sr-Cyrl\": \"https://example.com/tos-sr-cyrl\",
            \"jwks_uri\": \"https://example.com/jwks\",
            \"jwks\": {{\"keys\": [{}]}},
            \"sector_identifier_uri\": \"https://example.com/sector\",
            \"subject_type\": \"pairwise\",
            \"id_token_signed_response_alg\": \"HS256\",
            \"id_token_encrypted_response_alg\": \"RSA1_5\",
            \"id_token_encrypted_response_enc\": \"A128CBC-HS256\",
            \"userinfo_signed_response_alg\": \"RS384\",
            \"userinfo_encrypted_response_alg\": \"RSA-OAEP\",
            \"userinfo_encrypted_response_enc\": \"A256CBC-HS512\",
            \"request_object_signing_alg\": \"ES512\",
            \"request_object_encryption_alg\": \"ECDH-ES+A128KW\",
            \"request_object_encryption_enc\": \"A256GCM\",
            \"token_endpoint_auth_method\": \"client_secret_basic\",
            \"token_endpoint_auth_signing_alg\": \"PS512\",
            \"default_max_age\": 3600,
            \"require_auth_time\": true,
            \"default_acr_values\": [\"0\", \"urn:mace:incommon:iap:silver\", \
                \"urn:mace:incommon:iap:bronze\"],
            \"initiate_login_uri\": \"https://example.com/login\",
            \"request_uris\": [\"https://example.com/request-1\", \"https://example.com/request-2\"]
        }}", TEST_RSA_PUB_KEY);

    let registration_response: CoreClientRegistrationResponse =
        serde_json::from_str(&json_response).unwrap();

    assert_eq!(
        *registration_response.client_id(),
        ClientId::new("abcdefgh".to_string())
    );
    assert_eq!(
        *registration_response.client_secret().unwrap().secret(),
        "shhhh"
    );
    assert_eq!(
        *registration_response
            .registration_access_token()
            .unwrap()
            .secret(),
        "use_me_to_update_registration",
    );
    assert_eq!(
        *registration_response.registration_client_uri().unwrap(),
        ClientConfigUrl::new("https://example-provider.com/registration".to_string()).unwrap()
    );
    assert_eq!(
        registration_response.client_id_issued_at().unwrap(),
        Utc.timestamp_opt(1523953306, 0)
            .single()
            .expect("valid timestamp")
    );
    assert_eq!(
        registration_response.client_secret_expires_at(),
        Some(&ClientSecretExpiration::ExpiresAt(
            Utc.timestamp_opt(1526545306, 0)
                .single()
                .expect("valid timestamp")
        )),
    );
    assert_eq!(
        *registration_response.redirect_uris(),
        vec![
            RedirectUrl::new("https://example.com/redirect-1".to_string()).unwrap(),
            RedirectUrl::new("https://example.com/redirect-2".to_string()).unwrap(),
        ]
    );
    assert_eq!(
        *registration_response.response_types().unwrap(),
        vec![
            ResponseTypes::new(vec![CoreResponseType::Code]),
            ResponseTypes::new(vec![
                CoreResponseType::Code,
                CoreResponseType::Token,
                CoreResponseType::IdToken,
            ]),
        ]
    );
    assert_eq!(
        registration_response.grant_types().unwrap(),
        &vec![
            CoreGrantType::AuthorizationCode,
            CoreGrantType::ClientCredentials,
            CoreGrantType::Implicit,
            CoreGrantType::Password,
            CoreGrantType::RefreshToken,
        ]
    );
    assert_eq!(
        *registration_response.application_type().unwrap(),
        CoreApplicationType::Web
    );
    assert_eq!(
        *registration_response.contacts().unwrap(),
        vec![
            ClientContactEmail::new("user@example.com".to_string()),
            ClientContactEmail::new("admin@openidconnect.local".to_string()),
        ]
    );
    assert_eq!(
        sorted(registration_response.client_name().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, ClientName)>>(),
        vec![
            (None, ClientName::new("Example".to_string())),
            (
                Some(LanguageTag::new("es".to_string())),
                ClientName::new("Ejemplo".to_string()),
            ),
        ]
    );
    assert_eq!(
        sorted(registration_response.logo_uri().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, LogoUrl)>>(),
        vec![
            (
                None,
                LogoUrl::new("https://example.com/logo.png".to_string()).unwrap(),
            ),
            (
                Some(LanguageTag::new("fr".to_string())),
                LogoUrl::new("https://example.com/logo-fr.png".to_string()).unwrap(),
            ),
        ]
    );
    assert_eq!(
        sorted(registration_response.client_uri().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, ClientUrl)>>(),
        vec![
            (
                None,
                ClientUrl::new("https://example.com/client-app".to_string()).unwrap(),
            ),
            (
                Some(LanguageTag::new("de".to_string())),
                ClientUrl::new("https://example.com/client-app-de".to_string()).unwrap(),
            ),
        ]
    );
    assert_eq!(
        sorted(registration_response.policy_uri().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, PolicyUrl)>>(),
        vec![
            (
                None,
                PolicyUrl::new("https://example.com/policy".to_string()).unwrap(),
            ),
            (
                Some(LanguageTag::new("sr-Latn".to_string())),
                PolicyUrl::new("https://example.com/policy-sr-latin".to_string()).unwrap(),
            ),
        ]
    );
    assert_eq!(
        sorted(registration_response.tos_uri().unwrap().clone())
            .collect::<Vec<(Option<LanguageTag>, ToSUrl)>>(),
        vec![
            (
                None,
                ToSUrl::new("https://example.com/tos".to_string()).unwrap(),
            ),
            (
                Some(LanguageTag::new("sr-Cyrl".to_string())),
                ToSUrl::new("https://example.com/tos-sr-cyrl".to_string()).unwrap(),
            ),
        ]
    );
    assert_eq!(
        *registration_response.jwks_uri().unwrap(),
        JsonWebKeySetUrl::new("https://example.com/jwks".to_string()).unwrap()
    );
    assert_eq!(
        registration_response.jwks(),
        Some(&JsonWebKeySet::new(vec![serde_json::from_str(
            TEST_RSA_PUB_KEY
        )
        .unwrap()],)),
    );
    assert_eq!(
        *registration_response.sector_identifier_uri().unwrap(),
        SectorIdentifierUrl::new("https://example.com/sector".to_string()).unwrap()
    );
    assert_eq!(
        *registration_response.subject_type().unwrap(),
        CoreSubjectIdentifierType::Pairwise
    );
    assert_eq!(
        *registration_response
            .id_token_signed_response_alg()
            .unwrap(),
        CoreJwsSigningAlgorithm::HmacSha256
    );
    assert_eq!(
        *registration_response
            .id_token_encrypted_response_alg()
            .unwrap(),
        CoreJweKeyManagementAlgorithm::RsaPkcs1V15
    );
    assert_eq!(
        *registration_response
            .id_token_encrypted_response_enc()
            .unwrap(),
        CoreJweContentEncryptionAlgorithm::Aes128CbcHmacSha256
    );
    assert_eq!(
        *registration_response
            .userinfo_signed_response_alg()
            .unwrap(),
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha384
    );
    assert_eq!(
        *registration_response
            .userinfo_encrypted_response_alg()
            .unwrap(),
        CoreJweKeyManagementAlgorithm::RsaOaep
    );
    assert_eq!(
        *registration_response
            .userinfo_encrypted_response_enc()
            .unwrap(),
        CoreJweContentEncryptionAlgorithm::Aes256CbcHmacSha512
    );
    assert_eq!(
        *registration_response.request_object_signing_alg().unwrap(),
        CoreJwsSigningAlgorithm::EcdsaP521Sha512
    );
    assert_eq!(
        *registration_response
            .request_object_encryption_alg()
            .unwrap(),
        CoreJweKeyManagementAlgorithm::EcdhEsAesKeyWrap128
    );
    assert_eq!(
        *registration_response
            .request_object_encryption_enc()
            .unwrap(),
        CoreJweContentEncryptionAlgorithm::Aes256Gcm
    );
    assert_eq!(
        *registration_response.token_endpoint_auth_method().unwrap(),
        CoreClientAuthMethod::ClientSecretBasic
    );
    assert_eq!(
        *registration_response
            .token_endpoint_auth_signing_alg()
            .unwrap(),
        CoreJwsSigningAlgorithm::RsaSsaPssSha512
    );
    assert_eq!(
        *registration_response.default_max_age().unwrap(),
        Duration::from_secs(3600)
    );
    assert!(registration_response.require_auth_time().unwrap());
    assert_eq!(
        *registration_response.default_acr_values().unwrap(),
        vec![
            AuthenticationContextClass::new("0".to_string()),
            AuthenticationContextClass::new("urn:mace:incommon:iap:silver".to_string()),
            AuthenticationContextClass::new("urn:mace:incommon:iap:bronze".to_string()),
        ]
    );
    assert_eq!(
        *registration_response.sector_identifier_uri().unwrap(),
        SectorIdentifierUrl::new("https://example.com/sector".to_string()).unwrap()
    );
    assert_eq!(
        *registration_response.request_uris().unwrap(),
        vec![
            RequestUrl::new("https://example.com/request-1".to_string()).unwrap(),
            RequestUrl::new("https://example.com/request-2".to_string()).unwrap(),
        ]
    );
    let serialized_json = serde_json::to_string(&registration_response).unwrap();

    let deserialized: CoreClientRegistrationResponse =
        serde_json::from_str(&serialized_json).unwrap();
    assert_eq!(registration_response.client_id, deserialized.client_id);
    assert_eq!(
        registration_response.client_secret.unwrap().secret(),
        deserialized.client_secret.unwrap().secret(),
    );
    assert_eq!(
        registration_response
            .registration_access_token
            .unwrap()
            .secret(),
        deserialized.registration_access_token.unwrap().secret(),
    );
    assert_eq!(
        registration_response.registration_client_uri,
        deserialized.registration_client_uri,
    );
    assert_eq!(
        registration_response.client_id_issued_at,
        deserialized.client_id_issued_at,
    );
    assert_eq!(
        registration_response.client_secret_expires_at,
        deserialized.client_secret_expires_at,
    );
    assert_eq!(
        registration_response.client_metadata,
        deserialized.client_metadata,
    );
    assert_eq!(
        registration_response.additional_response,
        deserialized.additional_response,
    );
}

#[test]
fn test_client_secret_expiration_never_expires() {
    let json_response = r#"{
            "client_id": "abcdefgh",
            "client_secret_expires_at": 0,
            "redirect_uris": ["https://example.com/redirect-1"]
        }"#;

    let registration_response: CoreClientRegistrationResponse =
        serde_json::from_str(json_response).unwrap();

    // The numeric value 0 means the client secret does not expire; it must
    // not surface as an already-expired Unix-epoch timestamp.
    assert_eq!(
        registration_response.client_secret_expires_at(),
        Some(&ClientSecretExpiration::NeverExpires)
    );

    // Round trip: never-expires serializes back to the numeric sentinel 0.
    let serialized_json = serde_json::to_string(&registration_response).unwrap();
    assert!(
        serialized_json.contains(r#""client_secret_expires_at":0"#),
        "unexpected serialization: {}",
        serialized_json
    );
    let deserialized: CoreClientRegistrationResponse =
        serde_json::from_str(&serialized_json).unwrap();
    assert_eq!(
        deserialized.client_secret_expires_at(),
        Some(&ClientSecretExpiration::NeverExpires)
    );
}

#[test]
fn test_client_secret_expiration_expires_at() {
    let json_response = r#"{
            "client_id": "abcdefgh",
            "client_secret_expires_at": 1526545306,
            "redirect_uris": ["https://example.com/redirect-1"]
        }"#;

    let registration_response: CoreClientRegistrationResponse =
        serde_json::from_str(json_response).unwrap();

    // Any other numeric timestamp is a real expiry, keeping the shared
    // Timestamp adapter's round-down semantics.
    let expected = ClientSecretExpiration::ExpiresAt(
        Utc.timestamp_opt(1526545306, 0)
            .single()
            .expect("valid timestamp"),
    );
    assert_eq!(
        registration_response.client_secret_expires_at(),
        Some(&expected)
    );

    // Round trip: a real expiry serializes back to its timestamp in seconds.
    let serialized_json = serde_json::to_string(&registration_response).unwrap();
    assert!(
        serialized_json.contains(r#""client_secret_expires_at":1526545306"#),
        "unexpected serialization: {}",
        serialized_json
    );
    let deserialized: CoreClientRegistrationResponse =
        serde_json::from_str(&serialized_json).unwrap();
    assert_eq!(deserialized.client_secret_expires_at(), Some(&expected));
}

#[test]
fn test_client_secret_expiration_absent() {
    let json_response = r#"{
            "client_id": "abcdefgh",
            "redirect_uris": ["https://example.com/redirect-1"]
        }"#;

    let registration_response: CoreClientRegistrationResponse =
        serde_json::from_str(json_response).unwrap();

    // A missing field stays absent: None is distinct from never-expires.
    assert_eq!(registration_response.client_secret_expires_at(), None);

    // Round trip: an absent expiry is omitted on serialization.
    let serialized_json = serde_json::to_string(&registration_response).unwrap();
    assert!(
        !serialized_json.contains("client_secret_expires_at"),
        "absent expiry must stay omitted: {}",
        serialized_json
    );
    let deserialized: CoreClientRegistrationResponse =
        serde_json::from_str(&serialized_json).unwrap();
    assert_eq!(deserialized.client_secret_expires_at(), None);
}

#[test]
fn test_client_secret_expiration_setter() {
    use crate::registration::{
        EmptyAdditionalClientMetadata, EmptyAdditionalClientRegistrationResponse,
    };

    // The setter accepts both states, and a never-expires value is stored as
    // never-expiring rather than an already-expired epoch timestamp.
    let registration_response = CoreClientRegistrationResponse::new(
        ClientId::new("abcdefgh".to_string()),
        vec![RedirectUrl::new("https://example.com/redirect-1".to_string()).unwrap()],
        EmptyAdditionalClientMetadata {},
        EmptyAdditionalClientRegistrationResponse {},
    )
    .set_client_secret_expires_at(Some(ClientSecretExpiration::NeverExpires));
    assert_eq!(
        registration_response.client_secret_expires_at(),
        Some(&ClientSecretExpiration::NeverExpires)
    );
    let serialized_json = serde_json::to_string(&registration_response).unwrap();
    assert!(
        serialized_json.contains(r#""client_secret_expires_at":0"#),
        "unexpected serialization: {}",
        serialized_json
    );

    let registration_response = registration_response.set_client_secret_expires_at(Some(
        ClientSecretExpiration::ExpiresAt(
            Utc.timestamp_opt(1526545306, 0)
                .single()
                .expect("valid timestamp"),
        ),
    ));
    assert_eq!(
        registration_response.client_secret_expires_at(),
        Some(&ClientSecretExpiration::ExpiresAt(
            Utc.timestamp_opt(1526545306, 0)
                .single()
                .expect("valid timestamp")
        ))
    );
    let serialized_json = serde_json::to_string(&registration_response).unwrap();
    assert!(
        serialized_json.contains(r#""client_secret_expires_at":1526545306"#),
        "unexpected serialization: {}",
        serialized_json
    );
}

#[derive(Debug)]
struct MockHttpClientError;

impl std::fmt::Display for MockHttpClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("mock http client error")
    }
}

impl std::error::Error for MockHttpClientError {}

use std::future::Future;

/// Returns an HTTP client that records each dispatched request and answers with the given
/// response parts.
fn recording_client(
    dispatched: std::rc::Rc<std::cell::RefCell<Vec<crate::HttpRequest>>>,
    status: http::StatusCode,
    content_type: &'static str,
    body: &'static str,
) -> impl Fn(crate::HttpRequest) -> Result<crate::HttpResponse, MockHttpClientError> {
    move |request| {
        dispatched.borrow_mut().push(request);
        Ok(http::Response::builder()
            .status(status)
            .header(http::header::CONTENT_TYPE, content_type)
            .body(body.as_bytes().to_vec())
            .unwrap())
    }
}

/// Returns an asynchronous HTTP client with the same recording behavior, shaped for the
/// `AsyncHttpClient` blanket impl.
fn recording_async_client(
    dispatched: std::rc::Rc<std::cell::RefCell<Vec<crate::HttpRequest>>>,
) -> impl Fn(
    crate::HttpRequest,
)
    -> std::pin::Pin<Box<dyn Future<Output = Result<crate::HttpResponse, MockHttpClientError>>>> {
    move |request| {
        let dispatched = dispatched.clone();
        Box::pin(async move {
            dispatched.borrow_mut().push(request);
            Ok(http::Response::builder()
                .status(http::StatusCode::CREATED)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(
                    "{\"client_id\":\"my_client\",\"redirect_uris\":[\"https://client.example.com/cb\"]}"
                        .as_bytes()
                        .to_vec(),
                )
                .unwrap())
        })
    }
}

/// A provider-issued initial access token containing a control character fails registration
/// request preparation without panicking, without echoing the token bytes, and without
/// dispatching any HTTP call.
#[test]
fn test_registration_malformed_initial_access_token_errors_before_dispatch() {
    use crate::core::CoreClientRegistrationRequest;
    use crate::registration::{ClientRegistrationError, EmptyAdditionalClientMetadata};
    use crate::{AccessToken, RegistrationUrl};

    let request = CoreClientRegistrationRequest::new(
        vec![RedirectUrl::new("https://client.example.com/cb".to_string()).unwrap()],
        EmptyAdditionalClientMetadata {},
    )
    .set_initial_access_token(Some(AccessToken::new("the_access\ntoken".to_string())));
    let registration_url =
        RegistrationUrl::new("https://server.example.com/register".to_string()).unwrap();

    let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let http_client = recording_client(
        dispatched.clone(),
        http::StatusCode::CREATED,
        "application/json",
        "{}",
    );

    match request.register(&registration_url, &http_client) {
        Err(ClientRegistrationError::Other(message)) => {
            assert!(
                message.contains("failed to prepare request"),
                "unexpected error message: {message}"
            );
            assert!(
                !message.contains("the_access"),
                "error must not echo the access token: {message}"
            );
        }
        other => panic!("expected a request-preparation error, got: {other:?}"),
    }
    assert!(
        dispatched.borrow().is_empty(),
        "no HTTP call should be dispatched"
    );
}

/// The asynchronous registration path surfaces the same preparation failure without
/// dispatching: request preparation runs before the first await, so polling once with a no-op
/// waker (no async executor in the dev-dependencies) returns the ready error.
#[test]
fn test_registration_async_malformed_initial_access_token_errors_before_dispatch() {
    use crate::core::CoreClientRegistrationRequest;
    use crate::registration::{ClientRegistrationError, EmptyAdditionalClientMetadata};
    use crate::{AccessToken, RegistrationUrl};

    let request = CoreClientRegistrationRequest::new(
        vec![RedirectUrl::new("https://client.example.com/cb".to_string()).unwrap()],
        EmptyAdditionalClientMetadata {},
    )
    .set_initial_access_token(Some(AccessToken::new("the_access\ntoken".to_string())));
    let registration_url =
        RegistrationUrl::new("https://server.example.com/register".to_string()).unwrap();

    let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let http_client = recording_async_client(dispatched.clone());

    let mut future = Box::pin(request.register_async(&registration_url, &http_client));
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    match future.as_mut().poll(&mut cx) {
        std::task::Poll::Ready(Err(ClientRegistrationError::Other(message))) => {
            assert!(
                message.contains("failed to prepare request"),
                "unexpected error message: {message}"
            );
            assert!(
                !message.contains("the_access"),
                "error must not echo the access token: {message}"
            );
        }
        other => panic!("expected a ready request-preparation error, got: {other:?}"),
    }
    assert!(
        dispatched.borrow().is_empty(),
        "no HTTP call should be dispatched"
    );
}

/// A valid initial access token still produces the expected `Authorization: Bearer` header on
/// the dispatched registration request, and the response Content-Type check accepts RFC 7231
/// optional whitespace and case variance.
#[test]
fn test_registration_sends_bearer_header_and_accepts_ows_content_type() {
    use crate::core::CoreClientRegistrationRequest;
    use crate::registration::EmptyAdditionalClientMetadata;
    use crate::{AccessToken, ClientId, RegistrationUrl};
    use http::header::HeaderValue;

    let request = CoreClientRegistrationRequest::new(
        vec![RedirectUrl::new("https://client.example.com/cb".to_string()).unwrap()],
        EmptyAdditionalClientMetadata {},
    )
    .set_initial_access_token(Some(AccessToken::new("the_access_token".to_string())));
    let registration_url =
        RegistrationUrl::new("https://server.example.com/register".to_string()).unwrap();

    let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let http_client = recording_client(
        dispatched.clone(),
        http::StatusCode::CREATED,
        "APPLICATION/JSON ; charset=UTF-8",
        "{\"client_id\":\"my_client\",\"redirect_uris\":[\"https://client.example.com/cb\"]}",
    );

    let response = request
        .register(&registration_url, &http_client)
        .expect("registration should succeed");
    assert_eq!(
        *response.client_id(),
        ClientId::new("my_client".to_string())
    );

    let dispatched = dispatched.borrow();
    assert_eq!(dispatched.len(), 1);
    assert_eq!(
        dispatched[0].headers().get(http::header::AUTHORIZATION),
        Some(&HeaderValue::from_static("Bearer the_access_token")),
    );
}

/// Registering without an initial access token sends no Authorization header.
#[test]
fn test_registration_without_access_token_sends_no_authorization_header() {
    use crate::core::CoreClientRegistrationRequest;
    use crate::registration::EmptyAdditionalClientMetadata;
    use crate::RegistrationUrl;

    let request = CoreClientRegistrationRequest::new(
        vec![RedirectUrl::new("https://client.example.com/cb".to_string()).unwrap()],
        EmptyAdditionalClientMetadata {},
    );
    let registration_url =
        RegistrationUrl::new("https://server.example.com/register".to_string()).unwrap();

    let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let http_client = recording_client(
        dispatched.clone(),
        http::StatusCode::CREATED,
        "application/json",
        "{\"client_id\":\"my_client\",\"redirect_uris\":[\"https://client.example.com/cb\"]}",
    );

    request
        .register(&registration_url, &http_client)
        .expect("registration should succeed");

    let dispatched = dispatched.borrow();
    assert_eq!(dispatched.len(), 1);
    assert!(
        dispatched[0]
            .headers()
            .get(http::header::AUTHORIZATION)
            .is_none(),
        "no Authorization header should be sent without an initial access token"
    );
}
