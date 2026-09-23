# W-000013 — UserInfo subject binding passes at the pre-publication gate

## Promise

situation/promises/P-000010-user-info-flows-bind-verified-id-token-subject.md

## Oracle

situation/oracles/O-000011-judge-user-info-subject-binding.md

## Result

PASS — the gate lib suite decided the fixture legs, the gate doctest run
decided the tutorial half of P5, and a retained compile observation at the
same head decided the `gitlab` example half of P5.

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
- P1–P4 and F1: `src/verification/tests.rs::test_user_info_subject_binding`
  is a plain, non-`#[ignore]`d lib test at this head, so the gate's lib
  suite executed it; its passing outcome decides those legs (matching JSON
  and signed subjects accepted; mismatched subjects on both paths rejected
  with `ClaimsVerificationError::InvalidSubject`).
- P5, tutorial half: the gate doctest run (7 passed / 2 ignored, produced
  under `--all-features`) compiled the synchronous PKCE tutorial, whose
  body passes the verified ID-token subject to `user_info`
  (`Some(claims.subject().clone())`, with the OIDC Core 5.3.2 note that the
  optional `at_hash` check does not substitute for the binding).
- P5, `gitlab` example half: this closure run executed
  `cargo check --offline --example gitlab --features reqwest-blocking` at
  this head on 2026-09-23 (local Linux, rustc/cargo 1.98.0, offline):
  `Checking oidc-client v4.1.0` … `Finished \`dev\` profile`, `exit=0`.
  `situation/witnesses/evidence/W-000013/gitlab-example-check-reqwest-blocking.log`
  retains the verbatim output, and
  `situation/witnesses/evidence/W-000013/SHA256SUMS` records SHA-256
  `b71171d8ad92c79d72776ba02148ae98d5aae16246606ceff619b3d68ad3972f` for
  it. The example passes the verified subject at its `user_info` call
  (`examples/gitlab.rs`, `Some(id_token_claims.subject().clone())` with the
  OIDC Core 5.3.2 comment). The gate's `cargo clippy --all-targets` did not
  cover this target: `Cargo.toml` declares
  `required-features = ["reqwest-blocking"]` for the `gitlab` example and
  the gate clippy ran with default features, which is why a separate
  retained observation supplies this leg.
- The Fail leg F1 is an embedded negative assertion of
  `test_user_info_subject_binding` per O-000011's coverage table, decided
  by the same gate run; no Fail condition occurred.
- Stream A (UserInfo policy) landed this record together with the
  implementation and the focused test in commit `78529bc`; it recorded no
  separate focused-result numbers.
- Non-claims: no live-network flow was executed; the tutorial leg is
  `no_run` compilation and the `gitlab` observation is a compile-only
  check. `cargo deny check` was not run locally (cargo-deny is not
  installed on this host); dependency-policy assurance remains on the
  configured pull-request CI route per
  situation/promises/P-000005-configured-ci-gate-route.md and its witness
  situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md — no
  deny pass is claimed here. The 19 live-network certification tests remain
  `#[ignore]`d, pre-existing.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_user_info_subject_binding` accepted the JSON response whose `sub` matches the expected subject (gate lib suite, 0 failures). |
| P2 | PASS — the same test rejected the mismatched-`sub` JSON response with `ClaimsVerificationError::InvalidSubject`. |
| P3 | PASS — the same test accepted the signed user info JWT whose `sub` matches the expected subject. |
| P4 | PASS — the same test rejected the mismatched-`sub` signed JWT with `ClaimsVerificationError::InvalidSubject` after signature verification succeeded. |
| P5 | PASS — the synchronous tutorial compiled against the bound call in the gate doctest run (7 passed / 2 ignored, `--all-features`); the `gitlab` example compiled with the bound call in the retained `cargo check --offline --example gitlab --features reqwest-blocking` observation at this head (`exit=0`, retained log). |
