# O-000026 — Judge JWK Clone contract

## State

implemented

## Judges

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Inputs

At the judged head, the fixture
`test_core_jwk_clone_preserves_secret_fields_and_redaction`
(`src/core/jwk/tests.rs`): a symmetric key carrying the debug canary secret
(`k`), an EC key carrying private member `d`, clones of both, their serde
export strings, compact and pretty `Debug` renderings, and an HS256
sign/verify round through the cloned symmetric key. This successor judges
only the Promise's unchanged-`Clone` clause, which O-000023's legs omit; with
O-000023 it collectively decides P-000013's complete declared Scope.
Redaction in the format/key renderings the fixture does not canary-check
(symmetric pretty, EC compact) is decided by O-000023's structural
universal-redaction rule — `CoreJsonWebKey::fmt`
(`src/core/jwk/mod.rs` lines 77-99) replaces `d`/`k` with the redaction
marker before any field reaches the formatter — carried to the clones by the
fixture's byte-identity assertions.

## Pass

- P1 — Cloning preserves equality: the cloned symmetric key compares equal to
  its original and the cloned EC key compares equal to its original, so the
  secret-carrying fields `k` and `d` are carried over exactly.
- P2 — Serde export is unchanged through Clone: serializing a clone produces
  the same JSON as serializing the original, and that export still carries
  the base64url `k` and `d` material (the intentional export path).
- P3 — The redacted `Debug` contract is preserved through Clone: compact and
  pretty `Debug` of each clone is byte-identical to the original's rendering,
  shows the presence of `k`/`d` (`[redacted]` markers), and never their bytes
  in raw, printable, decimal, or base64url form.
- P4 — A cloned symmetric key retains verification capability: it verifies an
  HS256 signature over its secret and rejects a tampered message with
  `SignatureVerificationError::CryptoError`.

## Fail

- F1 — A clone loses or alters a secret-carrying field such that it compares
  unequal to its original.
- F2 — A clone's serde export drops or alters the secret material or differs
  from the original's export.
- F3 — A clone's `Debug` output leaks `d`/`k` bytes, loses the `[redacted]`
  presence markers, or renders differently from the original.
- F4 — A cloned key fails to verify a genuine signature over its secret, or
  accepts a tampered message.

## Implementation

`cargo test --offline --lib --quiet -- test_core_jwk_clone_preserves_secret_fields_and_redaction` executes the fixture.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Clones compare equal, carrying `k`/`d` exactly. | `src/core/jwk/tests.rs::test_core_jwk_clone_preserves_secret_fields_and_redaction` (`assert_eq!` on both clones) |
| P2 | Clone serde export is identical and still carries the secrets. | same fixture (`to_string` equality plus base64url canary `contains` assertions) |
| P3 | Clone `Debug` stays redacted and identical in both formats. | same fixture (compact/pretty byte-identity for both clones; `[redacted]` markers and printable/decimal/base64url canary absence on the symmetric compact and EC pretty renderings); structural for symmetric pretty and EC compact via byte-identity with the originals plus O-000023's universal-redaction decision (`CoreJsonWebKey::fmt`, `src/core/jwk/mod.rs` lines 77-99, retained by W-000032) |
| P4 | Cloned key still verifies HS256 and rejects tampering. | same fixture (`verify_signature` success and `CryptoError` match on the tampered message) |
| F1 | Unequal clones fail the leg. | same fixture (negation of P1's `assert_eq!`s) |
| F2 | Divergent or secret-dropping export fails the leg. | same fixture (negation of P2's assertions) |
| F3 | Leaking or divergent `Debug` fails the leg. | same fixture (negation of P3's fixture assertions); structural for the unchecked format/key renderings via the same O-000023 decision |
| F4 | Lost verification capability fails the leg. | same fixture (negation of P4's verification and tamper assertions) |

## References

- Completes O-000023 for P-000013's complete declared scope: O-000023 remains
  in force for the legs it lists, and O-000023 plus O-000026 collectively
  decide the Promise's Scope — the redaction, equality/serde-distinction, and
  rendering-shape legs by O-000023, the unchanged-`Clone` clause by this
  oracle. Neither supersedes the other.
- situation/gaps/G-000034-jwk-redaction-oracle-omits-clone-contract.md — this
  oracle and its witness supply the evidence that settles the gap's omitted
  Clone clause; State and Resolution transitions remain with the closure
  corrector.
- situation/witnesses/P-000013/W-000032-jwk-debug-redaction-oracle-leg-pass.md
  remains the observation of O-000023's listed legs.
