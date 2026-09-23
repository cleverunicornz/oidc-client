# W-000016 — JWK Debug secret redaction passes at the pre-publication gate

## Promise

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Oracle

situation/oracles/O-000014-judge-jwk-debug-redacts-secret-key-material.md

## Result

PASS — the parent's gate run executed the full `core::jwk` suite (the five
redaction tests and the pre-existing verification tests) at the head below
with no failures, deciding every O-000014 leg.

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
- Source inspection at this head: every test named by O-000014's coverage
  table is a plain, non-`#[ignore]`d `#[test]` under
  `src/core/jwk/tests.rs` — `test_core_jwk_debug_redacts_symmetric_secret`,
  `test_core_jwk_debug_redacts_ec_private_member`,
  `test_core_jwks_debug_redacts_symmetric_secret`,
  `test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes`,
  `test_core_jwk_rsa_verification_key_debug_has_no_private_material`, and
  the pre-existing verification tests (`test_ecdsa_verification`,
  `test_rsa_pkcs1_verification`, `test_rsa_pss_verification`,
  `test_hmac_sha256_verification`, `test_eddsa_verification`) — so the
  gate's lib suite executed them all, and its 0-failure outcome decides
  each leg.
- Stream C (jwk redaction) corroboration: P-000013's State evidence and
  D-000014's Evidence record the stream's focused battery —
  `cargo test --offline --lib core::jwk` passing 26 tests / 0 failed,
  `cargo check --offline` clean, and a falsification run against the
  pre-fix derived `Debug` failing exactly the four new redaction tests —
  at the stream's working head, cited in those records as `cdfe1ea`. That
  SHA does not resolve in this repository's history; the landed jwk stream
  commit carrying the same records is `5ab6d7b`. The discrepancy is
  recorded in
  situation/gaps/G-000029-stream-head-shas-unresolvable-on-branch.md and
  does not affect the legs above, which are decided at this head.
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
| P1 | PASS — `test_core_jwk_debug_redacts_symmetric_secret` found no representation of the canary secret in `{:?}` or `{:#?}` and the presence markers `k: Some([redacted])` / `[redacted]` (gate lib suite, 0 failures). |
| P2 | PASS — `test_core_jwk_debug_redacts_ec_private_member` found no representation of `d` in either format, with `d: Some([redacted])` and `k: None` markers. |
| P3 | PASS — `test_core_jwks_debug_redacts_symmetric_secret` found no representation of the contained symmetric secret in the key-set `Debug` output. |
| P4 | PASS — `test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes` verified identical `Debug` for keys differing only in `d`/`k`, unequal `PartialEq`, and serde round-trip preservation (with the `d` round-trip assertions of the EC test). |
| P5 | PASS — the pre-existing verification tests in `src/core/jwk/tests.rs` passed unchanged within the gate lib suite. |
| P6 | PASS — `test_core_jwk_rsa_verification_key_debug_has_no_private_material` verified the derived RSA verification key renders `d: None` and `k: None`. |
