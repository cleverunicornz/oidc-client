# P-000011 — Malformed bearer tokens fail request preparation

## State

implemented

## Promise

A provider-issued access token that cannot form an HTTP header value (for example one
containing a newline or other control byte) never panics and never reaches HTTP dispatch.
Both the user info flow (`UserInfoRequest::request` / `UserInfoRequest::request_async` via
`prepare_request`) and the dynamic registration flow
(`ClientRegistrationRequest::register` / `ClientRegistrationRequest::register_async` via
`prepare_registration`, when an initial access token is set) return a normal
request-preparation error. The error names the failed request preparation and never
includes the token bytes. Credentials are never stripped, replaced, or silently omitted: a
malformed token is always an error. Valid tokens continue to produce the
`Authorization: Bearer <token>` header on dispatched requests, and registration without an
initial access token continues to send no Authorization header.

## Scope

`auth_bearer` header construction in `src/http_utils.rs` and its two consumers,
`UserInfoRequest::prepare_request` in `src/user_info.rs` and
`ClientRegistrationRequest::prepare_registration` in `src/registration/mod.rs`, on both the
synchronous and asynchronous paths. Outside Scope: token issuance and serialization
(`AccessToken` itself), other Authorization-header producers elsewhere in the crate, and
HTTP client behavior after a well-formed request has been prepared.

## Oracle

situation/oracles/O-000012-judge-malformed-bearer-token-fails-request-preparation.md

## State evidence

State `implemented` cites implementation commit
`60c11639afc4d0b7504d7338c1a810f8c38eee30`, which changes
`auth_bearer`, its two consumers, and the focused tests named by O-000012.
The Promise, Oracle, and Witness records were attached retrospectively in
`9c4013a9d93a93e65084b3475022c43059e64fc2`.

situation/witnesses/P-000011/W-000014-malformed-bearer-token-preparation-errors.md
is a PASS observation from the parent's final gate at
`b96b920f52e0d8b392edcf0b5d752fa570c2b356` and decides each named
O-000012 leg. The state remains `implemented`, not `assured`: the failure
paths are observed asynchronously, but valid-token asynchronous dispatch has
no independently recorded coverage. That limit remains in Residual.

## Residual

The asynchronous happy paths are not separately exercised; the async tests cover the
malformed-token failure (where preparation must precede dispatch) and both async paths
funnel through the same `prepare_request` / `prepare_registration` the synchronous tests
exercise. Invalidity classes beyond `HeaderValue` parsing (for example an empty bearer
token) are valid header constructions and are not treated as errors.

## References

- src/http_utils.rs (`auth_bearer`; tests module)
- src/user_info.rs (`UserInfoRequest::prepare_request`;
  `tests::test_user_info_request_malformed_access_token_fails_without_dispatch`,
  `tests::test_user_info_request_async_malformed_access_token_fails_without_dispatch`,
  `tests::test_user_info_request_sends_bearer_header`)
- src/registration/tests.rs (`test_registration_malformed_initial_access_token_errors_before_dispatch`,
  `test_registration_async_malformed_initial_access_token_errors_before_dispatch`,
  `test_registration_sends_bearer_header_and_accepts_ows_content_type`,
  `test_registration_without_access_token_sends_no_authorization_header`)
- situation/decisions/D-000015-prepublication-hardening-maintenance-deltas-http-header-surface.md
