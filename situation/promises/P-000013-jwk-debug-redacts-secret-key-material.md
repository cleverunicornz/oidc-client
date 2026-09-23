# P-000013 — JWK Debug output never discloses secret key material

## State

implemented

## Promise

A consumer formatting a `CoreJsonWebKey` with `Debug` (`{:?}` or `{:#?}`),
directly or nested inside a container such as `JsonWebKeySet<CoreJsonWebKey>`,
observes every non-secret field and the presence or absence of private key
material, and never observes the bytes of the EC/RSA private member (`d`) or
the symmetric secret (`k`). Clone, `PartialEq`/`Eq`, and serde serialization
and deserialization of those fields are unchanged: serialization remains the
intentional export path for `d` and `k`, and keys differing only in `d`/`k`
still compare unequal.

## Scope

The `std::fmt::Debug` implementation of `CoreJsonWebKey` in
`src/core/jwk/mod.rs`, and the `Debug` output of containers that delegate to
it (for example `JsonWebKeySet<CoreJsonWebKey>`). Covers presence-only
rendering of `d` and `k` and the unchanged behavior of the remaining derives.
Custom `JsonWebKey` implementations supplied by library consumers own their
own `Debug` and are outside this promise, as are secrets held outside
`CoreJsonWebKey` (for example `CoreHmacKey::secret` or
`CoreRsaPrivateSigningKey::key_pair`).

## Oracle

situation/oracles/O-000014-judge-jwk-debug-redacts-secret-key-material.md

## State evidence

State `implemented` cites the implementing commit `cdfe1ea` (branch
`fix/prepublication-hardening`): it replaces the derived `Debug` with a manual
implementation and adds five focused tests to `src/core/jwk/tests.rs`. The
focused suite `cargo test --offline --lib core::jwk` passed 26 tests, 0
failed, and `cargo check --offline` finished clean at that commit. A
falsification run against the pre-fix derived `Debug` failed exactly the four
new redaction tests
(`test_core_jwk_debug_redacts_symmetric_secret`,
`test_core_jwk_debug_redacts_ec_private_member`,
`test_core_jwks_debug_redacts_symmetric_secret`,
`test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes`),
confirming the tests defend the contract. No Witness exists yet: witnesses
are to be collected under `situation/witnesses/P-000013/` by the parent's
final gate, not by this stream.

## Residual

Assurance is not complete: no Oracle has been applied to a Witness, so this
promise is not `assured` and is not yet invariant behavior under the
supersession rule. The promise does not cover `Display`, serde output (the
intentional export path), other `Debug` sources that reach key bytes without
`CoreJsonWebKey`'s `Debug`, signing, JWE, ES512/P-521, or ID-token claim
validation.

## References

- situation/oracles/O-000014-judge-jwk-debug-redacts-secret-key-material.md
- situation/decisions/D-000014-ec-key-type-doc-correction-and-scoped-import-fidelity-supersession.md
- src/core/jwk/mod.rs
- src/core/jwk/tests.rs::test_core_jwk_debug_redacts_symmetric_secret
- src/core/jwk/tests.rs::test_core_jwk_debug_redacts_ec_private_member
- src/core/jwk/tests.rs::test_core_jwks_debug_redacts_symmetric_secret
- src/core/jwk/tests.rs::test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes
- src/core/jwk/tests.rs::test_core_jwk_rsa_verification_key_debug_has_no_private_material
