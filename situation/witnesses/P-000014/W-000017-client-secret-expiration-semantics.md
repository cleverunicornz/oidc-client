# W-000017 — Registration client-secret expiry semantics pass at the pre-publication gate

## Promise

situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md

## Oracle

situation/oracles/O-000015-judge-client-secret-expiration-semantics.md

## Result

PASS — the gate lib suite decided the registration-suite legs P1–P6, and
the compile leg P7 was executed by the gate's test-target build and by the
stream's recorded `cargo check --offline --tests` clean result.

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
- Source inspection at this head: every test named by O-000015's coverage
  table is a plain, non-`#[ignore]`d `#[test]` in
  `src/registration/tests.rs` —
  `test_client_secret_expiration_never_expires`,
  `test_client_secret_expiration_expires_at`,
  `test_client_secret_expiration_absent`,
  `test_client_secret_expiration_setter`, and the pre-existing suite
  including `test_response_serialization` with its migrated real-timestamp
  expectation and the wave-2 bearer/Content-Type lanes — so the gate's lib
  suite executed them all, and its 0-failure outcome decides P1–P6.
- P7: `cargo test --all-features --offline` builds every test target before
  running; the reported integration result (0 passed / 19 ignored) is only
  producible after a successful build of
  `tests/rp_certification_dynamic.rs`, so the compile leg executed at this
  head. Corroborating: the registration stream recorded
  `cargo check --offline --tests` clean at its implementing commit
  (`f93c8c7`), per D-000017's Evidence.
- Stream D (registration expiry) corroboration, recorded in P-000014's
  State evidence and D-000017's Evidence at commit `f93c8c7`:
  `cargo test --offline --lib registration` — 11 passed, 0 failed; the four
  new tests also pass under `--features accept-rfc3339-timestamps`; a
  falsification run with the zero-sentinel check disabled failed exactly
  `test_client_secret_expiration_never_expires`.
- Non-claims: no live-network flow was executed; the `#[ignore]`d
  certification tests compiled but never ran. `cargo deny check` was not
  run locally (cargo-deny is not installed on this host);
  dependency-policy assurance remains on the configured pull-request CI
  route per situation/promises/P-000005-configured-ci-gate-route.md and its
  witness situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
  — no deny pass is claimed here. The doctest result of 7 passed / 2
  ignored reflects two pre-existing `rust,ignore` blocks.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_client_secret_expiration_never_expires` deserialized numeric `0` to `Some(ClientSecretExpiration::NeverExpires)` and serialized it back to `0` (gate lib suite, 0 failures). |
| P2 | PASS — `test_client_secret_expiration_expires_at` mapped `1526545306` to the matching `ExpiresAt` with floor-rounding semantics; the migrated real-timestamp assertion in `test_response_serialization` agreed. |
| P3 | PASS — `test_client_secret_expiration_absent` deserialized the absent field to `None` and omitted it on serialization. |
| P4 | PASS — the round-trip halves of the three focused tests plus the response round trip in `test_response_serialization` verified stability for all three states. |
| P5 | PASS — `test_client_secret_expiration_setter` stored both states and read a stored `NeverExpires` back as `Some(&ClientSecretExpiration::NeverExpires)`, serializing to numeric `0`. |
| P6 | PASS — the pre-existing registration tests (including `test_response_serialization`, `test_metadata_serialization`, `test_metadata_serialization_minimal`, and the wave-2 lanes) passed unchanged within the gate lib suite. |
| P7 | PASS — the gate's test-target build compiled `tests/rp_certification_dynamic.rs` (integration suite reported 0 passed / 19 ignored, requiring a successful build); the stream's recorded `cargo check --offline --tests` clean at `f93c8c7` corroborates. |
