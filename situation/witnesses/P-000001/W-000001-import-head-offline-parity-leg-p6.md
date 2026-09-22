# W-000001 — Import-head offline suite ran; complete observation invalid

## Promise

situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md

## Oracle

situation/oracles/O-000001-judge-full-oidc-rp-flow-parity-with-upstream.md

## Result

INVALID — the real-provider Pass legs P1–P5 were not exercised. The retained
test run passed, but it cannot decide historical O-000001 P6 because that rule
required an unchanged suite except for import renames.

## Head

24835e4b44caa8a0baae2ac5b865bbd6bdf355ba

## Observed

2026-09-22

## Evidence

- `situation/witnesses/evidence/W-000001/gates-final.log` is the retained
  pinned-container run at the head above. It records `cargo fmt --all --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test --all-features`,
  and `cargo deny check` succeeding.
- The `cargo test --all-features` output in that log records 70 passed and
  0 failed tests. It names `test_core_jwk_deserialization_ec` and
  `test_ecdsa_verification` among the passing tests; the 21 live-network
  certification tests compiled and remained ignored.
- `situation/witnesses/evidence/W-000001/SHA256SUMS` retains the evidence
  digests, and `image-digest.txt` identifies the pinned rust:1.98.0 image.
- D-000004 records non-rename test deltas: the
  `CoreIdTokenVerifier<'_>` mechanical-lint change in
  `tests/rp_certification_code.rs` and the `#[allow(dead_code)]` plus
  explanatory comment in `tests/rp_common.rs`. It retains no exhaustive
  classification table in this witness; the gate log establishes only the run.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | NOT EXECUTED — no real-provider discovery matrix or upstream comparison ran. |
| P2 | NOT EXECUTED — no real-provider authorization-code/PKCE matrix or upstream comparison ran. |
| P3 | NOT EXECUTED — no real-provider ID-token matrix or upstream comparison ran. |
| P4 | NOT EXECUTED — no real-provider UserInfo matrix or upstream comparison ran. |
| P5 | NOT EXECUTED — no real-provider refresh-token matrix or upstream comparison ran. |
| P6 | NOT DECIDED — `gates-final.log` records the imported `cargo test --all-features` suite passing at the stated head, but O-000001 requires unchanged test bytes except for imports and D-000004 records non-rename deltas. |
