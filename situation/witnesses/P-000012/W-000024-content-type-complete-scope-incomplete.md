# W-000024 — Content-Type complete-scope observation is incomplete

## Promise

situation/promises/P-000012-content-type-essence-matches-rfc7231-optional-whitespace.md

## Oracle

situation/oracles/O-000022-judge-content-type-essence-complete-scope.md

## Result

INVALID — the fresh bounded command passed the retained finite helper and
consumer fixtures, but it did not retain the successor oracle's structural
manual observation deciding every legal SP/HTAB OWS arrangement. This is an
incomplete observation of the complete-scope rule, not a partial PASS and not
assurance.

## Head

2228a05f4f53b9304c0509b978c2aff7a8fedb12

## Observed

2026-09-24

## Evidence

- `situation/witnesses/evidence/W-000024/cargo-test-content-type-paths.log`
  retains the sanitized output of `cargo test --offline --lib --quiet --
  content_type` at this head (`exit=0`). Its SHA-256 is
  `2e9e2df0e396ac1b360beab466d3730f73043ae2637d8e8352d0dde194df0e0e`
  in the adjacent `SHA256SUMS` file.
- The finite fixtures do not themselves decide every legal unbounded OWS
  arrangement, and no manual structural observation was retained for this
  successor rule.
- W-000015 remains the PASS observation of superseded O-000013, not of this
  successor rule.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | INVALID — no structural manual observation of universal SP/HTAB trimming was retained. |
| P2 | PASS — the case-variance fixture passed in the retained command. |
| P3 | PASS — malformed and wrong-essence rejection fixtures passed in the retained command. |
| P4 | PASS — the non-UTF-8 rejection fixture passed in the retained command. |
| P5 | PASS — the `check_content_type` missing/OWS/wrong fixture passed in the retained command. |
| P6 | PASS — the registration OWS response fixture passed in the retained command. |
| P7 | PASS — the UserInfo OWS routing fixture passed in the retained command. |
