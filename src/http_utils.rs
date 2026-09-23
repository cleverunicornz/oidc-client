use crate::AccessToken;

use http::header::{
    HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue, AUTHORIZATION, CONTENT_TYPE,
};

pub const MIME_TYPE_JSON: &str = "application/json";
pub const MIME_TYPE_JWKS: &str = "application/jwk-set+json";
pub const MIME_TYPE_JWT: &str = "application/jwt";

pub const BEARER: &str = "Bearer";

// The [essence](https://mimesniff.spec.whatwg.org/#mime-type-essence) is the <type>/<subtype>
// representation.
pub fn content_type_has_essence(content_type: &HeaderValue, expected_essence: &str) -> bool {
    content_type
        .to_str()
        .ok()
        .filter(|ct| {
            // Section 3.1.1.1 of RFC 7231 indicates that media types are case insensitive and
            // may be preceded and/or followed by optional whitespace (SP/HTAB) and parameters
            // (e.g., charset). Whitespace inside the type/subtype itself remains invalid.
            // See https://tools.ietf.org/html/rfc7231#section-3.1.1.1.
            let essence = match ct.find(';') {
                Some(index) => &ct[..index],
                None => ct,
            };
            essence
                .trim_matches(|c: char| c == ' ' || c == '\t')
                .eq_ignore_ascii_case(expected_essence)
        })
        .is_some()
}

pub fn check_content_type(headers: &HeaderMap, expected_content_type: &str) -> Result<(), String> {
    headers
        .get(CONTENT_TYPE)
        .map_or(Ok(()), |content_type|
            // Section 3.1.1.1 of RFC 7231 indicates that media types are case insensitive and
            // may be followed by optional whitespace and/or a parameter (e.g., charset).
            // See https://tools.ietf.org/html/rfc7231#section-3.1.1.1.
            if !content_type_has_essence(content_type, expected_content_type) {
                Err(
                    format!(
                        "Unexpected response Content-Type: {:?}, should be `{}`",
                        content_type,
                        expected_content_type
                    )
                )
            } else {
                Ok(())
            }
        )
}

/// Builds the `Authorization: Bearer` header pair for the supplied access token.
///
/// Returns the underlying header construction error if the token contains characters that are
/// invalid in an HTTP header value (e.g., a newline in a provider-issued token). The error
/// message never includes the token bytes.
pub fn auth_bearer(
    access_token: &AccessToken,
) -> Result<(HeaderName, HeaderValue), InvalidHeaderValue> {
    HeaderValue::from_str(&format!("{} {}", BEARER, access_token.secret()))
        .map(|auth_value| (AUTHORIZATION, auth_value))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::AccessToken;

    #[test]
    fn test_content_type_has_essence_accepts_case_and_optional_whitespace() {
        for content_type in [
            "application/json",
            "Application/JSON",
            "application/json ; charset=utf-8",
            "application/json;charset=utf-8",
            "\tapplication/json\t",
        ] {
            assert!(
                content_type_has_essence(
                    &HeaderValue::from_str(content_type).unwrap(),
                    MIME_TYPE_JSON
                ),
                "expected `{content_type}` to match `{MIME_TYPE_JSON}`"
            );
        }
    }

    #[test]
    fn test_content_type_has_essence_rejects_unexpected_and_malformed_values() {
        for content_type in [
            "application/jsonx",
            "application/jwt",
            "appl ication/json",
            "text/plain",
        ] {
            assert!(
                !content_type_has_essence(
                    &HeaderValue::from_str(content_type).unwrap(),
                    MIME_TYPE_JSON
                ),
                "expected `{content_type}` not to match `{MIME_TYPE_JSON}`"
            );
        }

        // Non-UTF-8 header values stay rejected.
        assert!(!content_type_has_essence(
            &HeaderValue::from_bytes(&[0xff, 0xfe]).unwrap(),
            MIME_TYPE_JSON
        ));
    }

    #[test]
    fn test_auth_bearer_builds_authorization_header() {
        let (name, auth_value) = auth_bearer(&AccessToken::new("the_access_token".to_string()))
            .expect("valid access token should build the Authorization header");
        assert_eq!(name, AUTHORIZATION);
        assert_eq!(auth_value, "Bearer the_access_token");
    }

    #[test]
    fn test_auth_bearer_rejects_malformed_token_without_echo() {
        let error = auth_bearer(&AccessToken::new("the_access\ntoken".to_string()))
            .expect_err("malformed access token must error instead of panicking");
        assert!(
            !error.to_string().contains("the_access"),
            "error must not echo the access token: {error}"
        );
    }
    #[test]
    fn test_check_content_type_tolerates_missing_header_and_accepts_ows() {
        // A missing Content-Type header stays tolerated (deliberate).
        let headers = HeaderMap::new();
        assert_eq!(check_content_type(&headers, MIME_TYPE_JSON), Ok(()));

        // An OWS-bearing Content-Type is accepted on the check_content_type path.
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json ; charset=utf-8"),
        );
        assert_eq!(check_content_type(&headers, MIME_TYPE_JSON), Ok(()));

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        assert!(check_content_type(&headers, MIME_TYPE_JSON).is_err());
    }
}
