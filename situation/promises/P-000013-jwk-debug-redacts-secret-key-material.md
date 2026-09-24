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

State `implemented` cites landed implementation commit
`5ab6d7bbe6ec46fb164a761cce2e6f7f6f80029a`, which replaces the derived
`Debug` implementation and adds the five focused tests named by O-000014.
The stream also reported focused results at `cdfe1ea`; that stream-worktree
head is unresolvable, as retained by
situation/gaps/G-000029-stream-head-shas-unresolvable-on-branch.md, so it is
not used as a current implementation coordinate. The Promise, Oracle, and
Witness records were attached retrospectively in
`9c4013a9d93a93e65084b3475022c43059e64fc2`.

situation/witnesses/P-000013/W-000016-jwk-debug-secret-redaction.md is a
PASS observation from the parent's final gate at
`b96b920f52e0d8b392edcf0b5d752fa570c2b356` and decides each named
O-000014 leg. The state remains `implemented`, not `assured`: the oracle
observes fixed canary encodings but has no structural manual leg deciding the
universal no-disclosure clause across every possible `d`/`k` value.

## Residual

The PASS witness covers its named fixed-fixture legs but does not assure the
universal no-disclosure wording beyond those observations. The promise does
not cover `Display`, serde output (the intentional export path), other
`Debug` sources that reach key bytes without `CoreJsonWebKey`'s `Debug`,
signing, JWE, ES512/P-521, or ID-token claim validation.

## References

- situation/oracles/O-000014-judge-jwk-debug-redacts-secret-key-material.md
- situation/decisions/D-000014-ec-key-type-doc-correction-and-scoped-import-fidelity-supersession.md
- src/core/jwk/mod.rs
- src/core/jwk/tests.rs::test_core_jwk_debug_redacts_symmetric_secret
- src/core/jwk/tests.rs::test_core_jwk_debug_redacts_ec_private_member
- src/core/jwk/tests.rs::test_core_jwks_debug_redacts_symmetric_secret
- src/core/jwk/tests.rs::test_core_jwk_debug_redaction_is_stable_while_partial_eq_distinguishes
- src/core/jwk/tests.rs::test_core_jwk_rsa_verification_key_debug_has_no_private_material
