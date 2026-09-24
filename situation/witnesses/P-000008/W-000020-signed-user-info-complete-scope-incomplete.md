# W-000020 — Signed-UserInfo complete-scope observation is incomplete

## Promise

situation/promises/P-000008-signed-user-info-verification-policy.md

## Oracle

situation/oracles/O-000018-judge-signed-user-info-verification-policy-complete-scope.md

## Result

INVALID — the fresh bounded command passed the retained direct ES256 and HS256
representative fixtures, but it did not independently observe the successor
oracle's request-builder, ES384, HS384, HS512, Client confidential-routing,
claim-preservation, or API-surface legs. This is an incomplete observation of
the complete-scope rule, not a partial PASS and not assurance.

## Head

2228a05f4f53b9304c0509b978c2aff7a8fedb12

## Observed

2026-09-24

## Evidence

- `situation/witnesses/evidence/W-000020/cargo-test-user-info-signed-response.log`
  retains the sanitized output of `cargo test --offline --lib --quiet --
  user_info_signed_response_` at this head (`exit=0`). Its SHA-256 is
  `a3dd7c62cd3dfd4fc1c1e9a2781cfd0c60820c31252b86daf64db8bae5585bc9`
  in the adjacent `SHA256SUMS` file.
- The command selects only
  `test_user_info_signed_response_es256` and
  `test_user_info_signed_response_hs256`; it cannot decide the newly explicit
  complete-scope legs listed as `INVALID` below.
- W-000011 remains the PASS observation of superseded O-000009, not of this
  successor rule.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | INVALID — the direct default-verifier assertion passed, but the request-builder default configuration was not selected. |
| P2 | PASS — the ES256 allowed-algorithm fixture passed in the retained command. |
| P3 | INVALID — no ES384 fixture or retained manual observation was applied to this successor rule. |
| P4 | INVALID — the direct HS256 confidential-verifier fixture passed, but the request-builder configuration was not selected. |
| P5 | INVALID — no HS384 fixture or retained manual observation was applied to this successor rule. |
| P6 | INVALID — no HS512 fixture or retained manual observation was applied to this successor rule. |
| P7 | INVALID — the HS256 representative does not decide every HS* mode. |
| P8 | INVALID — the command did not observe `Client::user_info` confidential routing. |
| P9 | INVALID — the command did not execute the claim-preservation fixture. |
| P10 | INVALID — no API-surface manual observation was retained for this rule. |
