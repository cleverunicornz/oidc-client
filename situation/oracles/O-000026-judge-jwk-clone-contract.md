# O-000026 — Judge JWK Debug complete-scope successor

## State

implemented

## Judges

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Inputs

At the judged head, the manual `Debug` implementation of `CoreJsonWebKey`, its
direct and container output, keys carrying EC/RSA private `d` and symmetric
`k` material, their equality and serde export/round-trip behavior, derived RSA
verification keys, and clones of symmetric and EC keys. The executable inputs
are the retained `debug_redact` fixtures, the exact
`test_core_jwk_rsa_verification_key_debug_has_no_private_material` fixture,
and `test_core_jwk_clone_preserves_secret_fields_and_redaction`.

This successor judges every explicit clause of P-000013's declared Scope. It
supersedes O-000023 rather than composing two partial rules.

## Pass

- P1 — The `Debug` implementation transforms every present `d` member into a
  redaction marker before passing the field to the formatter, so no `d` bytes
  can enter direct or delegated `Debug` output.
- P2 — The `Debug` implementation transforms every present `k` member into a
  redaction marker before passing the field to the formatter, so no `k` bytes
  can enter direct or delegated `Debug` output.
- P3 — A container such as `JsonWebKeySet<CoreJsonWebKey>` delegates to this
  redacted key formatter rather than exposing private material itself.
- P4 — Fixed canaries confirm neither compact nor pretty `Debug` exposes raw,
  printable, decimal, or base64url forms of representative `d` and `k` data.
- P5 — `PartialEq` and serde export/round trips retain their intentional
  distinction and export behavior for keys differing only in `d` or `k`.
- P6 — Direct output keeps presence or absence of `d` and `k` visible while
  leaving non-secret fields available to consumers.
- P7 — An RSA verification key derived from a private key renders neither
  private member as present.
- P8 — Cloning a symmetric or EC key preserves equality with its original, so
  the `k` and `d` fields are carried over exactly.
- P9 — Serializing either clone produces its original's export, including the
  intentional `k` and `d` export material.
- P10 — Compact and pretty `Debug` of either clone is byte-identical to the
  original rendering and retains the redaction markers without secret bytes.
- P11 — A cloned symmetric key verifies an HS256 signature over its secret and
  rejects a tampered message with `SignatureVerificationError::CryptoError`.

## Fail

- F1 — The formatter sends an actual `d` or `k` value, rather than a redaction
  marker, to a `Debug` field.
- F2 — A direct or delegated `Debug` rendering contains a representative
  private-material canary in raw, printable, decimal, or base64url form.
- F3 — Keys differing only in `d` or `k` compare equal, or a serde round trip
  loses intentional private-material export behavior.
- F4 — Presence/absence markers or non-secret fields disappear, or an RSA
  verification key renders private material as present.
- F5 — A clone loses or alters a secret-carrying field such that it compares
  unequal to its original.
- F6 — A clone's serde export drops or alters secret material or differs from
  its original's export.
- F7 — A clone's `Debug` output leaks `d`/`k` bytes, loses redaction markers,
  or renders differently from its original.
- F8 — A cloned symmetric key fails to verify a genuine signature over its
  secret or accepts a tampered message.

## Implementation

`cargo test --offline --lib --quiet -- debug_redact` executes the retained
direct and delegated canary fixtures, and the exact-name RSA fixture supplies
the derived-key decision. `cargo test --offline --lib --quiet --
test_core_jwk_clone_preserves_secret_fields_and_redaction` executes the Clone
fixture. The universal `d`/`k` formatter decisions remain manual because
`CoreJsonWebKey::fmt` is the only formatter receiving those values.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Every present `d` is replaced before formatting. | manual — `src/core/jwk/mod.rs::CoreJsonWebKey::fmt` maps `self.d.as_ref()` only to `RedactedKeyMaterial` |
| P2 | Every present `k` is replaced before formatting. | manual — `src/core/jwk/mod.rs::CoreJsonWebKey::fmt` maps `self.k.as_ref()` only to `RedactedKeyMaterial` |
| P3 | Container output delegates to redacted keys. | `src/core/jwk/tests.rs::test_core_jwks_debug_redacts_symmetric_secret` |
| P4 | Representative forms do not leak. | `test_core_jwk_debug_redacts_symmetric_secret`; `test_core_jwk_debug_redacts_ec_private_member` |
| P5 | Equality and serde behavior remain intentional. | `src/core/jwk/tests.rs::test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes`; EC round-trip assertions |
| P6 | Presence markers and non-secret fields remain visible. | `test_core_jwk_debug_redacts_symmetric_secret`; `test_core_jwk_debug_redacts_ec_private_member` |
| P7 | Derived RSA verification key has no private material. | `src/core/jwk/tests.rs::test_core_jwk_rsa_verification_key_debug_has_no_private_material` |
| P8 | Clones compare equal, carrying `k`/`d` exactly. | `src/core/jwk/tests.rs::test_core_jwk_clone_preserves_secret_fields_and_redaction` (`assert_eq!` on both clones) |
| P9 | Clone serde export is identical and carries the secrets. | same fixture (`to_string` equality plus base64url canary `contains` assertions) |
| P10 | Clone `Debug` stays redacted and identical in both formats. | same fixture (compact/pretty byte identity; redaction markers and printable/decimal/base64url canary absence) |
| P11 | Cloned key verifies HS256 and rejects tampering. | same fixture (`verify_signature` success and `CryptoError` match) |
| F1 | No actual private field reaches the formatter. | manual — same structural `CoreJsonWebKey::fmt` decision as P1/P2 |
| F2 | Representative material never leaks. | P3/P4 fixtures |
| F3 | Equality and serde semantics remain distinct. | P5 fixture |
| F4 | Rendering shape remains correct. | P6/P7 fixtures |
| F5 | Unequal clones fail the Clone equality leg. | Clone fixture (negation of P8's `assert_eq!`s) |
| F6 | Divergent or secret-dropping clone export fails the serde leg. | Clone fixture (negation of P9's assertions) |
| F7 | Leaking or divergent clone `Debug` fails the rendering leg. | Clone fixture (negation of P10's assertions) |
| F8 | Lost verification capability or accepted tampering fails the Clone behavior leg. | Clone fixture (negation of P11's assertions) |

## References

- Supersedes O-000023 for P-000013's complete declared Scope. O-000023 and
  W-000032 remain immutable historical partial-rule records and do not compose
  with this successor.
- situation/gaps/G-000034-jwk-redaction-oracle-omits-clone-contract.md remains
  open: W-000036 is INVALID for this complete successor because its retained
  run decides only the Clone legs.
- situation/gaps/G-000032-current-pass-evidence-outpaces-frozen-promise-states.md
  retains the frozen Promise-state and forward-link reconciliation concern.
- situation/witnesses/P-000013/W-000036-jwk-clone-contract-oracle-leg-pass.md
  retains the Clone fixture observation, classified INVALID against this
  successor.
