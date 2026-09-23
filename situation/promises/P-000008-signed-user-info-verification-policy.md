# P-000008 — Signed-UserInfo verification policy

## State

implemented

## Promise

A consumer verifying a signed user info JWT response controls the accepted
signature algorithms through an explicit allowlist configured on
`UserInfoVerifier` (`set_allowed_algs`) or on the request builder
(`UserInfoRequest::set_allowed_algs`). The default allowlist contains only
`RS256`. `ES256`, `ES384`, and the `HS*` algorithms verify only when explicitly
allowed. `HS*` verification additionally requires confidential-client
credentials: the verifier must hold the client secret (via
`UserInfoVerifier::new_confidential_client` or `UserInfoRequest::set_client_secret`),
whose UTF-8 octets are the shared secret. No public API enables accepting any
algorithm on this surface. Issuer, audience, and expected-subject checks remain
enforced for signed responses.

## Scope

The signed-response user info verification path: `UserInfoVerifier` and the
`UserInfoRequest` builders (`set_allowed_algs`, `set_client_secret`) and the
confidential routing performed when a `Client` holding a client secret issues a
user info request. This promise does not cover unsigned JSON responses, the
generic JWT layer, ID-token verification policy, or signature production.

## Oracle

situation/oracles/O-000009-judge-signed-user-info-verification-policy.md

## State evidence

State `implemented` cites the Stream A pre-publication-hardening commit on
branch `fix/prepublication-hardening` that carries this record together with
the implementation and the focused tests named by the oracle
(situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
authorizes the deltas). Witnesses have not been collected yet; the parent's
final gate attaches them under `situation/witnesses/P-000008/` and applies the
oracle to decide the disposition.

## Residual

`ES384`, `HS384`, and `HS512` follow the same dispatch as the tested `ES256`
and `HS256` but are not exercised by a named test. Asynchronous request
submission is not distinguished from the synchronous path (both funnel through
the same `user_info_response` verification). Algorithm allowlisting for ID
tokens is separately governed and not covered here.

## References

- src/verification/mod.rs (UserInfoVerifier: `new_confidential_client`,
  `set_allowed_algs`, `set_client_secret`)
- src/user_info.rs (UserInfoRequest builders; confidential routing in
  `user_info_impl`)
- src/verification/tests.rs::test_user_info_signed_response_es256
- src/verification/tests.rs::test_user_info_signed_response_hs256
- src/user_info.rs::test_user_info_request_signed_response_policy
- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
