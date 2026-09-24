# O-000021 — Judge malformed bearer-token complete scope

## State

implemented

## Judges

situation/promises/P-000011-malformed-bearer-token-fails-request-preparation.md

## Inputs

At the judged head, malformed and valid access tokens through the synchronous
and asynchronous UserInfo and dynamic-registration request paths, together
with recording HTTP clients that observe dispatch and Authorization headers.
This successor retains O-000012's malformed-token fixtures and separately
judges the previously unobserved valid-token asynchronous continuity.

## Pass

- P1 — A malformed UserInfo access token returns a request-preparation error on
  the synchronous path without dispatch.
- P2 — A malformed registration initial access token returns a
  request-preparation error on the synchronous path without dispatch.
- P3 — A malformed UserInfo access token returns the preparation error ready on
  the first asynchronous poll without dispatch.
- P4 — A malformed registration initial access token returns the preparation
  error ready on the first asynchronous poll without dispatch.
- P5 — Malformed-token errors never include any token bytes.
- P6 — A valid UserInfo access token produces `Authorization: Bearer <token>`
  on a synchronous dispatched request and its response parses normally.
- P7 — A valid registration initial access token produces
  `Authorization: Bearer <token>` on a synchronous dispatched request and its
  response parses normally.
- P8 — Registration without an initial access token dispatches no Authorization
  header.
- P9 — A valid UserInfo access token produces the same Bearer header and
  dispatches normally on the asynchronous path.
- P10 — A valid registration initial access token produces the same Bearer
  header and dispatches normally on the asynchronous path.

## Fail

- F1 — A panic escapes request preparation for a malformed token.
- F2 — A malformed token reaches either HTTP client.
- F3 — A malformed-token error includes token bytes.
- F4 — A malformed token is stripped, replaced, or silently omitted while the
  request proceeds.
- F5 — A valid synchronous request has a missing or non-Bearer Authorization
  header, or tokenless registration sends one.
- F6 — A valid asynchronous request has a missing or non-Bearer Authorization
  header, or does not preserve the normal dispatch path.

## Implementation

`cargo test --offline --lib --quiet -- malformed` executes the retained
malformed-token fixtures. The existing valid synchronous fixtures and the
structural async-continuity decisions are named below; an independent valid
async request observation is still required for a complete witness.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Malformed synchronous UserInfo request errors before dispatch. | `src/user_info.rs::test_user_info_request_malformed_access_token_fails_without_dispatch` |
| P2 | Malformed synchronous registration request errors before dispatch. | `src/registration/tests.rs::test_registration_malformed_initial_access_token_errors_before_dispatch` |
| P3 | Malformed asynchronous UserInfo request errors before dispatch. | `src/user_info.rs::test_user_info_request_async_malformed_access_token_fails_without_dispatch` |
| P4 | Malformed asynchronous registration request errors before dispatch. | `src/registration/tests.rs::test_registration_async_malformed_initial_access_token_errors_before_dispatch` |
| P5 | No malformed token bytes appear in errors. | `src/http_utils.rs::test_auth_bearer_rejects_malformed_token_without_echo` and no-echo assertions in P1–P4 fixtures |
| P6 | Valid synchronous UserInfo request has a Bearer header. | `src/user_info.rs::test_user_info_request_sends_bearer_header` |
| P7 | Valid synchronous registration request has a Bearer header. | `src/registration/tests.rs::test_registration_sends_bearer_header_and_accepts_ows_content_type` |
| P8 | Tokenless registration has no Authorization header. | `src/registration/tests.rs::test_registration_without_access_token_sends_no_authorization_header` |
| P9 | Valid async UserInfo preserves preparation and dispatch. | manual — `src/user_info.rs::UserInfoRequest::request_async` calls `prepare_request` before `AsyncHttpClient::call` |
| P10 | Valid async registration preserves preparation and dispatch. | manual — `src/registration/mod.rs::ClientRegistrationRequest::register_async` calls `prepare_registration` before `AsyncHttpClient::call` |
| F1 | Malformed preparation does not panic. | P1–P4 fixtures fail on an escaping panic |
| F2 | Malformed request does not dispatch. | P1–P4 recording-client assertions |
| F3 | Errors do not echo tokens. | P5 fixtures |
| F4 | Error propagates rather than stripping credentials. | manual — `src/http_utils.rs::auth_bearer` and both preparation methods |
| F5 | Synchronous Authorization behavior is exact. | P6–P8 fixtures |
| F6 | Async Authorization behavior is exact. | manual — `request_async` and `register_async` shared preparation paths |

## References

- Supersedes O-000012 for P-000011's complete declared scope; W-000014
  remains an observation of the historical, narrower rule.
- situation/witnesses/P-000011/W-000023-malformed-bearer-complete-scope-incomplete.md
