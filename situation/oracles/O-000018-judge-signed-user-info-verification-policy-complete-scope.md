# O-000018 — Judge signed-UserInfo verification policy complete scope

## State

implemented

## Judges

situation/promises/P-000008-signed-user-info-verification-policy.md

## Inputs

At the judged head, the signed UserInfo verifier and request-builder surfaces,
the `Client::user_info` construction path, and focused fixtures for the
representative ES256 and HS256 flows. This successor retains O-000009's
representative executable checks and adds explicit manual decisions for the
previously unrepresented algorithms and confidential-client routing.

## Pass

- P1 — The default UserInfo verifier accepts only RS256; every other algorithm
  requires an explicit `set_allowed_algs` configuration.
- P2 — An ES256 response with a matching P-256 key verifies when ES256 is
  explicitly allowed.
- P3 — An ES384 response with a matching P-384 key verifies when ES384 is
  explicitly allowed.
- P4 — An HS256 response verifies through a confidential verifier holding the
  matching client secret only when HS256 is explicitly allowed.
- P5 — An HS384 response verifies through a confidential verifier holding the
  matching client secret only when HS384 is explicitly allowed.
- P6 — An HS512 response verifies through a confidential verifier holding the
  matching client secret only when HS512 is explicitly allowed.
- P7 — No HS256, HS384, or HS512 response verifies through a public verifier
  without a client secret, even when that algorithm is allowed.
- P8 — `Client::user_info` constructs a confidential `UserInfoVerifier` when
  the Client holds a secret, preserving that secret for shared-secret
  verification.
- P9 — Signed responses still enforce issuer, audience, and configured
  expected-subject checks.
- P10 — Neither `UserInfoVerifier` nor `UserInfoRequest` exposes an
  allow-any-algorithm API.

## Fail

- F1 — A default verifier accepts an ES256, ES384, or HS* response without an
  explicit allowlist entry.
- F2 — An ES256 or ES384 response signed by an altered or nonmatching key
  verifies.
- F3 — An HS256, HS384, or HS512 response verifies with a missing or wrong
  client secret.
- F4 — A confidential Client routes a UserInfo request through a public
  verifier and loses its client-secret verification key.
- F5 — A signed response with a mismatched issuer, audience, or expected
  subject is accepted.
- F6 — A UserInfo API can disable the allowlist or accept any algorithm.

## Implementation

`cargo test --offline --lib --quiet -- user_info_signed_response_` executes
the retained ES256 and HS256 representative fixtures. The remaining explicit
algorithm and Client-routing legs require the manual checks named below until
their own focused fixtures are retained.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Default is RS256-only; opt-in is required. | `src/verification/tests.rs::test_user_info_signed_response_es256`; `src/user_info.rs::test_user_info_request_signed_response_policy` |
| P2 | Allowed ES256 verifies. | `src/verification/tests.rs::test_user_info_signed_response_es256` |
| P3 | Allowed ES384 verifies. | manual — `src/core/jwk/mod.rs::CoreJsonWebKey::verify_signature` ES384 branch and verifier allowlist dispatch |
| P4 | Allowed confidential HS256 verifies. | `src/verification/tests.rs::test_user_info_signed_response_hs256` |
| P5 | Allowed confidential HS384 verifies. | manual — `src/core/jwk/mod.rs::CoreJsonWebKey::verify_signature` HS384 branch and shared-secret key resolution |
| P6 | Allowed confidential HS512 verifies. | manual — `src/core/jwk/mod.rs::CoreJsonWebKey::verify_signature` HS512 branch and shared-secret key resolution |
| P7 | Every HS* mode rejects a public verifier. | manual — `src/verification/mod.rs::JwtClaimsVerifier::verification_key` applies `uses_shared_secret` to every HS* algorithm |
| P8 | Client-secret routing selects a confidential verifier. | manual — `src/user_info.rs::Client::user_info_impl` |
| P9 | Issuer, audience, and expected subject remain enforced. | `src/verification/tests.rs::test_user_info_verified_claims` |
| P10 | No UserInfo allow-any API exists. | manual — public-surface inspection of `src/verification/mod.rs` and `src/user_info.rs` |
| F1 | Default/non-allowlisted algorithm acceptance is rejected. | `src/verification/tests.rs::test_user_info_signed_response_es256`; manual for ES384/HS384/HS512 |
| F2 | Altered or nonmatching EC signatures are rejected. | `src/verification/tests.rs::test_user_info_signed_response_es256`; manual for ES384 |
| F3 | Missing or wrong HS* secrets are rejected. | `src/verification/tests.rs::test_user_info_signed_response_hs256`; manual for HS384/HS512 |
| F4 | Confidential Client routing retains its secret. | manual — `src/user_info.rs::Client::user_info_impl` |
| F5 | Mismatched signed claims are rejected. | `src/verification/tests.rs::test_user_info_verified_claims` |
| F6 | Allow-any UserInfo configuration is absent. | manual — public-surface inspection of `src/verification/mod.rs` and `src/user_info.rs` |

## References

- Supersedes O-000009 for P-000008's complete declared scope; W-000011
  remains an observation of the historical, narrower rule.
- situation/witnesses/P-000008/W-000020-signed-user-info-complete-scope-incomplete.md
