# W-000032 — JWK Debug redaction Oracle-leg observation passes

## Promise

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Oracle

situation/oracles/O-000023-judge-jwk-debug-redaction-complete-scope.md

## Result

PASS — every O-000023 leg is decided at this head. This is PASS evidence for
O-000023's listed legs, not complete-Promise coverage: no O-000023 Pass or
Fail leg decides P-000013's unchanged `Clone` clause. The retained
`debug_redact` battery executes four canary fixtures, the universal `d`/`k`
redaction rule (P1/P2/F1), which the oracle designates structurally manual, is
decided by code inspection of the only formatter that receives those field
values, and the P7 derived-RSA fixture — which the oracle's name filter does
not select — is executed by an explicitly named supplementary command recorded
below.

Finding recorded honestly: the oracle's Implementation command
(`-- debug_redact`) selects four of the five coverage-cited fixtures; the P7
fixture is named
`test_core_jwk_rsa_verification_key_debug_has_no_private_material` and does
not contain the `debug_redact` filter substring. It was run by its exact name
and passed (`1 passed`); no substitution was made silently.

## Head

d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000032/cargo-test-debug-redact.log` retains
  `cargo test --offline --lib --quiet -- debug_redact` (`exit=0`): `test
  result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 106 filtered out`.
  The selected tests are
  `test_core_jwk_debug_redacts_symmetric_secret`,
  `test_core_jwk_debug_redacts_ec_private_member`,
  `test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes`,
  and `test_core_jwks_debug_redacts_symmetric_secret`. SHA-256
  `ce2c3265169429f6b7936efd3c38f8883eac535b0c6fa5ea2dc8d1c5bb5d761d`.
- `situation/witnesses/evidence/W-000032/cargo-test-jwk-rsa-verification-key-fixture.log`
  retains `cargo test --offline --lib --quiet --
  test_core_jwk_rsa_verification_key_debug_has_no_private_material`
  (`exit=0`): `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 109
  filtered out`. SHA-256
  `dcf863aba63813100f801687abcd0ba26f3523a7f145e815a52bfa60ffda1399`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness observes one promise under one oracle at one head;
  promise state transitions remain with the closure corrector.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — structural: the manual `Debug` impl of `CoreJsonWebKey` (`src/core/jwk/mod.rs` lines 77-99) computes `let d = self.d.as_ref().map(|_| RedactedKeyMaterial)` (line 83) before formatting; the `map` discards the payload for every present `d` and passes the `Option<RedactedKeyMaterial>` local (line 95) — never `self.d` — to `debug_struct`. `RedactedKeyMaterial::fmt` writes only `[redacted]` (lines 71-75). Direct and delegated (`debug_struct`-produced) output therefore cannot contain `d` bytes. |
| P2 | PASS — structural, same formatter: `let k = self.k.as_ref().map(|_| RedactedKeyMaterial)` (line 84), passed at line 96; no `k` bytes can enter the output. |
| P3 | PASS — `test_core_jwks_debug_redacts_symmetric_secret` renders a `JsonWebKeySet<CoreJsonWebKey>` container and asserts the delegated output carries the `[redacted]` marker and none of the canary bytes (`src/core/jwk/tests.rs` lines 256-273). |
| P4 | PASS — `test_core_jwk_debug_redacts_symmetric_secret` and `test_core_jwk_debug_redacts_ec_private_member` assert neither compact nor pretty `Debug` output contains the representative `d`/`k` canaries in raw, printable, decimal, or base64url form (`src/core/jwk/tests.rs` lines 199-255; the EC fixture's `x`/`y` fills are chosen so their Debug output can never contain the `0xaa` decimal run a leaked `d` would print, lines 220-222). |
| P5 | PASS — `test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes` asserts keys differing only in `d`/`k` render identically yet compare unequal, and retains serde export/round-trip assertions (`src/core/jwk/tests.rs` lines 274-308; export fidelity anchored by `deserialize_option_or_none` on the fields, `src/core/jwk/mod.rs` lines 59-65). |
| P6 | PASS — the direct fixtures assert presence markers stay visible for present material and `None` for absent, while non-secret fields (`kty`, `use_`, `kid`, `alg`, `n`, `e`, `crv`, `x`, `y`) pass through unredacted (`src/core/jwk/tests.rs` lines 199-255; formatter field list `src/core/jwk/mod.rs` lines 85-97). |
| P7 | PASS — `test_core_jwk_rsa_verification_key_debug_has_no_private_material` (executed by the named supplementary command above; fixture at `src/core/jwk/tests.rs` lines 309-330) derives the verification key via `CoreRsaPrivateSigningKey::from_pem(...).as_verification_key()` and asserts its Debug rendering shows neither private member as present. |
| F1 | PASS (negative structural) — same structural decision as P1/P2: the only formatter receiving the `d`/`k` field values is `CoreJsonWebKey::fmt`, and both values are replaced by `RedactedKeyMaterial` before any `.field(...)` call; no code path forwards `self.d` or `self.k` to `Debug`. |
| F2 | PASS (negative executed) — the canary fixtures (P3/P4) assert no representative private material appears in direct or delegated output in any encoded form. |
| F3 | PASS (negative executed) — the P5 fixture asserts `PartialEq` still distinguishes and serde round trips retain export behavior. |
| F4 | PASS (negative executed and structural) — the P6 fixtures assert the executed portion of the rendering shape (canary absence and `d`/`k` presence/absence markers, `src/core/jwk/tests.rs` lines 199-255) and the P7 fixture asserts neither private member renders as present on the derived RSA verification key (lines 309-330). Retention of the non-secret fields (`kty`, `use_`, `kid`, `alg`, `n`, `e`, `crv`, `x`, `y`) is structural: the formatter passes them through unredacted (`src/core/jwk/mod.rs` lines 85-97); the retained fixtures do not assert those fields individually. |
