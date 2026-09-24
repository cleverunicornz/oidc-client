# W-000014 — Malformed bearer tokens fail request preparation at the pre-publication gate

## Promise

situation/promises/P-000011-malformed-bearer-token-fails-request-preparation.md

## Oracle

situation/oracles/O-000012-judge-malformed-bearer-token-fails-request-preparation.md

## Result

PASS — the parent's gate run executed every named O-000012 leg at the head
below with no failures, and the manual F4 leg holds by inspection of
`auth_bearer` at the same head.

## Head

b96b920f52e0d8b392edcf0b5d752fa570c2b356

## Observed

2026-09-23

## Evidence

- Gate run (observed by the parent orchestrator at a working tree equal to
  this head; local Linux, rustc/cargo 1.98.0, offline): `cargo fmt --all
  --check` clean; `cargo clippy --all-targets -- -D warnings` clean,
  `Finished \`dev\` profile`; `cargo test --all-features --offline` lib
  suite 99 passed / 0 failed / 0 ignored in run, integration suite 0 passed
  / 19 ignored (the 19 live-network certification tests were `#[ignore]`d
  pre-existing), doctests 7 passed / 2 ignored (pre-existing);
  `RUSTDOCFLAGS="-D warnings" cargo doc --offline --all-features
  --no-deps` clean. No artifact was retained from that run; these results
  are recorded here from the parent's gate report.
- Source inspection at this head: every test named by O-000012's coverage
  table is a plain, non-`#[ignore]`d `#[test]` in the lib target —
  `src/user_info.rs::test_user_info_request_malformed_access_token_fails_without_dispatch`,
  `::test_user_info_request_async_malformed_access_token_fails_without_dispatch`,
  `::test_user_info_request_sends_bearer_header`;
  `src/registration/tests.rs::test_registration_malformed_initial_access_token_errors_before_dispatch`,
  `::test_registration_async_malformed_initial_access_token_errors_before_dispatch`,
  `::test_registration_sends_bearer_header_and_accepts_ows_content_type`,
  `::test_registration_without_access_token_sends_no_authorization_header`;
  `src/http_utils.rs::test_auth_bearer_rejects_malformed_token_without_echo`,
  `::test_auth_bearer_builds_authorization_header` — so the gate's lib
  suite executed them all, and its 0-failure outcome decides each
  executable leg.
- F4 (manual): inspection of `src/http_utils.rs::auth_bearer` at this head
  — the function returns the `Authorization: Bearer` header pair from
  `HeaderValue::from_str` or propagates the `InvalidHeaderValue`
  construction error as `Err`; no stripping, replacement, or silent
  omission path exists.
- The Fail legs F1–F3 and F5 are embedded negative assertions of the same
  named tests per O-000012's coverage table, so the gate's 0-failure lib
  run decides them; no Fail condition occurred.
- Stream B (bearer/content-type) landed its records and implementation in
  commit `60c1163`; D-000015's Evidence records its focused run
  `cargo test --offline --lib -- http_utils user_info registration
  verification` — 36 passed, 0 failed, including these lanes without
  regression.
- Non-claims: no live-network flow was executed. `cargo deny check` was not
  run locally (cargo-deny is not installed on this host);
  dependency-policy assurance remains on the configured pull-request CI
  route per situation/promises/P-000005-configured-ci-gate-route.md and its
  witness situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
  — no deny pass is claimed here. The doctest result of 7 passed / 2
  ignored reflects two pre-existing `rust,ignore` blocks; the 19
  live-network certification tests remain `#[ignore]`d, pre-existing.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_user_info_request_malformed_access_token_fails_without_dispatch` returned the request-preparation error on the synchronous path with no recorded dispatch (gate lib suite, 0 failures). |
| P2 | PASS — `test_registration_malformed_initial_access_token_errors_before_dispatch` errored before any registration dispatch. |
| P3 | PASS — the async pair (`test_user_info_request_async_malformed_access_token_fails_without_dispatch`, `test_registration_async_malformed_initial_access_token_errors_before_dispatch`) returned the preparation error ready on first poll with no dispatch. |
| P4 | PASS — `test_auth_bearer_rejects_malformed_token_without_echo` plus the no-echo assertions inside the P1–P3 tests verified no token bytes in error strings. |
| P5 | PASS — `test_auth_bearer_builds_authorization_header`, `test_user_info_request_sends_bearer_header`, and `test_registration_sends_bearer_header_and_accepts_ows_content_type` verified the `Authorization: Bearer <token>` header on dispatched requests. |
| P6 | PASS — `test_registration_without_access_token_sends_no_authorization_header` verified the tokenless registration dispatches no Authorization header. |
