# O-000012 — Judge malformed bearer token request preparation

## State

implemented

## Judges

situation/promises/P-000011-malformed-bearer-token-fails-request-preparation.md

## Inputs

The scoped Cargo test command below over `src/http_utils.rs`, `src/user_info.rs`, and
`src/registration/tests.rs`, using recording mock HTTP clients (`Fn(HttpRequest) ->
Result<HttpResponse, E>` closures, with future-returning variants for the async paths) that
retain every dispatched request. The async paths are polled once with a no-op waker; request
preparation runs before the first await, so the outcome is ready without an executor.

## Pass

- P1: A user info request whose access token contains a control character returns
  `UserInfoError::Other` on the public synchronous path with a message naming the failed
  request preparation, and the HTTP client records no dispatch.
- P2: A registration request with a malformed initial access token returns
  `ClientRegistrationError::Other` before any dispatch.
- P3: The asynchronous user info and registration paths return the preparation error ready
  on first poll, with no dispatch.
- P4: The error strings produced for malformed tokens do not contain the token bytes.
- P5: A valid access token produces `Authorization: Bearer <token>` on the dispatched user
  info and registration requests, and the responses parse normally.
- P6: A registration request without an initial access token dispatches no Authorization
  header.

## Fail

- F1: A panic (unwind) escapes from user info or registration request preparation when the
  access token cannot form a header value.
- F2: The HTTP client records a dispatch for a request whose bearer token is malformed.
- F3: An error message for a malformed token contains any part of the token bytes.
- F4: A malformed token is silently stripped, replaced, or omitted while the request
  proceeds.
- F5: A valid token produces a missing or non-Bearer Authorization header, or a tokenless
  registration emits one.

## Implementation

`cargo test --offline --lib -- http_utils user_info registration verification` executes the
executable legs.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Malformed token errors on the user info path without dispatch. | `src/user_info.rs::test_user_info_request_malformed_access_token_fails_without_dispatch` |
| P2 | Malformed initial access token errors before registration dispatch. | `src/registration/tests.rs::test_registration_malformed_initial_access_token_errors_before_dispatch` |
| P3 | Async paths return the ready preparation error without dispatch. | `src/user_info.rs::test_user_info_request_async_malformed_access_token_fails_without_dispatch`, `src/registration/tests.rs::test_registration_async_malformed_initial_access_token_errors_before_dispatch` |
| P4 | No token bytes in error strings. | `src/http_utils.rs::test_auth_bearer_rejects_malformed_token_without_echo` plus the no-echo assertions inside the P1–P3 tests |
| P5 | Valid tokens produce the Bearer header. | `src/http_utils.rs::test_auth_bearer_builds_authorization_header`, `src/user_info.rs::test_user_info_request_sends_bearer_header`, `src/registration/tests.rs::test_registration_sends_bearer_header_and_accepts_ows_content_type` |
| P6 | Tokenless registration sends no Authorization header. | `src/registration/tests.rs::test_registration_without_access_token_sends_no_authorization_header` |
| F1 | No panic escapes preparation. | the P1–P3 tests (a panic fails the test instead of yielding `Err`) |
| F2 | No dispatch with a malformed token. | the P1–P3 tests' empty recording assertions |
| F3 | No echo (failure direction). | the same tests as P4 |
| F4 | No silent credential stripping or omission. | manual (code inspection of `src/http_utils.rs::auth_bearer`: the construction error maps to `Err`; no replacement or omission path exists) |
| F5 | Bearer header exactness both directions. | the same tests as P5 and P6 |

## References

- Superseded for P-000011's complete declared Scope by
  situation/oracles/O-000021-judge-malformed-bearer-token-complete-scope.md.
  W-000014 remains an observation of this historical, narrower rule.
