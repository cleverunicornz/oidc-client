# W-000002 — Import-head ES256 unit observation

## Promise

situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md

## Oracle

situation/oracles/O-000002-judge-es256-verification.md

## Result

INVALID — this is a retrospective application of O-000002 to the retained
2026-09-22 gate run. The run independently evidences P1–P3 and the
invalid-P-256-signature rejection decided by F3, but it did not exercise the
manual full-ID-token P4 and F4 legs; it cannot support assurance or claim that
O-000002 was predeclared at run time.

## Head

24835e4b44caa8a0baae2ac5b865bbd6bdf355ba

## Observed

2026-09-22

## Evidence

`situation/witnesses/evidence/W-000001/gates-final.log` retains the real,
pinned-container `cargo test --all-features` run at this head. It records the
following passing tests: `test_ecdsa_verification`,
`test_core_jwk_deserialization_ec`, and
`test_discovery_deserialization`. The file and its pinned-image evidence are
digested in `situation/witnesses/evidence/W-000001/SHA256SUMS`.

The test's mismatched-curve `if let Some(err) = ... .err()` branches do not
fail on unexpected success. O-000002 therefore treats no mismatched-curve
behavior as an in-scope executable decision.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_ecdsa_verification` passed in `gates-final.log`; it accepts the valid P-256 fixture under direct ES256 enum dispatch. |
| P2 | PASS — `test_core_jwk_deserialization_ec` and `test_ecdsa_verification` passed in `gates-final.log`. |
| P3 | PASS — `test_discovery_deserialization` passed in `gates-final.log` with a fixture that contains ES256. |
| P4 | NOT EXECUTED — no retained full ES256 ID-token fixture exercises issuer, audience, nonce, and expiry through the public verifier. |
