use crate::helpers::{deserialize_string_or_vec_opt, FilteredFlatten};
use crate::http_utils::{auth_bearer, content_type_has_essence, MIME_TYPE_JSON, MIME_TYPE_JWT};
use crate::jwt::{JsonWebTokenError, JsonWebTokenJsonPayloadSerde};
use crate::verification::UserInfoVerifier;
use crate::{
    AccessToken, AdditionalClaims, AddressClaim, AsyncHttpClient, Audience, AudiencesClaim,
    AuthDisplay, AuthPrompt, ClaimsVerificationError, Client, ClientSecret, EndUserBirthday,
    EndUserEmail, EndUserFamilyName, EndUserGivenName, EndUserMiddleName, EndUserName,
    EndUserNickname, EndUserPhoneNumber, EndUserPictureUrl, EndUserProfileUrl, EndUserTimezone,
    EndUserUsername, EndUserWebsiteUrl, EndpointState, ErrorResponse, GenderClaim, HttpRequest,
    HttpResponse, IssuerClaim, IssuerUrl, JsonWebKey, JsonWebToken, JweContentEncryptionAlgorithm,
    JwsSigningAlgorithm, LanguageTag, LocalizedClaim, PrivateSigningKey, RevocableToken,
    SignatureVerificationError, StandardClaims, SubjectIdentifier, SyncHttpClient,
    TokenIntrospectionResponse, TokenResponse,
};

use chrono::{DateTime, Utc};
use http::header::{HeaderValue, ACCEPT, CONTENT_TYPE};
use http::method::Method;
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use thiserror::Error;

use std::future::Future;
use std::str;

impl<
        AC,
        AD,
        GC,
        JE,
        K,
        P,
        TE,
        TR,
        TIR,
        RT,
        TRE,
        HasAuthUrl,
        HasDeviceAuthUrl,
        HasIntrospectionUrl,
        HasRevocationUrl,
        HasTokenUrl,
        HasUserInfoUrl,
    >
    Client<
        AC,
        AD,
        GC,
        JE,
        K,
        P,
        TE,
        TR,
        TIR,
        RT,
        TRE,
        HasAuthUrl,
        HasDeviceAuthUrl,
        HasIntrospectionUrl,
        HasRevocationUrl,
        HasTokenUrl,
        HasUserInfoUrl,
    >
where
    AC: AdditionalClaims,
    AD: AuthDisplay,
    GC: GenderClaim,
    JE: JweContentEncryptionAlgorithm<
        KeyType = <K::SigningAlgorithm as JwsSigningAlgorithm>::KeyType,
    >,
    K: JsonWebKey,
    P: AuthPrompt,
    TE: ErrorResponse + 'static,
    TR: TokenResponse<AC, GC, JE, K::SigningAlgorithm>,
    TIR: TokenIntrospectionResponse,
    RT: RevocableToken,
    TRE: ErrorResponse + 'static,
    HasAuthUrl: EndpointState,
    HasDeviceAuthUrl: EndpointState,
    HasIntrospectionUrl: EndpointState,
    HasRevocationUrl: EndpointState,
    HasTokenUrl: EndpointState,
    HasUserInfoUrl: EndpointState,
{
    pub(crate) fn user_info_impl<'a>(
        &'a self,
        userinfo_endpoint: &'a UserInfoUrl,
        access_token: AccessToken,
        expected_subject: Option<SubjectIdentifier>,
    ) -> UserInfoRequest<'a, JE, K> {
        UserInfoRequest {
            url: userinfo_endpoint,
            access_token,
            require_signed_response: false,
            response_type: UserInfoResponseType::Json,
            // Route the signed-response verifier confidentially when this client holds a secret,
            // exactly like `Client::id_token_verifier`. The algorithm allowlist stays the
            // RS256-only default; consumers opt in to other algorithms via
            // `UserInfoRequest::set_allowed_algs`.
            signed_response_verifier: if let Some(client_secret) = self.client_secret() {
                UserInfoVerifier::new_confidential_client(
                    self.client_id.clone(),
                    client_secret.clone(),
                    self.issuer.clone(),
                    self.jwks.clone(),
                    expected_subject,
                )
            } else {
                UserInfoVerifier::new(
                    self.client_id.clone(),
                    self.issuer.clone(),
                    self.jwks.clone(),
                    expected_subject,
                )
            },
        }
    }
}

/// User info request.
pub struct UserInfoRequest<'a, JE, K>
where
    JE: JweContentEncryptionAlgorithm<
        KeyType = <K::SigningAlgorithm as JwsSigningAlgorithm>::KeyType,
    >,
    K: JsonWebKey,
{
    pub(crate) url: &'a UserInfoUrl,
    pub(crate) access_token: AccessToken,
    pub(crate) require_signed_response: bool,
    pub(crate) signed_response_verifier: UserInfoVerifier<'static, JE, K>,
    pub(crate) response_type: UserInfoResponseType,
}
impl<'a, JE, K> UserInfoRequest<'a, JE, K>
where
    JE: JweContentEncryptionAlgorithm<
        KeyType = <K::SigningAlgorithm as JwsSigningAlgorithm>::KeyType,
    >,
    K: JsonWebKey,
{
    /// Submits this request to the associated user info endpoint using the specified synchronous
    /// HTTP client.
    pub fn request<AC, GC, C>(
        self,
        http_client: &C,
    ) -> Result<UserInfoClaims<AC, GC>, UserInfoError<<C as SyncHttpClient>::Error>>
    where
        AC: AdditionalClaims,
        GC: GenderClaim,
        C: SyncHttpClient,
    {
        http_client
            .call(
                self.prepare_request().map_err(|err| {
                    UserInfoError::Other(format!("failed to prepare request: {err}"))
                })?,
            )
            .map_err(UserInfoError::Request)
            .and_then(|http_response| self.user_info_response(http_response))
    }

    /// Submits this request to the associated user info endpoint using the specified asynchronous
    /// HTTP client.
    pub fn request_async<'c, AC, C, GC>(
        self,
        http_client: &'c C,
    ) -> impl Future<
        Output = Result<UserInfoClaims<AC, GC>, UserInfoError<<C as AsyncHttpClient<'c>>::Error>>,
    > + 'c
    where
        Self: 'c,
        AC: AdditionalClaims,
        C: AsyncHttpClient<'c>,
        GC: GenderClaim,
    {
        Box::pin(async move {
            let http_response = http_client
                .call(self.prepare_request().map_err(|err| {
                    UserInfoError::Other(format!("failed to prepare request: {err}"))
                })?)
                .await
                .map_err(UserInfoError::Request)?;

            self.user_info_response(http_response)
        })
    }

    fn prepare_request(&self) -> Result<HttpRequest, http::Error> {
        let (auth_header, auth_value) =
            auth_bearer(&self.access_token).map_err(http::Error::from)?;
        let accept_value = match self.response_type {
            UserInfoResponseType::Jwt => MIME_TYPE_JWT,
            _ => MIME_TYPE_JSON,
        };

        http::Request::builder()
            .uri(self.url.to_string())
            .method(Method::GET)
            .header(ACCEPT, HeaderValue::from_static(accept_value))
            .header(auth_header, auth_value)
            .body(Vec::new())
    }

    fn user_info_response<AC, GC, RE>(
        self,
        http_response: HttpResponse,
    ) -> Result<UserInfoClaims<AC, GC>, UserInfoError<RE>>
    where
        AC: AdditionalClaims,
        GC: GenderClaim,
        RE: std::error::Error + 'static,
    {
        if http_response.status() != StatusCode::OK {
            return Err(UserInfoError::Response(
                http_response.status(),
                http_response.body().to_owned(),
                "unexpected HTTP status code".to_string(),
            ));
        }

        match http_response
            .headers()
            .get(CONTENT_TYPE)
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| HeaderValue::from_static(MIME_TYPE_JSON))
        {
            ref content_type if content_type_has_essence(content_type, MIME_TYPE_JSON) => {
                if self.require_signed_response {
                    return Err(UserInfoError::ClaimsVerification(
                        ClaimsVerificationError::SignatureVerification(
                            SignatureVerificationError::NoSignature,
                        ),
                    ));
                }
                UserInfoClaims::from_json(
                    http_response.body(),
                    self.signed_response_verifier.expected_subject(),
                )
            }
            ref content_type if content_type_has_essence(content_type, MIME_TYPE_JWT) => {
                let jwt_str = String::from_utf8(http_response.body().to_owned()).map_err(|_| {
                    UserInfoError::Other("response body has invalid UTF-8 encoding".to_string())
                })?;
                serde_path_to_error::deserialize::<
                    _,
                    UserInfoJsonWebToken<AC, GC, JE, K::SigningAlgorithm>,
                >(serde_json::Value::String(jwt_str))
                .map_err(UserInfoError::Parse)?
                .claims(&self.signed_response_verifier)
                .map_err(UserInfoError::ClaimsVerification)
            }
            ref content_type => Err(UserInfoError::Response(
                http_response.status(),
                http_response.body().to_owned(),
                format!("unexpected response Content-Type: `{:?}`", content_type),
            )),
        }
    }

    /// Specifies whether to require the user info response to be a signed JSON Web Token (JWT).
    pub fn require_signed_response(mut self, require_signed_response: bool) -> Self {
        self.require_signed_response = require_signed_response;
        self
    }

    /// Specifies whether to require the issuer of the signed JWT response to match the expected
    /// issuer URL for this provider.
    ///
    /// This option has no effect on unsigned JSON responses.
    pub fn require_issuer_match(mut self, iss_required: bool) -> Self {
        self.signed_response_verifier = self
            .signed_response_verifier
            .require_issuer_match(iss_required);
        self
    }

    /// Specifies whether to require the audience of the signed JWT response to match the expected
    /// audience (client ID).
    ///
    /// This option has no effect on unsigned JSON responses.
    pub fn require_audience_match(mut self, aud_required: bool) -> Self {
        self.signed_response_verifier = self
            .signed_response_verifier
            .require_audience_match(aud_required);
        self
    }

    /// Specifies which JSON Web Signature algorithms are supported for the signed JWT response.
    ///
    /// The default allowlist contains only `RS256`. Verifying responses signed with `ES256`,
    /// `ES384`, or an `HS*` algorithm requires explicitly allowing those algorithms here;
    /// `HS*` algorithms additionally require a confidential verifier
    /// ([`set_client_secret`](Self::set_client_secret) or a confidential
    /// [`Client`](crate::Client)).
    ///
    /// This option has no effect on unsigned JSON responses.
    pub fn set_allowed_algs<I>(mut self, algs: I) -> Self
    where
        I: IntoIterator<Item = K::SigningAlgorithm>,
    {
        self.signed_response_verifier = self.signed_response_verifier.set_allowed_algs(algs);
        self
    }

    /// Specifies the client secret used to verify a signed JWT response that uses a shared
    /// secret algorithm such as `HS256`, `HS384`, or `HS512`, replacing any secret inherited
    /// from the [`Client`](crate::Client).
    ///
    /// For these algorithms, the octets of the UTF-8 representation of the client secret are
    /// used as the key to validate the signature.
    ///
    /// This option has no effect on unsigned JSON responses.
    pub fn set_client_secret(mut self, client_secret: ClientSecret) -> Self {
        self.signed_response_verifier = self
            .signed_response_verifier
            .set_client_secret(client_secret);
        self
    }

    /// Specifies the expected response type by setting the `Accept` header. Note that the server can ignore this header.
    pub fn set_response_type(mut self, response_type: UserInfoResponseType) -> Self {
        self.response_type = response_type;
        self
    }
}

/// User info claims.
#[derive(Clone, Debug, Serialize)]
pub struct UserInfoClaims<AC: AdditionalClaims, GC: GenderClaim>(UserInfoClaimsImpl<AC, GC>);
impl<AC, GC> UserInfoClaims<AC, GC>
where
    AC: AdditionalClaims,
    GC: GenderClaim,
{
    /// Initializes user info claims.
    pub fn new(standard_claims: StandardClaims<GC>, additional_claims: AC) -> Self {
        Self(UserInfoClaimsImpl {
            issuer: None,
            audiences: None,
            standard_claims,
            additional_claims: additional_claims.into(),
        })
    }

    /// Initializes user info claims from the provided raw JSON response.
    ///
    /// If an `expected_subject` is provided, this function verifies that the user info claims
    /// contain the expected subject and returns an error otherwise.
    pub fn from_json<RE>(
        user_info_json: &[u8],
        expected_subject: Option<&SubjectIdentifier>,
    ) -> Result<Self, UserInfoError<RE>>
    where
        RE: std::error::Error + 'static,
    {
        let user_info = serde_path_to_error::deserialize::<_, UserInfoClaimsImpl<AC, GC>>(
            &mut serde_json::Deserializer::from_slice(user_info_json),
        )
        .map_err(UserInfoError::Parse)?;

        // This is the only verification we need to do for JSON-based user info claims, so don't
        // bother with the complexity of a separate verifier object.
        if expected_subject
            .iter()
            .all(|expected_subject| user_info.standard_claims.sub == **expected_subject)
        {
            Ok(Self(user_info))
        } else {
            Err(UserInfoError::ClaimsVerification(
                ClaimsVerificationError::InvalidSubject(format!(
                    "expected `{}` (found `{}`)",
                    // This can only happen when expected_subject is not None.
                    expected_subject.unwrap().as_str(),
                    user_info.standard_claims.sub.as_str(),
                )),
            ))
        }
    }

    field_getters_setters![
        pub self [self.0] ["claim"] {
            set_issuer -> issuer[Option<IssuerUrl>],
            set_audiences -> audiences[Option<Vec<Audience>>] ["aud"],
        }
    ];

    /// Returns the `sub` claim.
    pub fn subject(&self) -> &SubjectIdentifier {
        &self.0.standard_claims.sub
    }
    /// Sets the `sub` claim.
    pub fn set_subject(&mut self, subject: SubjectIdentifier) {
        self.0.standard_claims.sub = subject
    }

    field_getters_setters![
        pub self [self.0.standard_claims] ["claim"] {
            set_name -> name[Option<LocalizedClaim<EndUserName>>],
            set_given_name -> given_name[Option<LocalizedClaim<EndUserGivenName>>],
            set_family_name ->
                family_name[Option<LocalizedClaim<EndUserFamilyName>>],
            set_middle_name ->
                middle_name[Option<LocalizedClaim<EndUserMiddleName>>],
            set_nickname -> nickname[Option<LocalizedClaim<EndUserNickname>>],
            set_preferred_username -> preferred_username[Option<EndUserUsername>],
            set_profile -> profile[Option<LocalizedClaim<EndUserProfileUrl>>],
            set_picture -> picture[Option<LocalizedClaim<EndUserPictureUrl>>],
            set_website -> website[Option<LocalizedClaim<EndUserWebsiteUrl>>],
            set_email -> email[Option<EndUserEmail>],
            set_email_verified -> email_verified[Option<bool>],
            set_gender -> gender[Option<GC>],
            set_birthday -> birthday[Option<EndUserBirthday>],
            set_birthdate -> birthdate[Option<EndUserBirthday>],
            set_zoneinfo -> zoneinfo[Option<EndUserTimezone>],
            set_locale -> locale[Option<LanguageTag>],
            set_phone_number -> phone_number[Option<EndUserPhoneNumber>],
            set_phone_number_verified -> phone_number_verified[Option<bool>],
            set_address -> address[Option<AddressClaim>],
            set_updated_at -> updated_at[Option<DateTime<Utc>>],
        }
    ];

    /// Returns the standard claims as a `StandardClaims` object.
    pub fn standard_claims(&self) -> &StandardClaims<GC> {
        &self.0.standard_claims
    }

    /// Returns additional user info claims.
    pub fn additional_claims(&self) -> &AC {
        self.0.additional_claims.as_ref()
    }
    /// Returns mutable additional user info claims.
    pub fn additional_claims_mut(&mut self) -> &mut AC {
        self.0.additional_claims.as_mut()
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct UserInfoClaimsImpl<AC, GC>
where
    AC: AdditionalClaims,
    GC: GenderClaim,
{
    #[serde(rename = "iss")]
    pub issuer: Option<IssuerUrl>,
    // We always serialize as an array, which is valid according to the spec.
    #[serde(
        default,
        rename = "aud",
        deserialize_with = "deserialize_string_or_vec_opt"
    )]
    pub audiences: Option<Vec<Audience>>,

    #[serde(bound = "GC: GenderClaim", flatten)]
    pub standard_claims: StandardClaims<GC>,

    #[serde(bound = "AC: AdditionalClaims", flatten)]
    pub additional_claims: FilteredFlatten<StandardClaims<GC>, AC>,
}
impl<AC, GC> AudiencesClaim for UserInfoClaimsImpl<AC, GC>
where
    AC: AdditionalClaims,
    GC: GenderClaim,
{
    fn audiences(&self) -> Option<&Vec<Audience>> {
        self.audiences.as_ref()
    }
}
impl<AC, GC> AudiencesClaim for &UserInfoClaimsImpl<AC, GC>
where
    AC: AdditionalClaims,
    GC: GenderClaim,
{
    fn audiences(&self) -> Option<&Vec<Audience>> {
        self.audiences.as_ref()
    }
}

impl<AC, GC> IssuerClaim for UserInfoClaimsImpl<AC, GC>
where
    AC: AdditionalClaims,
    GC: GenderClaim,
{
    fn issuer(&self) -> Option<&IssuerUrl> {
        self.issuer.as_ref()
    }
}
impl<AC, GC> IssuerClaim for &UserInfoClaimsImpl<AC, GC>
where
    AC: AdditionalClaims,
    GC: GenderClaim,
{
    fn issuer(&self) -> Option<&IssuerUrl> {
        self.issuer.as_ref()
    }
}

/// JSON Web Token (JWT) containing user info claims.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserInfoJsonWebToken<
    AC: AdditionalClaims,
    GC: GenderClaim,
    JE: JweContentEncryptionAlgorithm<KeyType = JS::KeyType>,
    JS: JwsSigningAlgorithm,
>(
    #[serde(bound = "AC: AdditionalClaims")]
    JsonWebToken<JE, JS, UserInfoClaimsImpl<AC, GC>, JsonWebTokenJsonPayloadSerde>,
);
impl<AC, GC, JE, JS> UserInfoJsonWebToken<AC, GC, JE, JS>
where
    AC: AdditionalClaims,
    GC: GenderClaim,
    JE: JweContentEncryptionAlgorithm<KeyType = JS::KeyType>,
    JS: JwsSigningAlgorithm,
{
    /// Initializes a new signed JWT containing the specified claims, signed with the specified key
    /// and signing algorithm.
    pub fn new<S>(
        claims: UserInfoClaims<AC, GC>,
        signing_key: &S,
        alg: JS,
    ) -> Result<Self, JsonWebTokenError>
    where
        S: PrivateSigningKey,
        <S as PrivateSigningKey>::VerificationKey: JsonWebKey<SigningAlgorithm = JS>,
    {
        Ok(Self(JsonWebToken::new(claims.0, signing_key, &alg)?))
    }

    /// Verifies and returns the user info claims.
    pub fn claims<K>(
        self,
        verifier: &UserInfoVerifier<JE, K>,
    ) -> Result<UserInfoClaims<AC, GC>, ClaimsVerificationError>
    where
        K: JsonWebKey<SigningAlgorithm = JS>,
    {
        Ok(UserInfoClaims(verifier.verified_claims(self.0)?))
    }
}

new_url_type![
    /// URL for a provider's user info endpoint.
    UserInfoUrl
];

/// Indicates via the `Accept` header the body response type the server should use to return the user info. Note that the server can ignore this header.
///
/// Defaults to Json.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UserInfoResponseType {
    /// Sets the `Accept` header to `application/json`.
    Json,
    /// Sets the `Accept` header to `application/jwt`.
    Jwt,
}

/// Error retrieving user info.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum UserInfoError<RE>
where
    RE: std::error::Error + 'static,
{
    /// Failed to verify user info claims.
    #[error("Failed to verify claims")]
    ClaimsVerification(#[source] ClaimsVerificationError),
    /// Failed to parse server response.
    #[error("Failed to parse server response")]
    Parse(#[source] serde_path_to_error::Error<serde_json::Error>),
    /// An error occurred while sending the request or receiving the response (e.g., network
    /// connectivity failed).
    #[error("Request failed")]
    Request(#[source] RE),
    /// Server returned an invalid response.
    #[error("Server returned invalid response: {2}")]
    Response(StatusCode, Vec<u8>, String),
    /// An unexpected error occurred.
    #[error("Other error: {0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use crate::core::CoreGenderClaim;
    use crate::{AdditionalClaims, UserInfoClaims};

    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    use std::future::Future;

    use crate::{HttpRequest, HttpResponse};

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    struct TestClaims {
        pub tfa_method: String,
    }
    impl AdditionalClaims for TestClaims {}

    #[test]
    fn test_additional_claims() {
        let claims =
            UserInfoClaims::<TestClaims, CoreGenderClaim>::from_json::<crate::reqwest::Error>(
                "{
                \"iss\": \"https://server.example.com\",
                \"sub\": \"24400320\",
                \"aud\": [\"s6BhdRkqt3\"],
                \"tfa_method\": \"u2f\"
            }"
                .as_bytes(),
                None,
            )
            .expect("failed to deserialize");
        assert_eq!(claims.additional_claims().tfa_method, "u2f");
        assert_eq!(
            serde_json::to_string(&claims).expect("failed to serialize"),
            "{\
             \"iss\":\"https://server.example.com\",\
             \"aud\":[\"s6BhdRkqt3\"],\
             \"sub\":\"24400320\",\
             \"tfa_method\":\"u2f\"\
             }",
        );

        UserInfoClaims::<TestClaims, CoreGenderClaim>::from_json::<crate::reqwest::Error>(
            "{
                \"iss\": \"https://server.example.com\",
                \"sub\": \"24400320\",
                \"aud\": [\"s6BhdRkqt3\"]
            }"
            .as_bytes(),
            None,
        )
        .expect_err("missing claim should fail to deserialize");
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct AllOtherClaims(HashMap<String, serde_json::Value>);
    impl AdditionalClaims for AllOtherClaims {}

    #[test]
    fn test_catch_all_additional_claims() {
        let claims =
            UserInfoClaims::<AllOtherClaims, CoreGenderClaim>::from_json::<crate::reqwest::Error>(
                "{
                \"iss\": \"https://server.example.com\",
                \"sub\": \"24400320\",
                \"aud\": [\"s6BhdRkqt3\"],
                \"tfa_method\": \"u2f\",
                \"updated_at\": 1000
            }"
                .as_bytes(),
                None,
            )
            .expect("failed to deserialize");

        assert_eq!(claims.additional_claims().0.len(), 1);
        assert_eq!(claims.additional_claims().0["tfa_method"], "u2f");
    }

    /// The `UserInfoRequest` builders configure the embedded signed-response verifier: an
    /// explicitly allowed shared-secret algorithm verifies through the supplied client secret,
    /// and stays rejected without it.
    #[test]
    fn test_user_info_request_signed_response_policy() {
        use crate::core::{
            CoreHmacKey, CoreJsonWebKey, CoreJsonWebKeySet, CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm, CoreUserInfoVerifier,
        };
        use crate::{
            AccessToken, ClaimsVerificationError, ClientId, ClientSecret, EmptyAdditionalClaims,
            IssuerUrl, PrivateSigningKey, SignatureVerificationError, SubjectIdentifier,
            UserInfoError, UserInfoRequest, UserInfoResponseType,
        };
        use base64::Engine;

        let client_id = ClientId::new("my_client".to_string());
        let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
        let sub = SubjectIdentifier::new("the_subject".to_string());

        let b64 = crate::core::base64_url_safe_no_pad();
        let payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"the_subject\",\"name\":\"Jane Doe\"}";
        let sign_hs = |alg: &CoreJwsSigningAlgorithm, secret: &str| -> String {
            let alg_name =
                serde_plain::to_string(alg).expect("alg should serialize to its JOSE name");
            let header = format!("{{\"alg\":\"{alg_name}\",\"typ\":\"JWT\"}}");
            let signing_input = format!(
                "{}.{}",
                b64.encode(header.as_bytes()),
                b64.encode(payload.as_bytes())
            );
            let hmac_key = CoreHmacKey::new(secret);
            let signature = hmac_key
                .sign(&alg, signing_input.as_bytes())
                .expect("HS signing should succeed");
            format!("{}.{}", signing_input, b64.encode(signature))
        };
        let token = sign_hs(&CoreJwsSigningAlgorithm::HmacSha256, "the_client_secret");

        let url = crate::UserInfoUrl::new("https://example.com/userinfo".to_string()).unwrap();
        let make_request = || UserInfoRequest::<CoreJweContentEncryptionAlgorithm, CoreJsonWebKey> {
            url: &url,
            access_token: AccessToken::new("the_access_token".to_string()),
            require_signed_response: false,
            response_type: UserInfoResponseType::Jwt,
            signed_response_verifier: CoreUserInfoVerifier::new(
                client_id.clone(),
                issuer.clone(),
                CoreJsonWebKeySet::new(vec![]),
                Some(sub.clone()),
            ),
        };
        let jwt_response = |body: String| {
            http::Response::builder()
                .status(http::StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/jwt")
                .body(body.into_bytes())
                .unwrap()
        };

        // The builders reach the embedded verifier: HS256 verifies with the supplied client
        // secret.
        let claims = make_request()
            .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256])
            .set_client_secret(ClientSecret::new("the_client_secret".to_string()))
            .user_info_response::<EmptyAdditionalClaims, CoreGenderClaim, crate::reqwest::Error>(
                jwt_response(token.clone()),
            )
            .expect("verification should succeed");
        assert_eq!(*claims.subject(), sub);

        // Without the client secret, the allowed shared-secret algorithm is still rejected.
        match make_request()
            .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256])
            .user_info_response::<EmptyAdditionalClaims, CoreGenderClaim, crate::reqwest::Error>(
                jwt_response(token.clone()),
            ) {
            Err(UserInfoError::ClaimsVerification(
                ClaimsVerificationError::SignatureVerification(
                    SignatureVerificationError::DisallowedAlg(_),
                ),
            )) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        // Without an explicit allowlist, the RS256-only default rejects HS256 even with the
        // client secret present.
        match make_request()
            .set_client_secret(ClientSecret::new("the_client_secret".to_string()))
            .user_info_response::<EmptyAdditionalClaims, CoreGenderClaim, crate::reqwest::Error>(
                jwt_response(token),
            ) {
            Err(UserInfoError::ClaimsVerification(
                ClaimsVerificationError::SignatureVerification(
                    SignatureVerificationError::DisallowedAlg(_),
                ),
            )) => {}
            other => panic!("unexpected result: {:?}", other),
        }

        // The builder route carries the client secret for the remaining shared-secret
        // algorithms as well: HS384 and HS512 verify once the algorithm is explicitly allowed
        // and the client secret is present, and stay rejected otherwise.
        for alg in [
            CoreJwsSigningAlgorithm::HmacSha384,
            CoreJwsSigningAlgorithm::HmacSha512,
        ] {
            let token = sign_hs(&alg, "the_client_secret");
            let new_secret = || ClientSecret::new("the_client_secret".to_string());

            // With the supplied client secret, the allowed shared-secret algorithm verifies.
            let request = make_request()
                .set_allowed_algs(vec![alg.clone()])
                .set_client_secret(new_secret());
            let claims = signed_response_claims(request, token.clone());
            assert_eq!(*claims.subject(), sub);

            // Without the client secret, the allowed shared-secret algorithm is still rejected.
            let request = make_request().set_allowed_algs(vec![alg.clone()]);
            assert_signed_response_disallowed(request, token.clone());

            // Without an explicit allowlist, the RS256-only default rejects the response even
            // with the client secret present.
            let request = make_request().set_client_secret(new_secret());
            assert_signed_response_disallowed(request, token);
        }

        fn signed_response_claims(
            request: UserInfoRequest<'_, CoreJweContentEncryptionAlgorithm, CoreJsonWebKey>,
            token: String,
        ) -> UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim> {
            request
                .user_info_response::<
                    EmptyAdditionalClaims,
                    CoreGenderClaim,
                    crate::reqwest::Error,
                >(
                    http::Response::builder()
                        .status(http::StatusCode::OK)
                        .header(http::header::CONTENT_TYPE, "application/jwt")
                        .body(token.into_bytes())
                        .unwrap(),
                )
                .expect("verification should succeed")
        }

        fn assert_signed_response_disallowed(
            request: UserInfoRequest<'_, CoreJweContentEncryptionAlgorithm, CoreJsonWebKey>,
            token: String,
        ) {
            match request
                .user_info_response::<
                    EmptyAdditionalClaims,
                    CoreGenderClaim,
                    crate::reqwest::Error,
                >(
                    http::Response::builder()
                        .status(http::StatusCode::OK)
                        .header(http::header::CONTENT_TYPE, "application/jwt")
                        .body(token.into_bytes())
                        .unwrap(),
                ) {
                Err(UserInfoError::ClaimsVerification(
                    ClaimsVerificationError::SignatureVerification(
                        SignatureVerificationError::DisallowedAlg(_),
                    ),
                )) => {}
                other => panic!("unexpected result: {:?}", other),
            }
        }
    }

    /// Error type for the recording mock HTTP clients used in the tests below.
    #[derive(Debug)]
    struct MockHttpClientError;

    impl std::fmt::Display for MockHttpClientError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("mock http client error")
        }
    }

    impl std::error::Error for MockHttpClientError {}

    /// Returns an HTTP client that records each dispatched request and answers with a JSON
    /// user info response carrying the given status and Content-Type.
    fn recording_client(
        dispatched: std::rc::Rc<std::cell::RefCell<Vec<HttpRequest>>>,
        status: http::StatusCode,
        content_type: &'static str,
    ) -> impl Fn(HttpRequest) -> Result<HttpResponse, MockHttpClientError> {
        move |request| {
            dispatched.borrow_mut().push(request);
            Ok(http::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, content_type)
                .body("{\"sub\":\"the_subject\"}".as_bytes().to_vec())
                .unwrap())
        }
    }

    /// Returns an asynchronous HTTP client with the same recording behavior, shaped for the
    /// `AsyncHttpClient` blanket impl.
    fn recording_async_client(
        dispatched: std::rc::Rc<std::cell::RefCell<Vec<HttpRequest>>>,
    ) -> impl Fn(
        HttpRequest,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<HttpResponse, MockHttpClientError>>>,
    > {
        move |request| {
            let dispatched = dispatched.clone();
            Box::pin(async move {
                dispatched.borrow_mut().push(request);
                Ok(http::Response::builder()
                    .status(http::StatusCode::OK)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body("{\"sub\":\"the_subject\"}".as_bytes().to_vec())
                    .unwrap())
            })
        }
    }

    /// A provider-issued access token containing a control character fails request preparation
    /// through the public path without panicking, without echoing the token bytes, and without
    /// dispatching any HTTP call.
    #[test]
    fn test_user_info_request_malformed_access_token_fails_without_dispatch() {
        use crate::core::{
            CoreJsonWebKey, CoreJsonWebKeySet, CoreJweContentEncryptionAlgorithm,
            CoreUserInfoVerifier,
        };
        use crate::{
            AccessToken, ClientId, EmptyAdditionalClaims, IssuerUrl, SubjectIdentifier,
            UserInfoError, UserInfoRequest, UserInfoResponseType,
        };

        let url = crate::UserInfoUrl::new("https://example.com/userinfo".to_string()).unwrap();
        let request = UserInfoRequest::<CoreJweContentEncryptionAlgorithm, CoreJsonWebKey> {
            url: &url,
            access_token: AccessToken::new("the_access\ntoken".to_string()),
            require_signed_response: false,
            response_type: UserInfoResponseType::Json,
            signed_response_verifier: CoreUserInfoVerifier::new(
                ClientId::new("my_client".to_string()),
                IssuerUrl::new("https://example.com".to_string()).unwrap(),
                CoreJsonWebKeySet::new(vec![]),
                Some(SubjectIdentifier::new("the_subject".to_string())),
            ),
        };

        let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let http_client =
            recording_client(dispatched.clone(), http::StatusCode::OK, "application/json");

        match request.request::<EmptyAdditionalClaims, CoreGenderClaim, _>(&http_client) {
            Err(UserInfoError::Other(message)) => {
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

    /// The asynchronous path surfaces the same preparation failure without dispatching: request
    /// preparation runs before the first await, so polling once with a no-op waker (no async
    /// executor in the dev-dependencies) returns the ready error.
    #[test]
    fn test_user_info_request_async_malformed_access_token_fails_without_dispatch() {
        use crate::core::{
            CoreJsonWebKey, CoreJsonWebKeySet, CoreJweContentEncryptionAlgorithm,
            CoreUserInfoVerifier,
        };
        use crate::{
            AccessToken, ClientId, EmptyAdditionalClaims, IssuerUrl, SubjectIdentifier,
            UserInfoError, UserInfoRequest, UserInfoResponseType,
        };

        let url = crate::UserInfoUrl::new("https://example.com/userinfo".to_string()).unwrap();
        let request = UserInfoRequest::<CoreJweContentEncryptionAlgorithm, CoreJsonWebKey> {
            url: &url,
            access_token: AccessToken::new("the_access\ntoken".to_string()),
            require_signed_response: false,
            response_type: UserInfoResponseType::Json,
            signed_response_verifier: CoreUserInfoVerifier::new(
                ClientId::new("my_client".to_string()),
                IssuerUrl::new("https://example.com".to_string()).unwrap(),
                CoreJsonWebKeySet::new(vec![]),
                Some(SubjectIdentifier::new("the_subject".to_string())),
            ),
        };

        let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let http_client = recording_async_client(dispatched.clone());

        let mut future = Box::pin(
            request.request_async::<EmptyAdditionalClaims, _, CoreGenderClaim>(&http_client),
        );
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match future.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(Err(UserInfoError::Other(message))) => {
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

    /// A valid access token still produces the expected `Authorization: Bearer` header on the
    /// dispatched request, and the JSON response parses into claims.
    #[test]
    fn test_user_info_request_sends_bearer_header() {
        use crate::core::{
            CoreJsonWebKey, CoreJsonWebKeySet, CoreJweContentEncryptionAlgorithm,
            CoreUserInfoVerifier,
        };
        use crate::{
            AccessToken, ClientId, EmptyAdditionalClaims, IssuerUrl, SubjectIdentifier,
            UserInfoRequest, UserInfoResponseType,
        };
        use http::header::HeaderValue;

        let sub = SubjectIdentifier::new("the_subject".to_string());
        let url = crate::UserInfoUrl::new("https://example.com/userinfo".to_string()).unwrap();
        let request = UserInfoRequest::<CoreJweContentEncryptionAlgorithm, CoreJsonWebKey> {
            url: &url,
            access_token: AccessToken::new("the_access_token".to_string()),
            require_signed_response: false,
            response_type: UserInfoResponseType::Json,
            signed_response_verifier: CoreUserInfoVerifier::new(
                ClientId::new("my_client".to_string()),
                IssuerUrl::new("https://example.com".to_string()).unwrap(),
                CoreJsonWebKeySet::new(vec![]),
                Some(sub.clone()),
            ),
        };

        let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let http_client =
            recording_client(dispatched.clone(), http::StatusCode::OK, "application/json");

        let claims = request
            .request::<EmptyAdditionalClaims, CoreGenderClaim, _>(&http_client)
            .expect("user info request should succeed");
        assert_eq!(*claims.subject(), sub);

        let dispatched = dispatched.borrow();
        assert_eq!(dispatched.len(), 1);
        assert_eq!(
            dispatched[0].headers().get(http::header::AUTHORIZATION),
            Some(&HeaderValue::from_static("Bearer the_access_token")),
        );
    }

    /// Content-Type routing accepts RFC 7231 optional whitespace and case variance: an
    /// OWS-bearing JSON Content-Type parses JSON claims, and an OWS-bearing JWT Content-Type
    /// routes to JWT verification (an invalid JWT body then fails parsing, not Content-Type
    /// matching).
    #[test]
    fn test_user_info_response_routes_on_content_type_with_optional_whitespace() {
        use crate::core::{
            CoreJsonWebKey, CoreJsonWebKeySet, CoreJweContentEncryptionAlgorithm,
            CoreUserInfoVerifier,
        };
        use crate::{
            AccessToken, ClientId, EmptyAdditionalClaims, IssuerUrl, SubjectIdentifier,
            UserInfoError, UserInfoRequest, UserInfoResponseType,
        };

        let sub = SubjectIdentifier::new("the_subject".to_string());
        let url = crate::UserInfoUrl::new("https://example.com/userinfo".to_string()).unwrap();
        let make_request = || UserInfoRequest::<CoreJweContentEncryptionAlgorithm, CoreJsonWebKey> {
            url: &url,
            access_token: AccessToken::new("the_access_token".to_string()),
            require_signed_response: false,
            response_type: UserInfoResponseType::Json,
            signed_response_verifier: CoreUserInfoVerifier::new(
                ClientId::new("my_client".to_string()),
                IssuerUrl::new("https://example.com".to_string()).unwrap(),
                CoreJsonWebKeySet::new(vec![]),
                Some(sub.clone()),
            ),
        };
        let response = |content_type: &'static str, body: &'static str| {
            http::Response::builder()
                .status(http::StatusCode::OK)
                .header(http::header::CONTENT_TYPE, content_type)
                .body(body.as_bytes().to_vec())
                .unwrap()
        };

        let claims = make_request()
            .user_info_response::<EmptyAdditionalClaims, CoreGenderClaim, crate::reqwest::Error>(
                response(
                    "APPLICATION/JSON ; charset=utf-8",
                    "{\"sub\":\"the_subject\"}",
                ),
            )
            .expect("JSON response with OWS-bearing Content-Type should parse");
        assert_eq!(*claims.subject(), sub);

        match make_request()
            .user_info_response::<EmptyAdditionalClaims, CoreGenderClaim, crate::reqwest::Error>(
                response("application/jwt\t", "not-a-jwt"),
            ) {
            Err(UserInfoError::Parse(_)) => {}
            other => panic!("expected the JWT route to fail parsing, got: {other:?}"),
        }
    }

    /// A confidential `Client` routes its client secret into the signed-response verifier built
    /// by `user_info`: an HS256 response verifies with only `set_allowed_algs`, while a public
    /// client (no secret) rejects the same response even when the algorithm is allowed.
    #[test]
    fn test_user_info_request_confidential_client_routes_secret() {
        use crate::core::{CoreClient, CoreHmacKey, CoreJsonWebKeySet, CoreJwsSigningAlgorithm};
        use crate::{
            AccessToken, ClaimsVerificationError, ClientId, ClientSecret, EmptyAdditionalClaims,
            IssuerUrl, PrivateSigningKey, SignatureVerificationError, SubjectIdentifier,
            UserInfoError,
        };
        use base64::Engine;

        let b64 = crate::core::base64_url_safe_no_pad();
        let client_id = ClientId::new("my_client".to_string());
        let issuer = IssuerUrl::new("https://example.com".to_string()).unwrap();
        let sub = SubjectIdentifier::new("the_subject".to_string());

        let header = "{\"alg\":\"HS256\",\"typ\":\"JWT\"}";
        let payload = "{\"iss\":\"https://example.com\",\"aud\":[\"my_client\"],\
\"sub\":\"the_subject\",\"name\":\"Jane Doe\"}";
        let signing_input = format!(
            "{}.{}",
            b64.encode(header.as_bytes()),
            b64.encode(payload.as_bytes())
        );
        let hmac_key = CoreHmacKey::new("the_client_secret");
        let signature = hmac_key
            .sign(
                &CoreJwsSigningAlgorithm::HmacSha256,
                signing_input.as_bytes(),
            )
            .expect("HS256 signing should succeed");
        let token = format!("{}.{}", signing_input, b64.encode(signature));

        let url = crate::UserInfoUrl::new("https://example.com/userinfo".to_string()).unwrap();
        let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let http_client = |request: HttpRequest| -> Result<HttpResponse, MockHttpClientError> {
            dispatched.borrow_mut().push(request);
            Ok(http::Response::builder()
                .status(http::StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/jwt")
                .body(token.clone().into_bytes())
                .unwrap())
        };

        // The confidential client's secret reaches the signed-response verifier: with the
        // algorithm allowed, the response verifies without further configuration.
        let confidential_client = CoreClient::new(
            client_id.clone(),
            issuer.clone(),
            CoreJsonWebKeySet::new(vec![]),
        )
        .set_client_secret(ClientSecret::new("the_client_secret".to_string()))
        .set_user_info_url(url.clone());
        let claims = confidential_client
            .user_info(
                AccessToken::new("the_access_token".to_string()),
                Some(sub.clone()),
            )
            .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256])
            .request::<EmptyAdditionalClaims, CoreGenderClaim, _>(&http_client)
            .expect("confidential routing should carry the client secret");
        assert_eq!(*claims.subject(), sub);

        // A public client has no secret to carry: the same response is rejected even when the
        // algorithm is allowed.
        let public_client = CoreClient::new(client_id, issuer, CoreJsonWebKeySet::new(vec![]))
            .set_user_info_url(url);
        match public_client
            .user_info(AccessToken::new("the_access_token".to_string()), Some(sub))
            .set_allowed_algs(vec![CoreJwsSigningAlgorithm::HmacSha256])
            .request::<EmptyAdditionalClaims, CoreGenderClaim, _>(&http_client)
        {
            Err(UserInfoError::ClaimsVerification(
                ClaimsVerificationError::SignatureVerification(
                    SignatureVerificationError::DisallowedAlg(_),
                ),
            )) => {}
            other => panic!("unexpected result: {:?}", other),
        }
        assert_eq!(dispatched.borrow().len(), 2);
    }

    /// The asynchronous path produces the same `Authorization: Bearer` header on the dispatched
    /// request: preparation runs before the first await, so polling once with a no-op waker (no
    /// async executor in the dev-dependencies) dispatches the request and completes.
    #[test]
    fn test_user_info_request_async_sends_bearer_header() {
        use crate::core::{
            CoreJsonWebKey, CoreJsonWebKeySet, CoreJweContentEncryptionAlgorithm,
            CoreUserInfoVerifier,
        };
        use crate::{
            AccessToken, ClientId, EmptyAdditionalClaims, IssuerUrl, SubjectIdentifier,
            UserInfoRequest, UserInfoResponseType,
        };
        use http::header::HeaderValue;

        let sub = SubjectIdentifier::new("the_subject".to_string());
        let url = crate::UserInfoUrl::new("https://example.com/userinfo".to_string()).unwrap();
        let request = UserInfoRequest::<CoreJweContentEncryptionAlgorithm, CoreJsonWebKey> {
            url: &url,
            access_token: AccessToken::new("the_access_token".to_string()),
            require_signed_response: false,
            response_type: UserInfoResponseType::Json,
            signed_response_verifier: CoreUserInfoVerifier::new(
                ClientId::new("my_client".to_string()),
                IssuerUrl::new("https://example.com".to_string()).unwrap(),
                CoreJsonWebKeySet::new(vec![]),
                Some(sub.clone()),
            ),
        };

        let dispatched = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let http_client = recording_async_client(dispatched.clone());

        let mut future = Box::pin(
            request.request_async::<EmptyAdditionalClaims, _, CoreGenderClaim>(&http_client),
        );
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match future.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(Ok(claims)) => {
                assert_eq!(*claims.subject(), sub);
            }
            other => panic!("expected a ready successful response, got: {other:?}"),
        }

        let dispatched = dispatched.borrow();
        assert_eq!(dispatched.len(), 1);
        assert_eq!(
            dispatched[0].headers().get(http::header::AUTHORIZATION),
            Some(&HeaderValue::from_static("Bearer the_access_token")),
        );
    }
}
