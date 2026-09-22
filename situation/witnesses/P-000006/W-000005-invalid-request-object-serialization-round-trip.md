# W-000005 — InvalidRequestObject serialization round-trip passes

## Promise

situation/promises/P-000006-invalid-request-object-serialization-round-trip.md

## Oracle

situation/oracles/O-000007-judge-invalid-request-object-serialization-round-trip.md

## Result

PASS — the scoped declared test passed at the head below, deciding every
O-000007 Pass leg for the public serialization and parsing surface.

## Head

6bc21cdbcf933bcaf52a9f7f7ef33e3dda851bb3

## Observed

2026-09-22

## Evidence

- `situation/witnesses/evidence/W-000005/scoped-serialization-round-trip.log`
  retains the scoped
  `cargo test --all-features test_auth_error_type_round_trip` result at the
  stated head: one matching unit test passed with no failures.
- `situation/witnesses/evidence/W-000005/SHA256SUMS` records SHA-256
  `5cd77a196adbfdb9b63f8e46921c49e3bf811c3841fdc05680f790936f168869`
  for that retained result file.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_auth_error_type_round_trip` compared the serde JSON output with `"invalid_request_object"` and passed. |
| P2 | PASS — the same test parsed its serialized JSON result and compared it with `InvalidRequestObject`; it passed. |
| P3 | PASS — the same test parsed the canonical string through serde-plain and compared it with `InvalidRequestObject`; it passed. |
