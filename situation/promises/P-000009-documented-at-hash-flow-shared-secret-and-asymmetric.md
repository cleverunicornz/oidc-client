# P-000009 — Documented at_hash flow for shared-secret and asymmetric ID tokens

## State

implemented

## Promise

A single uniform documented call path verifies the access-token hash of an ID
token for both shared-secret and asymmetric signature algorithms. For an ID
token signed with `HS256`, `HS384`, or `HS512`, a confidential verifier
resolves the effective verification key from the client secret and the
documented flow (`IdToken::verification_key` followed by
`AccessTokenHash::from_token`) succeeds even when the provider JSON Web Key Set
is empty. For asymmetric algorithms (e.g., `ES256`), the same call resolves the
matching key from the provider JSON Web Key Set. When the token's algorithm
uses a shared secret and the verifier holds no client secret, the call returns
an error rather than panicking. A substituted access token fails the hash
comparison.

## Scope

The public key-resolution and hash-verification path used by the crate-root
tutorials: `IdToken::verification_key`, the underlying verifier key resolution,
and `AccessTokenHash::from_token`. The pre-existing borrowed-key resolution
method `IdToken::signing_key` remains available unchanged with its
JWKS-only scope, documented. This promise does not cover `c_hash`,
ID-token claim validation beyond what the flow exercises, or providers that
sign with algorithms the verifier has not allowed.

## Oracle

situation/oracles/O-000010-judge-documented-at-hash-flow.md

## State evidence

State `implemented` cites implementation commit
`78529bc948824f2dc3f6eb2270b2e9c5aa98876c`, which adds
`IdToken::verification_key`, the tutorial migration, and the focused fixture
test named by O-000010. The Promise, Oracle, and Witness records were
attached retrospectively in `9c4013a9d93a93e65084b3475022c43059e64fc2`.

situation/witnesses/P-000009/W-000012-documented-at-hash-flow.md is a PASS
observation from the parent's final gate at
`b96b920f52e0d8b392edcf0b5d752fa570c2b356` and decides every named
O-000010 leg. The state remains `implemented`, not `assured`: the named
fixtures cover HS256 and ES256, while HS384, HS512, and ES384 lack
independently recorded coverage. Those limits remain in Residual.

## Residual

`HS384`/`HS512` and `ES384` follow the same dispatch as the tested `HS256` and
`ES256` but are not exercised by a named test. JWE-encrypted ID tokens remain
unsupported crate-wide. The end-to-end network flow with a live provider is not
exercised.

## References

- src/id_token/mod.rs (`IdToken::verification_key`; `signing_key` scope docs)
- src/verification/mod.rs (owned key resolution on the claims verifier)
- src/verification/tests.rs::test_id_token_verification_key_at_hash
- src/lib.rs (both crate-root tutorials migrated to `verification_key`)
- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
