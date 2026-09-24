# O-000023 — Judge JWK Debug redaction complete scope

## State

implemented

## Judges

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Inputs

At the judged head, the manual `Debug` implementation of `CoreJsonWebKey`,
its direct and container output, and fixtures carrying EC/RSA private `d` and
symmetric `k` material. This successor retains O-000014's canary fixtures and
adds structural manual decisions for the Promise's universal rendering rule.

## Pass

- P1 — The `Debug` implementation transforms every present `d` member into a
  redaction marker before passing the field to the formatter, so no `d` bytes
  can enter direct or delegated `Debug` output.
- P2 — The `Debug` implementation transforms every present `k` member into a
  redaction marker before passing the field to the formatter, so no `k` bytes
  can enter direct or delegated `Debug` output.
- P3 — A container such as `JsonWebKeySet<CoreJsonWebKey>` delegates to this
  redacted key formatter rather than exposing its private material itself.
- P4 — Fixed canaries confirm neither compact nor pretty `Debug` exposes raw,
  printable, decimal, or base64url forms of representative `d` and `k` data.
- P5 — `PartialEq` and serde export/round trips retain their intentional
  distinction and export behavior for keys differing only in `d` or `k`.
- P6 — Direct output keeps presence/absence of `d` and `k` visible while
  leaving non-secret fields available to consumers.
- P7 — An RSA verification key derived from a private key renders neither
  private member as present.

## Fail

- F1 — The formatter sends an actual `d` or `k` value, rather than a redaction
  marker, to a `Debug` field.
- F2 — A direct or delegated `Debug` rendering contains a representative
  private-material canary in raw, printable, decimal, or base64url form.
- F3 — Keys differing only in `d` or `k` compare equal, or a serde round trip
  loses intentional private-material export behavior.
- F4 — Presence/absence markers or non-secret fields disappear, or an RSA
  verification key renders private material as present.

## Implementation

`cargo test --offline --lib --quiet -- debug_redact` executes retained direct
and delegated canary fixtures. The universal rule is structurally manual:
`CoreJsonWebKey::fmt` must be inspected as the only formatter that receives
those field values.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Every present `d` is replaced before formatting. | manual — `src/core/jwk/mod.rs::CoreJsonWebKey::fmt` maps `self.d.as_ref()` only to `RedactedKeyMaterial` |
| P2 | Every present `k` is replaced before formatting. | manual — `src/core/jwk/mod.rs::CoreJsonWebKey::fmt` maps `self.k.as_ref()` only to `RedactedKeyMaterial` |
| P3 | Container output delegates to redacted keys. | `src/core/jwk/tests.rs::test_core_jwks_debug_redacts_symmetric_secret` |
| P4 | Representative forms do not leak. | `test_core_jwk_debug_redacts_symmetric_secret`; `test_core_jwk_debug_redacts_ec_private_member` |
| P5 | Equality and serde behavior remain intentional. | `src/core/jwk/tests.rs::test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes`; EC round-trip assertions |
| P6 | Presence markers stay visible. | `test_core_jwk_debug_redacts_symmetric_secret`; `test_core_jwk_debug_redacts_ec_private_member` |
| P7 | Derived RSA verification key has no private material. | `src/core/jwk/tests.rs::test_core_jwk_rsa_verification_key_debug_has_no_private_material` |
| F1 | No actual private field reaches the formatter. | manual — same structural `CoreJsonWebKey::fmt` decision as P1/P2 |
| F2 | Representative material never leaks. | P3/P4 fixtures |
| F3 | Equality and serde semantics remain distinct. | P5 fixture |
| F4 | Rendering shape remains correct. | P6/P7 fixtures |

## References

- Supersedes O-000014 for P-000013's complete declared scope; W-000016
  remains an observation of the historical, narrower rule.
- situation/witnesses/P-000013/W-000025-jwk-debug-complete-scope-incomplete.md
