# W-000038 — JWK Debug complete-scope successor PASS

## Promise

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Oracle

situation/oracles/O-000026-judge-jwk-clone-contract.md

## Result

PASS — every Pass and Fail leg of O-000026 is decided at this head: the
executable legs by the retained battery runs below, the manual legs by the
inspections cited per row. This is the complete-scope successor's own
observation at the current head; the retained W-000036 and W-000032 records
remain their own immutable observations and are not composed into this one.
P-000013's canonical State is frozen by the third closure; Promise-state
transitions and Gap resolutions belong to the fourth closure's corrector and
are not performed here.

## Head

c5260aeb4a426f7881c8261ef2101d5a30710c29

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000038/cargo-test-debug-redact.log` retains
  `cargo test --offline --lib --quiet -- debug_redact` (`exit=0`): `test
  result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 108 filtered out;
  finished in 0.00s` — the retained direct and delegated canary fixtures.
  SHA-256
  `69a6eb42996ba1baa757ee77804feaecbd3c29d9b72cf062297eac2074951ad3`.
- `situation/witnesses/evidence/W-000038/cargo-test-jwk-rsa-verification-key-fixture.log`
  retains `cargo test --offline --lib --quiet --
  test_core_jwk_rsa_verification_key_debug_has_no_private_material`
  (`exit=0`): `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured;
  111 filtered out; finished in 0.00s`. SHA-256
  `8c3ebb79536b8947f7e921be3b4e7d0a22e8735eb26d3fca56d44fed3f9c0450`.
- `situation/witnesses/evidence/W-000038/cargo-test-core-jwk-clone-contract-fixture.log`
  retains `cargo test --offline --lib --quiet --
  test_core_jwk_clone_preserves_secret_fields_and_redaction` (`exit=0`):
  `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 111 filtered
  out; finished in 0.00s`. SHA-256
  `a55fec9d0ae3c2bf5901d94fe623f023bee9366bce1fb27ce44fc63624a2e5d3`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `c5260aeb4a426f7881c8261ef2101d5a30710c29`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness decides the successor oracle's legs at this head only;
  it performs no Promise-state transition and resolves no Gap.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | Observed — manual: `src/core/jwk/mod.rs::CoreJsonWebKey::fmt` (lines 82-98) maps `self.d.as_ref()` only to the presence-marker `RedactedKeyMaterial` (line 83) before any formatter call; the `RedactedKeyMaterial` `Debug` impl writes only `[redacted]` (lines 71-75), so no `d` bytes can enter the formatter. |
| P2 | Observed — manual: the same `fmt` maps `self.k.as_ref()` only to `RedactedKeyMaterial` (line 84); the symmetric field passed at line 96 is the marker, so no `k` bytes can enter the formatter. |
| P3 | Observed — battery: `src/core/jwk/tests.rs::test_core_jwks_debug_redacts_symmetric_secret` (lines 257-272) — the `JsonWebKeySet` container `Debug` output contains neither the printable canary nor the base64url export nor the decimal byte run, and does contain `k: Some([redacted])`. |
| P4 | Observed — battery: `test_core_jwk_debug_redacts_symmetric_secret` (lines 200-216) asserts compact and pretty output exclude the printable canary, the base64url export, and the decimal forms 222/173/190/239; `test_core_jwk_debug_redacts_ec_private_member` (lines 219-254) asserts compact and pretty output exclude the decimal `d` run `170, 170, 170, 170` and the base64url `d` export. |
| P5 | Observed — battery: `test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes` (lines 275-307) — identical `Debug` for keys differing only in `k` (lines 278) with `assert_ne!` equality (line 279), the same pair for keys differing only in `d` (lines 305-306), and serde export carrying the secret with an equal round trip (lines 282-286; the EC export/round-trip pair also at lines 248-253). |
| P6 | Observed — battery: presence/absence stays visible — `k: Some([redacted])` (line 214), `d: Some([redacted])` and `k: None` (lines 244-245) — and `CoreJsonWebKey::fmt` keeps the non-secret fields `kty`, `use_`, `kid`, `alg`, `n`, `e`, `crv`, `x`, `y` in its output (`src/core/jwk/mod.rs` lines 86-94). |
| P7 | Observed — battery: `test_core_jwk_rsa_verification_key_debug_has_no_private_material` (lines 310-319) — the derived RSA verification key renders `d: None` and `k: None` and never `d: Some`. |
| P8 | Observed — battery: `test_core_jwk_clone_preserves_secret_fields_and_redaction` — `symmetric.clone()` and `ec.clone()` each satisfy `assert_eq!` with their original (lines 342-347), so `k` and `d` are carried over exactly. |
| P9 | Observed — battery: clone serde exports equal the originals' exports (lines 350-359) and retain the intentional base64url `k` and `d` export material (lines 360-361). |
| P10 | Observed — battery: compact and pretty clone `Debug` are byte-identical to the originals (lines 365-368) and the fixture's checked renderings retain the markers without secret bytes — symmetric compact checked at lines 369-376, EC pretty checked at lines 370 and 377-379. Manual: the remaining two renderings (symmetric-pretty, EC-compact) are decided by P1/P2's universal `CoreJsonWebKey::fmt` inspection — `fmt` is the only formatter that receives `d`/`k` values, so any rendering of those keys uses the same redaction — plus the byte-identity assertions. |
| P11 | Observed — battery: the cloned symmetric key verifies a genuine HS256 signature over its secret (lines 383-393) and rejects a tampered message matching `SignatureVerificationError::CryptoError` (lines 394-404). |
| F1 | Observed — manual: the same structural decision as P1/P2 — no code path sends an actual `d` or `k` value to a `Debug` field, because `CoreJsonWebKey::fmt` replaces both with `RedactedKeyMaterial` before `debug_struct` (lines 83-84, 95-96). |
| F2 | Observed — battery: the P3/P4 fixtures fail any direct or delegated rendering containing a representative canary in raw, printable, decimal, or base64url form (lines 200-216, 219-254, 257-272). |
| F3 | Observed — battery: the P5 fixture fails keys differing only in `d`/`k` comparing equal (`assert_ne!` at lines 279 and 306) and fails a serde round trip that loses the intentional export behavior (lines 282-286, 248-253). |
| F4 | Observed — battery: the P6/P7 fixtures fail disappeared presence/absence markers (`k: Some([redacted])` at line 214, `d: Some([redacted])` and `k: None` at lines 244-245, and `d: None`/`k: None`/no `d: Some` at lines 316-318). Manual: non-secret-field preservation is decided by the same `CoreJsonWebKey::fmt` inspection as P6 — `fmt` passes `kty`, `use_`, `kid`, `alg`, `n`, `e`, `crv`, `x`, `y` to the formatter (`src/core/jwk/mod.rs` lines 86-94) — so a rendering that dropped them would contradict the inspected structure. |
| F5 | Observed — battery: P8's `assert_eq!` assertions fail a clone that loses or alters a secret-carrying field so it compares unequal (lines 342-347). |
| F6 | Observed — battery: P9's export-equality and canary-presence assertions fail a clone export that drops, alters, or diverges from its original (lines 350-361). |
| F7 | Observed — battery: P10's byte-identity, marker, and canary-absence assertions fail a checked clone rendering that leaks `d`/`k` bytes, loses a marker, or renders differently (lines 365-379). Manual: for the unchecked renderings, the P1/P2 universal formatter decision supplies the redaction property, so a leak there would contradict the decided `fmt` structure. |
| F8 | Observed — battery: P11's verification-success and tamper-rejection assertions fail a clone that cannot verify a genuine signature or accepts a tampered message (lines 383-404). |
