# O-000002 — Judge ES256 verification

## State

implemented

## Judges

situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md

## Inputs

The fixed P-256 ECDSA signature and EC P-256 JWK fixtures in the imported
unit tests, a discovery-metadata fixture containing `ES256`, intentionally
invalid P-256 signature inputs, a complete ES256 ID-token fixture spanning
valid and invalid claim values, and `cargo test --all-features`.

## Pass

- P1: A valid fixed P-256 ECDSA signature verifies with its matching EC P-256
  JWK when dispatched as ES256 through the crate's verification path.
- P2: An EC JWK with `crv = P-256` parses and is accepted for ES256
  verification.
- P3: Discovery metadata containing `ES256` in
  `id_token_signing_alg_values_supported` parses as the ES256 algorithm.
- P4: A complete ES256 ID-token fixture verifies signature, issuer, audience,
  nonce, and expiry through the public ID-token verifier.

## Fail

- F1: The valid fixed P-256 signature cannot be verified with its matching
  P-256 JWK through ES256 dispatch.
- F2: The P-256 JWK is rejected during parsing or compatibility checks.
- F3: An invalid P-256 signature is accepted through ES256 dispatch.
- F4: A complete ES256 ID-token fixture accepts an invalid signature, issuer,
  audience, nonce, or expiry.

## Implementation

`cargo test --all-features` executes the imported unit tests; the CI route in
`.github/workflows/ci.yml` dispatches that command on the configured runner.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | `test_ecdsa_verification` accepts the valid P-256 signature through the ES256 enum dispatch. | `src/core/jwk/tests.rs::test_ecdsa_verification` |
| P2 | EC P-256 JWK parsing and ES256 key compatibility are asserted. | `src/core/jwk/tests.rs::test_core_jwk_deserialization_ec`; `src/core/jwk/tests.rs::test_ecdsa_verification` |
| P3 | A discovery fixture containing ES256 is deserialized and compared to `CoreJwsSigningAlgorithm::EcdsaP256Sha256`. | `src/discovery/tests.rs::test_discovery_deserialization` |
| P4 | A complete ES256 ID-token fixture exercises the public verifier's signature and claim checks. | manual |
| F1 | The valid-fixture assertion fails if ES256 dispatch rejects it. | `src/core/jwk/tests.rs::test_ecdsa_verification` |
| F2 | The EC parsing and compatibility assertions fail if the P-256 key is rejected. | `src/core/jwk/tests.rs::test_core_jwk_deserialization_ec`; `src/core/jwk/tests.rs::test_ecdsa_verification` |
| F3 | The invalid P-256 signature assertion fails if ES256 dispatch accepts it. | `src/core/jwk/tests.rs::test_ecdsa_verification` |
| F4 | The same fixture supplies each invalid in-scope condition and observes rejection. | manual |
