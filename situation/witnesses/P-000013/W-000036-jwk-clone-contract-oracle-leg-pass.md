# W-000036 — JWK complete-scope successor observation is invalid

## Promise

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Oracle

situation/oracles/O-000026-judge-jwk-clone-contract.md

## Result

INVALID — the retained run at this head meaningfully decides the supplemental
Clone fixture legs, but O-000026 is now the self-contained complete-scope
successor of O-000023. This run did not decide the successor's direct and
container redaction, equality distinction, serde round trip, presence-marker,
non-secret-field, or derived-RSA-key legs. A PASS witness that omits an Oracle
leg is INVALID, not partial PASS. The table identifies every unobserved
successor leg without composing this observation with W-000032 or an
observation at another head. P-000013 remains `implemented` and unassured;
G-000034 and G-000032 remain open.

## Head

48761b267c479b38918e4a139ed5c8fd530f2236

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000036/cargo-test-core-jwk-clone-contract-fixture.log`
  retains `cargo test --offline --lib --quiet --
  test_core_jwk_clone_preserves_secret_fields_and_redaction` (`exit=0`): `test
  result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 111 filtered out;
  finished in 0.00s`. SHA-256
  `a55fec9d0ae3c2bf5901d94fe623f023bee9366bce1fb27ce44fc63624a2e5d3`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `48761b267c479b38918e4a139ed5c8fd530f2236`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This retained observation neither supplies a complete-scope PASS
  nor changes P-000013's frozen canonical `implemented` State.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | Not observed — the retained Clone fixture does not inspect the universal `d` formatter transformation. |
| P2 | Not observed — the retained Clone fixture does not inspect the universal `k` formatter transformation. |
| P3 | Not observed — the retained Clone fixture does not render a `JsonWebKeySet<CoreJsonWebKey>` container. |
| P4 | Not observed — the retained Clone fixture does not apply the successor's direct compact-and-pretty canary decision. |
| P5 | Not observed — the retained Clone fixture does not compare keys differing only in `d`/`k` or retain the successor's serde round-trip decision. |
| P6 | Not observed — the retained Clone fixture does not decide direct presence/absence markers and non-secret fields for the successor rule. |
| P7 | Not observed — the retained Clone fixture does not derive and render an RSA verification key. |
| P8 | Observed — `symmetric.clone()` and `ec.clone()` each compare equal to their original (`src/core/jwk/tests.rs` lines 342-347). |
| P9 | Observed — clone serde exports equal the originals and retain the base64url `k`/`d` canaries (lines 350-361). |
| P10 | Observed — compact and pretty clone output is byte-identical to the originals; the fixture checks redaction markers and canary absence on its checked renderings (lines 363-379). |
| P11 | Observed — the cloned symmetric key verifies HS256 and rejects a tampered message with `CryptoError` (lines 381-404). |
| F1 | Not observed — the retained Clone fixture does not apply the universal formatter failure decision. |
| F2 | Not observed — the retained Clone fixture does not apply the direct or delegated canary-leak failure decision. |
| F3 | Not observed — the retained Clone fixture does not apply the keys-differing-only-in-`d`/`k` equality-and-serde failure decision. |
| F4 | Not observed — the retained Clone fixture does not apply the presence-marker, non-secret-field, or derived-RSA failure decision. |
| F5 | Observed — P8's equality assertions fail a clone that loses or alters a secret-carrying field (lines 342-347). |
| F6 | Observed — P9's export equality and canary-presence assertions fail divergent or secret-dropping clone export (lines 350-361). |
| F7 | Observed — P10's byte-identity, marker, and canary-absence assertions fail a checked clone rendering that leaks, loses a marker, or diverges (lines 363-379). |
| F8 | Observed — P11's verification and tamper assertions fail a clone that cannot verify or accepts tampering (lines 381-404). |
