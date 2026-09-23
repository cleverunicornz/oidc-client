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

State `implemented` cites the Stream B pre-publication-hardening commit on branch
`fix/prepublication-hardening` that carries this record together with the implementation and
the focused tests named by the oracle
(situation/decisions/D-000015-prepublication-hardening-maintenance-deltas-http-header-surface.md
authorizes the deltas). Witnesses have not been collected yet; the parent's final gate
attaches them under `situation/witnesses/P-000011/` and applies the oracle to decide the
disposition.

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
