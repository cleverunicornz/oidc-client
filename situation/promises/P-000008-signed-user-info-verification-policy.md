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

situation/oracles/O-000018-judge-signed-user-info-verification-policy-complete-scope.md

## State evidence

State `implemented` cites implementation commit
`78529bc948824f2dc3f6eb2270b2e9c5aa98876c`, which adds the scoped
UserInfo code and tests identified by O-000009. The Promise, Oracle, and
Witness records were attached retrospectively in
`9c4013a9d93a93e65084b3475022c43059e64fc2`; that is consistent with an
implementation record created after the behavior landed.

situation/oracles/O-000018-judge-signed-user-info-verification-policy-complete-scope.md
supersedes O-000009 for this Promise's complete Scope. Its first successor
observation,
situation/witnesses/P-000008/W-000020-signed-user-info-complete-scope-incomplete.md,
is INVALID: it preserves the absence of independent ES384, HS384, HS512,
request-builder, Client confidential-routing, claim-preservation, and
API-surface evidence.
The state therefore remains `implemented`, not `assured`. W-000011 remains
the PASS observation of the historical, narrower O-000009 rule.

## Residual

No assurance is claimed until O-000018 has a valid witness for every declared
Scope clause. W-000020 makes the currently unobserved ES384, HS384, HS512,
request-builder, Client confidential-routing, claim-preservation, and
API-surface legs visible;
that evidence boundary does not narrow this Promise. Algorithm allowlisting
for ID tokens is separately governed and not covered here.

## References

- src/verification/mod.rs (UserInfoVerifier: `new_confidential_client`,
  `set_allowed_algs`, `set_client_secret`)
- src/user_info.rs (UserInfoRequest builders; confidential routing in
  `user_info_impl`)
- src/verification/tests.rs::test_user_info_signed_response_es256
- src/verification/tests.rs::test_user_info_signed_response_hs256
- src/user_info.rs::test_user_info_request_signed_response_policy
- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
