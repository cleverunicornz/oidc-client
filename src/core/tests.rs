use crate::core::{CoreAuthErrorResponseType, CoreGrantType, CoreJwsSigningAlgorithm};

#[test]
fn test_grant_type_serialize() {
    let serialized_implicit = serde_json::to_string(&CoreGrantType::Implicit).unwrap();
    assert_eq!("\"implicit\"", serialized_implicit);
    assert_eq!(
        CoreGrantType::Implicit,
        serde_json::from_str::<CoreGrantType>(&serialized_implicit).unwrap()
    );
}

#[test]
fn test_signature_alg_serde_plain() {
    assert_eq!(
        serde_plain::to_string(&CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256).unwrap(),
        "RS256"
    );
    assert_eq!(
        serde_plain::from_str::<CoreJwsSigningAlgorithm>("RS256").unwrap(),
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256
    );
}

#[test]
fn test_auth_error_type_round_trip() {
    // Regression for the upstream serialization defect recorded in
    // situation/gaps/G-000003-invalid-request-object-serialization-typo.md and
    // corrected under
    // situation/decisions/D-000010-correct-invalid-request-object-serialization.md:
    // the variant must emit the spec string and parse it back.
    let serialized = serde_json::to_string(&CoreAuthErrorResponseType::InvalidRequestObject)
        .expect("serialization should succeed");
    assert_eq!("\"invalid_request_object\"", serialized);
    assert_eq!(
        CoreAuthErrorResponseType::InvalidRequestObject,
        serde_json::from_str::<CoreAuthErrorResponseType>(&serialized)
            .expect("parsing the serialized form should succeed")
    );
    assert_eq!(
        CoreAuthErrorResponseType::InvalidRequestObject,
        serde_plain::from_str::<CoreAuthErrorResponseType>("invalid_request_object")
            .expect("parsing the spec string should succeed")
    );
}
