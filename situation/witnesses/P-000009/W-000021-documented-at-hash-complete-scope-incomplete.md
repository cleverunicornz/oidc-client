# W-000021 — Documented at_hash complete-scope observation is incomplete

## Promise

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Oracle

situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md

## Result

INVALID — the fresh bounded command passed the retained HS256 and ES256
fixtures, including the missing-secret and substituted-token failures, but it
did not independently observe HS384, HS512, ES384, the `signing_key` API
scope, or the documentation leg. This is an incomplete observation of the
complete-scope rule, not a partial PASS and not assurance.

## Head

2228a05f4f53b9304c0509b978c2aff7a8fedb12

## Observed

2026-09-24

## Evidence

- `situation/witnesses/evidence/W-000021/cargo-test-id-token-verification-key-at-hash.log`
  retains the sanitized output of `cargo test --offline --lib --quiet --
  id_token_verification_key_at_hash` at this head (`exit=0`). Its SHA-256 is
  `e56020bf9cdb2a3b6633ccab52f440d6931bbfb55ea96f4d38cf968f749df62c`
  in the adjacent `SHA256SUMS` file.
- The command selects only
  `test_id_token_verification_key_at_hash`; it cannot decide the newly
  explicit complete-scope legs listed as `INVALID` below.
- W-000012 remains the PASS observation of superseded O-000010, not of this
  successor rule.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the HS256 confidential flow with an empty JWKS passed in the retained command. |
| P2 | INVALID — no HS384 fixture or retained manual observation was applied to this successor rule. |
| P3 | INVALID — no HS512 fixture or retained manual observation was applied to this successor rule. |
| P4 | PASS — the ES256 provider-JWK flow passed in the retained command. |
| P5 | INVALID — no ES384 fixture or retained manual observation was applied to this successor rule. |
| P6 | PASS — the public-verifier missing-secret failure passed in the retained command. |
| P7 | PASS — the substituted-token inequality assertion passed in the retained command. |
| P8 | INVALID — no `signing_key` scope observation was retained for this successor rule. |
| P9 | INVALID — the command did not execute the documentation compile leg. |
