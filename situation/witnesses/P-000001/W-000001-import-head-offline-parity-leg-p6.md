# W-000001 — Import-head offline parity: P6 passed; complete observation invalid

## Promise

situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md

## Oracle

situation/oracles/O-000001-judge-full-oidc-rp-flow-parity-with-upstream.md

## Result

INVALID — the real-provider Pass legs P1–P5 were not exercised. P6 passed,
but a PASS witness must evidence every Pass leg.

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
- The import-fidelity classification that supports the unchanged-test premise
  is retained in
  `situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | NOT EXECUTED — no real-provider discovery matrix or upstream comparison ran. |
| P2 | NOT EXECUTED — no real-provider authorization-code/PKCE matrix or upstream comparison ran. |
| P3 | NOT EXECUTED — no real-provider ID-token matrix or upstream comparison ran. |
| P4 | NOT EXECUTED — no real-provider UserInfo matrix or upstream comparison ran. |
| P5 | NOT EXECUTED — no real-provider refresh-token matrix or upstream comparison ran. |
| P6 | PASS — `gates-final.log` records the imported `cargo test --all-features` suite passing at the stated head; the import-fidelity decision records the only allowed crate-name test-import changes. |
