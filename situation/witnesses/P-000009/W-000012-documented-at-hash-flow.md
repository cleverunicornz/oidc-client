# W-000012 — Documented at_hash flow passes at the pre-publication gate

## Promise

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Oracle

situation/oracles/O-000010-judge-documented-at-hash-flow.md

## Result

PASS — the gate lib suite decided the fixture legs, and the gate doctest
run decided the documentation leg with both crate-root tutorials compiling
against the documented call under `--all-features`.

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
- P1, P2, F1, F2: `src/verification/tests.rs::test_id_token_verification_key_at_hash`
  is a plain, non-`#[ignore]`d lib test at this head, so the gate's lib
  suite executed it; its passing outcome decides those legs (HS256
  confidential flow with empty JWKS; ES256 JWKS key resolution; substituted
  token rejected; missing-secret errors instead of panicking).
- P3: the gate's doctest result was 7 passed / 2 ignored, produced under
  `--all-features` (so the `reqwest-blocking` feature was enabled). The
  doctest inventory at this head is exactly seven compiled blocks — five
  `rust,no_run` blocks in `src/lib.rs` (including the synchronous PKCE
  tutorial, "Getting started: Authorization Code Grant w/ PKCE", and the
  asynchronous tutorial) plus one `rust` block each in `src/jwt/mod.rs` and
  `src/client.rs` — and the two pre-existing `rust,ignore` blocks in
  `src/lib.rs`. The reported outcome therefore includes both tutorials
  compiling against the documented call: both tutorial bodies call
  `id_token.verification_key(&id_token_verifier)` feeding
  `AccessTokenHash::from_token`.
- The Fail legs F1–F2 are embedded negative assertions of
  `test_id_token_verification_key_at_hash` per O-000010's coverage table,
  decided by the same gate run; no Fail condition occurred.
- Stream A (UserInfo policy) landed this record together with the
  implementation and the focused test in commit `78529bc`; it recorded no
  separate focused-result numbers, so the gate run above decides every leg.
- Non-claims: no live-network flow was executed; the doctest leg compiles
  the tutorials (`no_run`), it does not execute them against a provider.
  `cargo deny check` was not run locally (cargo-deny is not installed on
  this host); dependency-policy assurance remains on the configured
  pull-request CI route per
  situation/promises/P-000005-configured-ci-gate-route.md and its witness
  situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md — no
  deny pass is claimed here. The 19 live-network certification tests remain
  `#[ignore]`d, pre-existing.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_id_token_verification_key_at_hash` resolved the client-secret key for the HS256 token through the confidential verifier with an empty JWKS and reproduced the token's `at_hash` (gate lib suite, 0 failures). |
| P2 | PASS — the same test resolved the provider's EC P-256 JWK from the JWKS for the ES256 token and reproduced the access-token hash. |
| P3 | PASS — the gate doctest run (7 passed / 2 ignored, `--all-features`) compiled both crate-root tutorials against `IdToken::verification_key`. |
