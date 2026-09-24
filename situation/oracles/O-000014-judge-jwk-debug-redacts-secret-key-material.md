# O-000014 — Judge JWK Debug secret redaction

## State

implemented

## Judges

situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md

## Inputs

The `Debug` (`{:?}`) and pretty `Debug` (`{:#?}`) output of `CoreJsonWebKey`
values carrying: a symmetric secret built by `JsonWebKey::new_symmetric` from
a canary; a deserialized EC JWK carrying `d`; an RSA verification key derived
from `CoreRsaPrivateSigningKey::as_verification_key`; and a
`JsonWebKeySet<CoreJsonWebKey>` containing a symmetric key. Also the serde
round trip of keys carrying `d`/`k`, and `PartialEq` comparison of keys
differing only in `d`/`k`. Evidence is produced by the focused command
`cargo test --offline --lib core::jwk` on the judged head.

## Pass

- P1 — Symmetric: neither `{:?}` nor `{:#?}` of a `new_symmetric` key contains
  the secret in any form (raw decimal byte list, printable text, or base64url
  export form); compact output shows `k: Some([redacted])` and pretty output
  shows `[redacted]`.
- P2 — EC private member: for an EC JWK deserialized with `d`, neither format
  contains `d`'s bytes in decimal-run or base64url form; compact output shows
  `d: Some([redacted])` and `k: None`.
- P3 — Key set: `JsonWebKeySet<CoreJsonWebKey>` `Debug` output contains no
  representation of the contained symmetric secret.
- P4 — Stability, distinction, export: keys differing only in `d`/`k` bytes
  produce identical `Debug` output, compare unequal via `PartialEq`, and a
  serde round trip preserves `d`/`k` (re-serialized export form present and
  round-tripped key equal).
- P5 — Non-regression: the pre-existing signature-verification tests in
  `src/core/jwk/tests.rs` (ECDSA P-256/P-384, RSA PKCS1/PSS, HMAC, EdDSA)
  still pass unchanged.
- P6 — RSA derivation path: the verification key derived from an RSA private
  key shows `d: None` and `k: None` and never `d: Some`.

## Fail

Any of: a representation of `d`/`k` bytes (decimal list, printable text, or
base64url export form) appears in `Debug` output; a presence marker is
missing; `Debug` output differs between keys differing only in `d`/`k` bytes;
keys differing only in `d`/`k` compare equal; a serde round trip loses `d`
or `k`; the RSA derivation path renders `d: Some`; or any pre-existing
`core::jwk` verification test fails.

## Implementation

The executable checks are the five redaction tests and the pre-existing
verification tests in `src/core/jwk/tests.rs`, run by
`cargo test --offline --lib core::jwk`.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Canary secret absent from both formats; presence marker shown | `src/core/jwk/tests.rs::test_core_jwk_debug_redacts_symmetric_secret` |
| P2 | EC `d` absent from both formats; presence markers shown | `src/core/jwk/tests.rs::test_core_jwk_debug_redacts_ec_private_member` |
| P3 | Key-set `Debug` shows no secret representation | `src/core/jwk/tests.rs::test_core_jwks_debug_redacts_symmetric_secret` |
| P4 | Identical `Debug` for differing secrets; unequal keys; round trip preserves `d`/`k` | `src/core/jwk/tests.rs::test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes`; the `d` round-trip assertions of `test_core_jwk_debug_redacts_ec_private_member` |
| P5 | Pre-existing verification tests pass unchanged | `test_ecdsa_verification`, `test_rsa_pkcs1_verification`, `test_rsa_pss_verification`, `test_hmac_sha256_verification`, `test_eddsa_verification` in `src/core/jwk/tests.rs` |
| P6 | RSA derivation path renders `d: None`, `k: None` | `src/core/jwk/tests.rs::test_core_jwk_rsa_verification_key_debug_has_no_private_material` |

## Rules note

situation/witnesses/P-000013/W-000016-jwk-debug-secret-redaction.md records
a PASS disposition of these fixed-fixture rules at gate head
`b96b920f52e0d8b392edcf0b5d752fa570c2b356`. Its result is bounded to the
named oracle legs; this oracle does not record results.
