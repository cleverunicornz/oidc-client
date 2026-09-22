# O-000008 — Judge HMAC SHA-2 and ES384 JWS verification

## State

implemented

## Judges

situation/promises/P-000007-hmac-sha2-and-es384-jws-verification.md

## Inputs

The fixed symmetric-key and EC P-384 JWK fixtures in
`src/core/jwk/tests.rs`, their matching JWS signing inputs and signatures, and
the two scoped Cargo test commands named below.

## Pass

- P1: The fixed HS256 input and matching signature verify with its symmetric
  JWK through `CoreJwsSigningAlgorithm::HmacSha256`.
- P2: The fixed HS384 input and matching signature verify with its symmetric
  JWK through `CoreJwsSigningAlgorithm::HmacSha384`.
- P3: The fixed HS512 input and matching signature verify with its symmetric
  JWK through `CoreJwsSigningAlgorithm::HmacSha512`.
- P4: The fixed ES384 input and matching signature verify with its EC P-384
  JWK through `CoreJwsSigningAlgorithm::EcdsaP384Sha384`.

## Fail

- F1: Any of the fixed matched HMAC signatures is rejected by its corresponding
  HS256, HS384, or HS512 verification path.
- F2: The fixed matched ES384 signature is rejected by the P-384 verification
  path.
- F3: The ES384 path accepts the P-256 signature fixture.
- F4: The ES384 path accepts a P-256 JWK instead of rejecting the mismatched
  curve before verification.

## Implementation

`cargo test --all-features test_hmac_sha256_verification` executes the three
HMAC fixture checks. `cargo test --all-features test_ecdsa_verification`
executes the ES384 positive, invalid-signature, and mismatched-curve checks.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | The HS256 fixed signature verifies with the symmetric JWK. | `src/core/jwk/tests.rs::test_hmac_sha256_verification` |
| P2 | The HS384 fixed signature verifies with the symmetric JWK. | `src/core/jwk/tests.rs::test_hmac_sha256_verification` |
| P3 | The HS512 fixed signature verifies with the symmetric JWK. | `src/core/jwk/tests.rs::test_hmac_sha256_verification` |
| P4 | The ES384 fixed signature verifies with the P-384 JWK. | `src/core/jwk/tests.rs::test_ecdsa_verification` |
| F1 | A rejection of any matched HMAC fixture fails its verification assertion. | `src/core/jwk/tests.rs::test_hmac_sha256_verification` |
| F2 | A rejection of the matched ES384 fixture fails its verification assertion. | `src/core/jwk/tests.rs::test_ecdsa_verification` |
| F3 | An accepted P-256 signature under ES384 fails `verify_invalid_signature`. | `src/core/jwk/tests.rs::test_ecdsa_verification` |
| F4 | An accepted P-256 JWK under ES384 fails the mismatched-curve assertion. | `src/core/jwk/tests.rs::test_ecdsa_verification` |
