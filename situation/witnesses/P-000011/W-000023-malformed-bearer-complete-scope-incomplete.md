# W-000023 — Malformed bearer-token complete-scope observation is incomplete

## Promise

situation/promises/P-000011-malformed-bearer-token-fails-request-preparation.md

## Oracle

situation/oracles/O-000021-judge-malformed-bearer-token-complete-scope.md

## Result

INVALID — the fresh bounded command passed the retained malformed-token
fixtures, but it did not execute the valid synchronous-header fixtures,
tokenless-registration fixture, or either valid-token asynchronous dispatch
scenario. This is an incomplete observation of the complete-scope rule, not a
partial PASS and not assurance.

## Head

2228a05f4f53b9304c0509b978c2aff7a8fedb12

## Observed

2026-09-24

## Evidence

- `situation/witnesses/evidence/W-000023/cargo-test-malformed-bearer-paths.log`
  retains the sanitized output of `cargo test --offline --lib --quiet --
  malformed` at this head (`exit=0`). Its SHA-256 is
  `6c8b366a4cc06128f27b9000cfea159166b0cb667024724fa7c9fcd18d4100dd`
  in the adjacent `SHA256SUMS` file.
- The command selects the malformed-token fixtures but no valid asynchronous
  request fixture, so it cannot decide the complete-scope legs marked
  `INVALID` below.
- W-000014 remains the PASS observation of superseded O-000012, not of this
  successor rule.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the synchronous UserInfo malformed-token fixture passed in the retained command. |
| P2 | PASS — the synchronous registration malformed-token fixture passed in the retained command. |
| P3 | PASS — the asynchronous UserInfo malformed-token fixture passed in the retained command. |
| P4 | PASS — the asynchronous registration malformed-token fixture passed in the retained command. |
| P5 | PASS — the malformed-token no-echo fixture passed in the retained command. |
| P6 | INVALID — the valid synchronous UserInfo header fixture was not selected. |
| P7 | INVALID — the valid synchronous registration header fixture was not selected. |
| P8 | INVALID — the tokenless-registration fixture was not selected. |
| P9 | INVALID — no valid asynchronous UserInfo request observation was retained. |
| P10 | INVALID — no valid asynchronous registration request observation was retained. |
