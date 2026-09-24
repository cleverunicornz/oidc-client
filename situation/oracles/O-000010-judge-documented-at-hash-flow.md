# O-000010 — Judge documented at_hash flow for shared-secret and asymmetric ID tokens

## State

implemented

## Judges

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Inputs

The scoped Cargo test command below over the fixed HS256 (client-secret) and
ES256 (EC P-256 JWK) ID-token fixtures in `src/verification/tests.rs`, the
crate-root tutorial doctests in `src/lib.rs`, and the `gitlab` example target.

## Pass

- P1: For an HS256 ID token carrying a correct `at_hash`, a confidential
  verifier with an empty JWKS verifies the token, `IdToken::verification_key`
  resolves a key, and `AccessTokenHash::from_token` with that key reproduces
  the token's `at_hash`.
- P2: For an ES256 ID token, the same documented call resolves the provider's
  EC P-256 JWK from the JWKS and reproduces the access-token hash.
- P3: Both crate-root tutorials compile against the documented call
  (`verification_key`) with the `reqwest-blocking` feature enabled.

## Fail

- F1: An `AccessTokenHash::from_token` comparison with a substituted access
  token equals the token's `at_hash`; it must differ.
- F2: `IdToken::verification_key` resolves a shared-secret key for an HS256
  token when the verifier holds no client secret, instead of returning an
  error; or it panics.

## Implementation

`cargo test --offline --lib -- verification::tests::test_id_token_verification_key_at_hash`
executes the fixture legs. `cargo test --offline --doc --features reqwest-blocking`
and `cargo check --offline --example gitlab --features reqwest-blocking`
execute the documentation leg.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | HS256 documented flow passes with empty JWKS via confidential verifier. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| P2 | ES256 documented flow resolves the JWKS key. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| P3 | Tutorials compile against the documented call. | `cargo test --offline --doc --features reqwest-blocking` |
| F1 | Substituted access token fails the comparison. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| F2 | Shared-secret resolution without a client secret errors instead of panicking. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |

## References

- Superseded for P-000009's complete declared Scope by
  situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md.
  W-000012 remains an observation of this historical, narrower rule.
