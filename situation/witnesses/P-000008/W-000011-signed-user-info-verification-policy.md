# W-000011 — Signed-UserInfo verification policy passes at the pre-publication gate

## Promise

situation/promises/P-000008-signed-user-info-verification-policy.md

## Oracle

situation/oracles/O-000009-judge-signed-user-info-verification-policy.md

## Result

PASS — the parent's gate run executed every named O-000009 leg at the head
below with no failures, and the manual F6 leg holds by API-surface
inspection at the same head.

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
- Source inspection at this head: every test named by O-000009's coverage
  table is a plain, non-`#[ignore]`d `#[test]` in the lib target —
  `src/verification/tests.rs::test_user_info_signed_response_es256`,
  `::test_user_info_signed_response_hs256`,
  `::test_user_info_verified_claims`, and
  `src/user_info.rs::test_user_info_request_signed_response_policy` — so
  the gate's lib suite executed them all, and its 0-failure outcome decides
  each executable leg.
- F6 (manual): inspection of the public surface at this head —
  `allow_any_alg` is defined only on `JwtClaimsVerifier` and
  `IdTokenVerifier` in `src/verification/mod.rs`, both outside this
  promise's Scope; `UserInfoVerifier` (`src/verification/mod.rs`) and
  `UserInfoRequest` (`src/user_info.rs`) expose only `set_allowed_algs`.
  D-000013 §2 records the deliberate exclusion.
- The Fail legs F1–F5 are embedded negative assertions of the same named
  tests per O-000009's coverage table, so the gate's 0-failure lib run
  decides them; no Fail condition occurred.
- Stream A (UserInfo policy) landed this record together with the
  implementation and the focused tests in commit `78529bc`; it recorded no
  separate focused-result numbers. D-000013's Evidence additionally records
  post-change regression probes
  (`src/verification/tests.rs::test_es256_id_token_verified_claims`;
  `src/core/jwk/tests.rs::test_hmac_sha256_verification`,
  `::test_ecdsa_verification`) passing.
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
| P1 | PASS — `test_user_info_signed_response_es256` verified the ES256-signed user info JWT with `EcdsaP256Sha256` explicitly allowed (gate lib suite, 0 failures). |
| P2 | PASS — `test_user_info_signed_response_hs256` verified the HS256-signed user info JWT through a confidential verifier holding the client secret, against an empty JWKS. |
| P3 | PASS — `test_user_info_verified_claims` enforced issuer, audience, and expected-subject checks on the signed path; `test_user_info_request_signed_response_policy` exercised the default-algorithm and confidential-routing legs through the request builder. |
