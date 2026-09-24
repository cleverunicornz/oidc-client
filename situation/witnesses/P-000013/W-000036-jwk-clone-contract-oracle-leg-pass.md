# W-000036 — JWK Clone contract Oracle-leg observation passes

## Promise

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Oracle

situation/oracles/O-000026-judge-jwk-clone-contract.md

## Result

PASS — every O-000026 leg is decided at this head. The fixture clones a
symmetric key carrying the debug canary secret and an EC key carrying private
member `d`, then asserts equality preservation, unchanged serde export of the
secret material, byte-identical `Debug` rendering in compact and pretty form,
canary-absence on the symmetric compact and EC pretty renderings, and
retained HS256 verification capability on the cloned symmetric key. The
remaining format/key renderings (symmetric pretty, EC compact) are
byte-identical mirrors of the originals' renderings and inherit the
universal redaction rule O-000023 decides structurally in
`CoreJsonWebKey::fmt`, retained by W-000032. Together with W-000032 (the
observation of O-000023's listed legs), O-000023 plus O-000026 now have
passing observations covering P-000013's Scope at this head; promise state
transitions remain with the closure corrector.

## Head

48761b267c479b38918e4a139ed5c8fd530f2236

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000036/cargo-test-core-jwk-clone-contract-fixture.log`
  retains `cargo test --offline --lib --quiet -- test_core_jwk_clone_preserves_secret_fields_and_redaction`
  (`exit=0`): `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured;
  111 filtered out; finished in 0.00s`. SHA-256
  `a55fec9d0ae3c2bf5901d94fe623f023bee9366bce1fb27ce44fc63624a2e5d3`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `48761b267c479b38918e4a139ed5c8fd530f2236`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness observes one promise under one oracle at one head;
  promise state transitions remain with the closure corrector.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `symmetric.clone()` and `ec.clone()` (`src/core/jwk/tests.rs` lines 342-343) both `assert_eq!` their originals (lines 346-347), so the secret-carrying `k`/`d` fields are carried over exactly. |
| P2 | PASS — `serde_json::to_string` of each clone equals the original's export string (`src/core/jwk/tests.rs` lines 350-359), and both export strings `contains` the base64url `k` canary and the base64url `d` canary (lines 360-361), so the intentional export path survives Clone. |
| P3 | PASS (negative executed and structural) — executed: compact and pretty `Debug` of each clone is byte-identical to the original's (`src/core/jwk/tests.rs` lines 365-368); the symmetric compact rendering shows `k: Some([redacted])` (line 371) with the printable canary, base64url form, and decimal byte runs `222/173/190/239` asserted absent (lines 372-376), and the EC pretty rendering shows `[redacted]` (line 377) with the `170, 170, 170, 170` decimal run and base64url `d` form asserted absent (lines 378-379); structural: the symmetric pretty and EC compact renderings are byte-identical mirrors of the originals' renderings (lines 365-368), and `CoreJsonWebKey::fmt` — the only formatter receiving `d`/`k` values (`src/core/jwk/mod.rs` lines 77-99) — replaces both with the `[redacted]` marker (lines 83-84; marker text lines 71-75) before any field reaches the formatter, the universal rule O-000023 decides and W-000032 retains, so no clone rendering can contain the secret bytes. |
| P4 | PASS — the cloned symmetric key verifies an HS256 signature over its secret (`verify_signature(...).expect(...)`, `src/core/jwk/tests.rs` lines 383-393) and rejects the tampered message with `SignatureVerificationError::CryptoError` (lines 394-404). |
| F1 | PASS (negative executed) — P1's `assert_eq!`s (`src/core/jwk/tests.rs` lines 346-347) fail any clone that loses or alters a secret-carrying field; the fixture passes. |
| F2 | PASS (negative executed) — P2's export-equality and canary-presence assertions (`src/core/jwk/tests.rs` lines 350-361) fail any divergent or secret-dropping clone export; the fixture passes. |
| F3 | PASS (negative executed and structural) — executed: P3's byte-identity and canary-absence assertions (`src/core/jwk/tests.rs` lines 365-379) fail any clone whose checked renderings leak `d`/`k` bytes, lose the `[redacted]` markers, or differ from the original; structural: the unchecked format/key renderings are byte-identical mirrors of originals whose redaction is the universal formatter rule (`src/core/jwk/mod.rs` lines 77-99; O-000023, retained by W-000032), so a leak there also fails the byte-identity assertions. |
| F4 | PASS (negative executed) — P4's verification and tamper assertions (`src/core/jwk/tests.rs` lines 383-404) fail a cloned key that cannot verify or accepts a tampered message; the fixture passes. |
