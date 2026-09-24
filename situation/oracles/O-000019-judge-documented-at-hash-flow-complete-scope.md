# O-000019 — Judge documented at_hash flow complete scope

## State

implemented

## Judges

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Inputs

At the judged head, the public `IdToken::verification_key` and
`AccessTokenHash::from_token` call path, the confidential and public verifier
key-resolution paths, and the documented crate-root tutorials. The retained
focused command exercises HS256 and ES256 representatives; this successor
also names the previously unrepresented HS384, HS512, and ES384 decisions.

## Pass

- P1 — An HS256 ID token with a correct access-token hash verifies through a
  confidential verifier with an empty JWKS, and the documented call reproduces
  that hash.
- P2 — The corresponding documented flow works for HS384.
- P3 — The corresponding documented flow works for HS512.
- P4 — An ES256 ID token resolves its matching provider JWK and the documented
  call reproduces its access-token hash.
- P5 — The corresponding asymmetric flow works for ES384 with a matching
  P-384 provider JWK.
- P6 — A shared-secret token used with a verifier holding no client secret
  returns an error rather than panicking.
- P7 — A substituted access token produces a different hash.
- P8 — `IdToken::signing_key` remains available with its documented JWKS-only
  borrowed-key scope.
- P9 — Both crate-root tutorials compile against `verification_key` followed
  by `AccessTokenHash::from_token` with `reqwest-blocking` enabled.

## Fail

- F1 — An HS256, HS384, or HS512 flow needs a provider JWKS key rather than
  the confidential verifier's client secret, or accepts a missing client
  secret.
- F2 — An ES256 or ES384 flow resolves a nonmatching provider JWK or cannot
  reproduce the expected access-token hash.
- F3 — A substituted access token compares equal to the token's `at_hash`.
- F4 — The documented tutorials no longer compile against the owned-key
  `verification_key` call.
- F5 — `signing_key` no longer has the documented borrowed JWKS-only scope.

## Implementation

`cargo test --offline --lib --quiet -- id_token_verification_key_at_hash`
executes the retained HS256 and ES256 fixtures, including the missing-secret
and substituted-token failures. The remaining algorithm and API/documentation
legs use the coverage entries below until their own observations are retained.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Confidential HS256 flow succeeds with empty JWKS. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| P2 | Confidential HS384 flow succeeds with empty JWKS. | manual — `src/verification/mod.rs::JwtClaimsVerifier::verification_key` and `src/core/jwk/mod.rs::CoreJsonWebKey::hash_bytes` SHA-384 branch |
| P3 | Confidential HS512 flow succeeds with empty JWKS. | manual — `src/verification/mod.rs::JwtClaimsVerifier::verification_key` and `src/core/jwk/mod.rs::CoreJsonWebKey::hash_bytes` SHA-512 branch |
| P4 | ES256 flow resolves matching JWK and hash. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| P5 | ES384 flow resolves matching P-384 JWK and hash. | manual — `src/core/jwk/mod.rs::CoreJsonWebKey::hash_bytes` SHA-384 branch and matching-key resolution |
| P6 | Missing shared secret errors without panic. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| P7 | Substituted token hash differs. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| P8 | Borrowed `signing_key` remains JWKS-only. | manual — `src/id_token/mod.rs::IdToken::signing_key` documentation and implementation |
| P9 | Both tutorials compile against the documented owned-key flow. | `cargo test --offline --doc --features reqwest-blocking` |
| F1 | Shared-secret key resolution does not fall back or accept no secret. | `src/verification/tests.rs::test_id_token_verification_key_at_hash`; manual for HS384/HS512 |
| F2 | Asymmetric key resolution/hash fidelity is preserved. | `src/verification/tests.rs::test_id_token_verification_key_at_hash`; manual for ES384 |
| F3 | Substituted token does not compare equal. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| F4 | Documented owned-key flow continues to compile. | `cargo test --offline --doc --features reqwest-blocking` |
| F5 | `signing_key` scope remains unchanged. | manual — `src/id_token/mod.rs::IdToken::signing_key` |

## References

- Supersedes O-000010 for P-000009's complete declared scope; W-000012
  remains an observation of the historical, narrower rule.
- situation/witnesses/P-000009/W-000021-documented-at-hash-complete-scope-incomplete.md
