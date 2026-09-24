# W-000030 — Malformed bearer-token complete-scope observation passes

## Promise

situation/promises/P-000011-malformed-bearer-token-fails-request-preparation.md

## Oracle

situation/oracles/O-000021-judge-malformed-bearer-token-complete-scope.md

## Result

PASS — every oracle leg is decided at this head. The retained `malformed`
battery executes the four malformed-token fixtures and the no-echo helper
fixture; the valid synchronous fixtures execute through the
`user_info_request` and `registration::` batteries; and the previously
unobserved valid-token asynchronous continuity (P9/P10) is now
fixture-executed by `test_user_info_request_async_sends_bearer_header` and
`test_registration_async_sends_bearer_header`, superseding the oracle's manual
route for those two legs. The shared-preparation structural decisions (F4/F6)
are additionally decided by code inspection named below.

## Head

d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000030/cargo-test-malformed.log` retains
  `cargo test --offline --lib --quiet -- malformed` (`exit=0`): `test result:
  ok. 6 passed; 0 failed; 0 ignored; 0 measured; 104 filtered out`. The
  selected tests are `test_user_info_request_malformed_access_token_fails_without_dispatch`,
  `test_user_info_request_async_malformed_access_token_fails_without_dispatch`,
  `test_registration_malformed_initial_access_token_errors_before_dispatch`,
  `test_registration_async_malformed_initial_access_token_errors_before_dispatch`,
  `test_auth_bearer_rejects_malformed_token_without_echo`, and (name match)
  `test_content_type_has_essence_rejects_unexpected_and_malformed_values`.
  SHA-256
  `a0ba383c6a4616d9cb121a4cbb94a3fcba16ad652c697961f660209ecfe411df`.
- `situation/witnesses/evidence/W-000027/cargo-test-user-info-request.log`
  (same battery as W-000027; run once at this head) retains
  `cargo test --offline --lib --quiet -- user_info_request` (`exit=0`):
  `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 104 filtered
  out`, including `test_user_info_request_sends_bearer_header` and
  `test_user_info_request_async_sends_bearer_header`. SHA-256
  `f79d975877dbeb627e0cb57ac4601910f4549c4a809496b959088b9f53d0435b`.
- `situation/witnesses/evidence/W-000030/cargo-test-registration.log` retains
  `cargo test --offline --lib --quiet -- registration::` (`exit=0`):
  `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 94 filtered
  out`, including `test_registration_sends_bearer_header_and_accepts_ows_content_type`,
  `test_registration_async_sends_bearer_header`, and
  `test_registration_without_access_token_sends_no_authorization_header`.
  SHA-256
  `76dce5568d13b6f486e231e2f52ef91a52f0579226a800a60d8adb16aa9ea0de`.
- Digests are recorded in the adjacent `SHA256SUMS` files and were re-verified
  with `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness observes one promise under one oracle at one head;
  promise state transitions remain with the closure corrector.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_user_info_request_malformed_access_token_fails_without_dispatch` gets a ready request-preparation error on the synchronous path with an empty dispatch log (`src/user_info.rs` lines 893-939; preparation entry `prepare_request` at lines 190-204). |
| P2 | PASS — `test_registration_malformed_initial_access_token_errors_before_dispatch` errors before any dispatch (`src/registration/tests.rs` lines 981-1024; preparation at `src/registration/mod.rs` lines 595-601 maps the `auth_bearer` error into `ClientRegistrationError::Other("failed to prepare request: ...")` before the request builder runs). |
| P3 | PASS — `test_user_info_request_async_malformed_access_token_fails_without_dispatch` returns the preparation error ready on the first poll with a no-op waker and no dispatch (`src/user_info.rs` lines 944-996). |
| P4 | PASS — `test_registration_async_malformed_initial_access_token_errors_before_dispatch` returns `Ready(Err(ClientRegistrationError::Other(...)))` on the first poll, asserts the message contains `failed to prepare request`, and asserts the dispatch log is empty (`src/registration/tests.rs` lines 1025-1061; ordering guaranteed by `register_async`'s `self.prepare_registration(...)?` before the call at `src/registration/mod.rs` lines 574-581). |
| P5 | PASS — `test_auth_bearer_rejects_malformed_token_without_echo` asserts the `auth_bearer` error string does not contain the token fragment (`src/http_utils.rs` lines 125-133), and all four request fixtures assert their error messages never contain the token fragment (e.g. `src/registration/tests.rs` lines 1050-1053). |
| P6 | PASS — `test_user_info_request_sends_bearer_header` produces `Authorization: Bearer the_access_token` on the dispatched synchronous request (`src/user_info.rs` lines 997-1044). |
| P7 | PASS — `test_registration_sends_bearer_header_and_accepts_ows_content_type` asserts the dispatched registration request carries `Authorization: Bearer the_access_token` (`src/registration/tests.rs` lines 1109-1146, header assertion at 1142-1145). |
| P8 | PASS — `test_registration_without_access_token_sends_no_authorization_header` dispatches with no Authorization header (fixture at `src/registration/tests.rs` lines 1149-1183; structural cause is the `if let Some((header, value)) = auth_header_opt` guard at `src/registration/mod.rs` lines 608-610). |
| P9 | PASS — fixture-executed (superseding the oracle's manual route): `test_user_info_request_async_sends_bearer_header` polls `request_async` once with a no-op waker, gets a ready success, and asserts the dispatched request carries exactly `Bearer the_access_token` (`src/user_info.rs` lines 1185-1231, header assertion at 1227-1230). |
| P10 | PASS — fixture-executed (superseding the oracle's manual route): `test_registration_async_sends_bearer_header` polls `register_async` once, gets a ready success, and asserts the dispatched request carries exactly `Bearer the_access_token` (`src/registration/tests.rs` lines 1067-1104, header assertion at 1100-1103). |
| F1 | PASS (negative executed) — none of the four malformed fixtures panic: each asserts a ready `Err` outcome (`src/user_info.rs` lines 893-996; `src/registration/tests.rs` lines 981-1061), and `auth_bearer` returns `Result` rather than panicking (`src/http_utils.rs` lines 61-66). |
| F2 | PASS (negative executed) — every malformed fixture asserts the recording client's dispatch log stays empty (`src/user_info.rs` lines 893-996; `src/registration/tests.rs` lines 1057-1060). |
| F3 | PASS (negative executed) — the no-echo assertions in P5 execute on all five error surfaces. |
| F4 | PASS (negative structural) — `auth_bearer` returns the header-construction error instead of stripping or substituting (`src/http_utils.rs` lines 61-66); `prepare_request` propagates it via `?` before building the request (`src/user_info.rs` lines 190-192), and `prepare_registration` propagates it before the builder and before any dispatch (`src/registration/mod.rs` lines 595-601); the sync/async callers surface the error with `?` (`src/user_info.rs` lines 156-158, 180-182; `src/registration/mod.rs` lines 549-555, 574-575). A malformed token therefore errors the whole call; the request never proceeds. |
| F5 | PASS (negative executed) — the valid synchronous fixtures assert the exact header value on dispatch (P6/P7) and the tokenless fixture asserts its absence (P8); no fixture observes a missing or non-Bearer header on a valid token, and the value is built solely by `format!("{} {}", BEARER, token)` (`src/http_utils.rs` line 64). |
| F6 | PASS (negative executed and structural) — the valid asynchronous fixtures assert the same exact header (P9/P10), and both async paths share the synchronous preparation methods (`request_async` and `register_async` call the same `prepare_request`/`prepare_registration` before their first await: `src/user_info.rs` lines 178-188, `src/registration/mod.rs` lines 574-581), so the Authorization behavior cannot diverge between sync and async dispatch. |
