# W-000035 — at_hash complete-scope successor observation is invalid

## Promise

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Oracle

situation/oracles/O-000025-judge-documented-at-hash-rsa-pss-eddsa-families.md

## Result

INVALID — the retained run at this head meaningfully decides the supplemental
RS256/384/512, PS256/384/512, and EdDSA fixture legs, but O-000025 is now the
self-contained complete-scope successor of O-000019. This run did not decide
the successor's shared-secret, ES256/ES384, `signing_key`, or tutorial legs.
A PASS witness that omits an Oracle leg is INVALID, not partial PASS. The
table identifies every unobserved or incomplete successor leg without
composing this observation with W-000028 or an observation at another head.
P-000009 remains `implemented` and unassured; G-000033 and G-000032 remain
open.

## Head

48761b267c479b38918e4a139ed5c8fd530f2236

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000035/cargo-test-at-hash-rsa-pss-eddsa-fixture.log`
  retains `cargo test --offline --lib --quiet --
  test_id_token_verification_key_at_hash_rsa_pss_eddsa` (`exit=0`): `test
  result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 111 filtered out;
  finished in 0.14s`. SHA-256
  `ddbfe0051704ee4deb9a266af9a85552522e72e2248af504e51eb5711589c65e`.
- `situation/witnesses/evidence/W-000035/cargo-test-at-hash-filter-battery.log`
  retains the shared `cargo test --offline --lib --quiet --
  id_token_verification_key_at_hash` battery (`exit=0`): `test result: ok. 4
  passed; 0 failed; 0 ignored; 0 measured; 108 filtered out; finished in
  0.14s`. It corroborates fixture selection but was not an application of
  O-000025's corrected complete-scope rule.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `48761b267c479b38918e4a139ed5c8fd530f2236`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This retained observation neither supplies a complete-scope PASS
  nor changes P-000009's frozen canonical `implemented` State.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | Not observed — the retained supplemental fixture does not exercise the HS256 confidential-verifier path. |
| P2 | Not observed — the retained supplemental fixture does not exercise the HS384 confidential-verifier path. |
| P3 | Not observed — the retained supplemental fixture does not exercise the HS512 confidential-verifier path. |
| P4 | Not observed — the retained supplemental fixture does not exercise the ES256 provider-JWK path. |
| P5 | Not observed — the retained supplemental fixture does not exercise the ES384 provider-JWK path. |
| P6 | Observed — the `RsaSsaPkcs1V15Sha256` iteration signs, verifies through the public verifier's matching JWK, resolves it with `IdToken::verification_key`, and reproduces the embedded `at_hash` (`src/verification/tests.rs` lines 2421-2463, 2504-2523). |
| P7 | Observed — same helper and `RsaSsaPkcs1V15Sha384` iteration (lines 2421-2463, 2504-2523). |
| P8 | Observed — same helper and `RsaSsaPkcs1V15Sha512` iteration (lines 2421-2463, 2504-2523). |
| P9 | Observed — same helper and `RsaSsaPssSha256` iteration (lines 2421-2463, 2504-2523). |
| P10 | Observed — same helper and `RsaSsaPssSha384` iteration (lines 2421-2463, 2504-2523). |
| P11 | Observed — same helper and `RsaSsaPssSha512` iteration (lines 2421-2463, 2504-2523). |
| P12 | Observed — the Ed25519 signing key and `EdDsa` iteration use the same helper (lines 2525-2541). |
| P13 | Not observed — the retained supplemental fixture has no missing-client-secret decision for HS256, HS384, or HS512. |
| P14 | Incomplete — substituted-token inequality is observed for P6 through P12 (lines 2465-2472), but not for the HS256/384/512 or ES256/ES384 legs required by the successor. |
| P15 | Not observed — no retained decision inspects `IdToken::signing_key`'s borrowed JWKS-only scope. |
| P16 | Not observed — no retained tutorial compilation applies the successor's documentation leg. |
| F1 | Not observed — the retained supplemental fixture has no missing-client-secret negative for HS256, HS384, or HS512. |
| F2 | Incomplete — matching-JWK resolution and hash reproduction are observed for P6 through P12, but no ES256/ES384 successor-family decision is observed here. |
| F3 | Incomplete — the substituted-token negative is observed only for P6 through P12; the successor requires every named family. |
| F4 | Not observed — no retained tutorial compilation applies the successor's documentation failure leg. |
| F5 | Not observed — no retained decision inspects `IdToken::signing_key`'s borrowed JWKS-only scope. |
