# W-000015 — Content-Type essence matching passes at the pre-publication gate

## Promise

situation/promises/P-000012-content-type-essence-matches-rfc7231-optional-whitespace.md

## Oracle

situation/oracles/O-000013-judge-content-type-essence-matching.md

## Result

PASS — the parent's gate run executed every named O-000013 leg at the head
below with no failures; every leg of this oracle is executable and all are
decided by the gate run.

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
- Source inspection at this head: every test named by O-000013's coverage
  table is a plain, non-`#[ignore]`d `#[test]` in the lib target —
  `src/http_utils.rs::test_content_type_has_essence_accepts_case_and_optional_whitespace`,
  `::test_content_type_has_essence_rejects_unexpected_and_malformed_values`,
  `::test_check_content_type_tolerates_missing_header_and_accepts_ows`;
  `src/registration/tests.rs::test_registration_sends_bearer_header_and_accepts_ows_content_type`;
  `src/user_info.rs::test_user_info_response_routes_on_content_type_with_optional_whitespace`
  — so the gate's lib suite executed them all, and its 0-failure outcome
  decides each leg.
- The Fail legs F1–F3 are embedded negative assertions of the same named
  tests per O-000013's coverage table, so the gate's 0-failure lib run
  decides them; no Fail condition occurred.
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
| P1 | PASS — `test_content_type_has_essence_accepts_case_and_optional_whitespace` accepted `application/json`, `Application/JSON`, `application/json ; charset=utf-8`, `application/json;charset=utf-8`, and `\tapplication/json\t` (gate lib suite, 0 failures). |
| P2 | PASS — `test_content_type_has_essence_rejects_unexpected_and_malformed_values` rejected `application/jsonx`, `application/jwt`, `appl ication/json`, and `text/plain`. |
| P3 | PASS — the same test rejected a non-UTF-8 `HeaderValue`. |
| P4 | PASS — `test_check_content_type_tolerates_missing_header_and_accepts_ows` covered tolerance, acceptance, and rejection on `check_content_type`; `test_registration_sends_bearer_header_and_accepts_ows_content_type` accepted `APPLICATION/JSON ; charset=UTF-8` on the public registration path. |
| P5 | PASS — `test_user_info_response_routes_on_content_type_with_optional_whitespace` parsed JSON claims under `APPLICATION/JSON ; charset=utf-8` and routed `application/jwt\t` to JWT verification. |
