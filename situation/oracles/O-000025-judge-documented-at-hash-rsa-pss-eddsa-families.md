# O-000025 — Judge documented at_hash flow complete-scope successor

## State

implemented

## Judges

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Inputs

At the judged head, the public `IdToken::verification_key` and
`AccessTokenHash::from_token` call path; confidential and public verifier
key-resolution paths; `IdToken::signing_key`; the crate-root tutorials; and
these retained fixtures:

- `test_id_token_verification_key_at_hash` for HS256, ES256, a substituted
  access token, and the missing-client-secret failure;
- `test_id_token_verification_key_at_hash_hs384_hs512` for HS384 and HS512;
- `test_id_token_verification_key_at_hash_es384` for ES384;
- `test_id_token_verification_key_at_hash_rsa_pss_eddsa` for
  RS256/384/512, PS256/384/512, and EdDSA.

This successor judges every explicit clause of P-000009's declared Scope. It
supersedes O-000019 rather than composing two partial rules.

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
- P6 — The corresponding asymmetric flow works for RS256 with a matching
  provider RSA JWK.
- P7 — The corresponding asymmetric flow works for RS384 with a matching
  provider RSA JWK.
- P8 — The corresponding asymmetric flow works for RS512 with a matching
  provider RSA JWK.
- P9 — The corresponding asymmetric flow works for PS256 with a matching
  provider RSA JWK.
- P10 — The corresponding asymmetric flow works for PS384 with a matching
  provider RSA JWK.
- P11 — The corresponding asymmetric flow works for PS512 with a matching
  provider RSA JWK.
- P12 — The corresponding asymmetric flow works for EdDSA (Ed25519) with a
  matching provider OKP JWK.
- P13 — A shared-secret token using HS256, HS384, or HS512 with a verifier
  holding no client secret returns an error rather than panicking.
- P14 — A substituted access token produces a different access-token hash than
  the token's `at_hash` for every signature family named in P1 through P12.
- P15 — `IdToken::signing_key` remains available with its documented borrowed
  JWKS-only scope.
- P16 — Both crate-root tutorials compile against `verification_key` followed
  by `AccessTokenHash::from_token` with `reqwest-blocking` enabled.

## Fail

- F1 — An HS256, HS384, or HS512 flow needs a provider JWKS key rather than
  the confidential verifier's client secret, or accepts a missing client
  secret.
- F2 — An ES256, ES384, RS256/384/512, PS256/384/512, or EdDSA flow resolves a
  nonmatching provider JWK or cannot reproduce the expected access-token hash.
- F3 — A substituted access token compares equal to the token's `at_hash` for
  any signature family named in P1 through P12.
- F4 — The documented tutorials no longer compile against the owned-key
  `verification_key` call.
- F5 — `signing_key` no longer has the documented borrowed JWKS-only scope.

## Implementation

`cargo test --offline --lib --quiet -- id_token_verification_key_at_hash`
executes the four named focused fixtures. `cargo test --offline --doc --features
reqwest-blocking` executes the tutorial leg. The `signing_key` scope and the
ES256/ES384 substituted-token negative retain explicit manual decisions in the
coverage table.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Confidential HS256 flow succeeds with empty JWKS. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| P2 | Confidential HS384 flow succeeds with empty JWKS. | `src/verification/tests.rs::test_id_token_verification_key_at_hash_hs384_hs512` |
| P3 | Confidential HS512 flow succeeds with empty JWKS. | `src/verification/tests.rs::test_id_token_verification_key_at_hash_hs384_hs512` |
| P4 | ES256 flow resolves a matching JWK and reproduces the hash. | `src/verification/tests.rs::test_id_token_verification_key_at_hash` |
| P5 | ES384 flow resolves a matching P-384 JWK and reproduces the hash. | `src/verification/tests.rs::test_id_token_verification_key_at_hash_es384` |
| P6 | RS256 flow resolves a matching RSA JWK and reproduces the hash. | `src/verification/tests.rs::test_id_token_verification_key_at_hash_rsa_pss_eddsa` (`RsaSsaPkcs1V15Sha256` iteration) |
| P7 | RS384 flow resolves a matching RSA JWK and reproduces the hash. | same fixture (`RsaSsaPkcs1V15Sha384` iteration) |
| P8 | RS512 flow resolves a matching RSA JWK and reproduces the hash. | same fixture (`RsaSsaPkcs1V15Sha512` iteration) |
| P9 | PS256 flow resolves a matching RSA JWK and reproduces the hash. | same fixture (`RsaSsaPssSha256` iteration) |
| P10 | PS384 flow resolves a matching RSA JWK and reproduces the hash. | same fixture (`RsaSsaPssSha384` iteration) |
| P11 | PS512 flow resolves a matching RSA JWK and reproduces the hash. | same fixture (`RsaSsaPssSha512` iteration) |
| P12 | EdDSA flow resolves a matching OKP JWK and reproduces the hash. | same fixture (`EdDsa` iteration) |
| P13 | Missing shared secret returns an error rather than panicking. | `test_id_token_verification_key_at_hash` for HS256; manual — inspect the common `uses_shared_secret()` branch for HS384/HS512 |
| P14 | A substituted token produces a different hash in every named family. | `test_id_token_verification_key_at_hash`, `test_id_token_verification_key_at_hash_hs384_hs512`, and `test_id_token_verification_key_at_hash_rsa_pss_eddsa`; manual — perform the same substitution for ES256/ES384 |
| P15 | Borrowed `signing_key` remains JWKS-only. | manual — `src/id_token/mod.rs::IdToken::signing_key` documentation and implementation |
| P16 | Both tutorials compile against the documented owned-key flow. | `cargo test --offline --doc --features reqwest-blocking` |
| F1 | Shared-secret key resolution does not fall back or accept no secret. | `test_id_token_verification_key_at_hash` for HS256; manual — inspect the common `uses_shared_secret()` branch for HS384/HS512 |
| F2 | Asymmetric key resolution and hash fidelity reject nonmatching keys. | `test_id_token_verification_key_at_hash`, `test_id_token_verification_key_at_hash_es384`, and `test_id_token_verification_key_at_hash_rsa_pss_eddsa`; manual — exercise a nonmatching key for each named asymmetric family |
| F3 | A substituted token does not compare equal in any named family. | same coverage as P14 |
| F4 | Documented owned-key flow continues to compile. | `cargo test --offline --doc --features reqwest-blocking` |
| F5 | `signing_key` scope remains unchanged. | manual — `src/id_token/mod.rs::IdToken::signing_key` |

## References

- Supersedes O-000019 for P-000009's complete declared Scope. O-000019 and
  W-000028 remain immutable historical partial-rule records and do not compose
  with this successor.
- situation/gaps/G-000033-at-hash-oracle-omits-supported-asymmetric-algorithms.md
  remains open: W-000035 is INVALID for this complete successor because its
  retained run decides only the supplemental asymmetric legs.
- situation/gaps/G-000032-current-pass-evidence-outpaces-frozen-promise-states.md
  retains the frozen Promise-state and forward-link reconciliation concern.
- situation/witnesses/P-000009/W-000035-at-hash-rsa-pss-eddsa-oracle-leg-pass.md
  retains the supplemental fixture observation, classified INVALID against this
  successor.
