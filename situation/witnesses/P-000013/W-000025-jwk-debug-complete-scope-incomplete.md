# W-000025 — JWK Debug complete-scope observation is incomplete

## Promise

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Oracle

situation/oracles/O-000023-judge-jwk-debug-redaction-complete-scope.md

## Result

INVALID — the fresh bounded command passed the retained canary fixtures, but it
did not retain the successor oracle's structural manual observation deciding
universal `d`/`k` redaction, and it did not select the RSA-derived-key fixture.
This is an incomplete observation of the complete-scope rule, not a partial
PASS and not assurance.

## Head

2228a05f4f53b9304c0509b978c2aff7a8fedb12

## Observed

2026-09-24

## Evidence

- `situation/witnesses/evidence/W-000025/cargo-test-jwk-debug-redaction.log`
  retains the sanitized output of `cargo test --offline --lib --quiet --
  debug_redact` at this head (`exit=0`). Its SHA-256 is
  `b8d5b27460cf25a312e713b12410382f58419a35cfd444fe92b1287f5c09eba7`
  in the adjacent `SHA256SUMS` file.
- The command exercises finite canaries only; no manual structural inspection
  deciding the universal formatter rule was retained for this successor rule.
- W-000016 remains the PASS observation of superseded O-000014, not of this
  successor rule.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | INVALID — no structural manual observation of the universal `d` redaction branch was retained. |
| P2 | INVALID — no structural manual observation of the universal `k` redaction branch was retained. |
| P3 | PASS — the delegated key-set canary fixture passed in the retained command. |
| P4 | PASS — the direct symmetric and EC canary fixtures passed in the retained command. |
| P5 | PASS — the equality and serde round-trip fixture passed in the retained command. |
| P6 | PASS — the direct canary fixtures kept the expected presence markers. |
| P7 | INVALID — the derived-RSA-key fixture was not selected. |
