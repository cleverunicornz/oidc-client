# O-000009 — Judge signed-UserInfo verification policy

## State

implemented

## Judges

situation/promises/P-000008-signed-user-info-verification-policy.md

## Inputs

The scoped Cargo test commands below over the fixed ES256 P-256 and HS256
fixtures in `src/verification/tests.rs` and `src/user_info.rs`, plus the public
API surface of `UserInfoVerifier` and `UserInfoRequest` in
`src/verification/mod.rs` and `src/user_info.rs`.

## Pass

- P1: An ES256-signed user info JWT whose claims (issuer, audience, subject)
  are valid verifies when `EcdsaP256Sha256` is explicitly allowed.
- P2: An HS256-signed user info JWT verifies through a confidential verifier
  holding the matching client secret with `HmacSha256` explicitly allowed,
  against an empty JWKS.
- P3: Issuer, audience, and expected-subject checks remain enforced for signed
  user info responses (valid fixture accepted; wrong-issuer, wrong-audience,
  and mismatched-subject fixtures rejected).

## Fail

- F1: The default verifier (no `set_allowed_algs`) accepts an ES256-signed
  user info JWT; it must reject it with
  `SignatureVerificationError::DisallowedAlg`.
- F2: A confidential verifier accepts an HS256-signed user info JWT when it
  holds a wrong secret.
- F3: An ES256-signed user info JWT with an altered signature verifies.
- F4: An ES256-signed user info JWT signed by a key outside the verifier's key
  set verifies.
- F5: A public verifier (no client secret) accepts an HS256-signed user info
  JWT even with `HmacSha256` explicitly allowed.
- F6: A method exists on `UserInfoVerifier` or `UserInfoRequest` that disables
  the algorithm allowlist (allow-any).

## Implementation

`cargo test --offline --lib -- verification::tests::test_user_info_signed_response_es256 verification::tests::test_user_info_signed_response_hs256 verification::tests::test_user_info_verified_claims user_info::tests::test_user_info_request_signed_response_policy` executes the fixture legs.
The default-algorithm and confidential-routing legs are additionally exercised
through the request builder in `src/user_info.rs`'s test module.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | ES256 accepted when explicitly allowed. | `src/verification/tests.rs::test_user_info_signed_response_es256` |
| P2 | HS256 accepted via confidential verifier + secret. | `src/verification/tests.rs::test_user_info_signed_response_hs256` |
| P3 | Issuer/audience/subject checks preserved on signed path. | `src/verification/tests.rs::test_user_info_verified_claims` |
| F1 | Default verifier rejects ES256 with DisallowedAlg. | `src/verification/tests.rs::test_user_info_signed_response_es256` |
| F2 | Wrong HS256 secret rejected. | `src/verification/tests.rs::test_user_info_signed_response_hs256` |
| F3 | Altered ES256 signature rejected. | `src/verification/tests.rs::test_user_info_signed_response_es256` |
| F4 | ES256 signature by wrong key rejected. | `src/verification/tests.rs::test_user_info_signed_response_es256` |
| F5 | Public verifier rejects allowed HS256 (no secret). | `src/verification/tests.rs::test_user_info_signed_response_hs256` |
| F6 | No allow-any method on the UserInfo surface. | manual (API surface inspection of `src/verification/mod.rs`, `src/user_info.rs`) |

## References

- Superseded for P-000008's complete declared Scope by
  situation/oracles/O-000018-judge-signed-user-info-verification-policy-complete-scope.md.
  W-000011 remains an observation of this historical, narrower rule.
