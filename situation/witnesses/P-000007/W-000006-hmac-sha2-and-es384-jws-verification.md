# W-000006 — HMAC SHA-2 and ES384 verification pass

## Promise

situation/promises/P-000007-hmac-sha2-and-es384-jws-verification.md

## Oracle

situation/oracles/O-000008-judge-hmac-sha2-and-es384-jws-verification.md

## Result

PASS — the two scoped declared tests passed at the head below, deciding every
O-000008 Pass leg for the named HMAC SHA-2 and ES384 verification paths.

## Head

6bc21cdbcf933bcaf52a9f7f7ef33e3dda851bb3

## Observed

2026-09-22

## Evidence

- `situation/witnesses/evidence/W-000006/scoped-hmac-es384-verification.log`
  retains the two scoped declared commands at the stated head:
  `cargo test --all-features test_hmac_sha256_verification` and
  `cargo test --all-features test_ecdsa_verification`. Each matching unit test
  passed with no failures.
- `situation/witnesses/evidence/W-000006/SHA256SUMS` records SHA-256
  `65fd2bfe2c9d0e4eee8df142b5fb6f5f30b14241fda2247f8d0c28dd99d40164`
  for that retained result file.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_hmac_sha256_verification` verified the fixed HS256 signature with its symmetric JWK. |
| P2 | PASS — `test_hmac_sha256_verification` verified the fixed HS384 signature with its symmetric JWK. |
| P3 | PASS — `test_hmac_sha256_verification` verified the fixed HS512 signature with its symmetric JWK. |
| P4 | PASS — `test_ecdsa_verification` verified the fixed ES384 signature with its EC P-384 JWK. |
