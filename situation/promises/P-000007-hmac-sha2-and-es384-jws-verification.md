# P-000007 — HMAC SHA-2 and ES384 JWS verification

## State

assured

## Promise

A consumer using `CoreJsonWebKey` can verify a matching JWS signing input and
signature with `CoreJwsSigningAlgorithm::HmacSha256`, `HmacSha384`,
`HmacSha512`, or `EcdsaP384Sha384` and the corresponding symmetric or EC P-384
public JWK.

## Scope

The public `CoreJsonWebKey` signature-verification path for HS256, HS384,
HS512, and ES384 with matching keys. This promise does not cover ID-token
claim validation, provider metadata, algorithm signing, ES256, ES512, or any
algorithm not named here.

## Oracle

situation/oracles/O-000008-judge-hmac-sha2-and-es384-jws-verification.md

## State evidence

State `assured` cites
situation/oracles/O-000008-judge-hmac-sha2-and-es384-jws-verification.md
and the complete PASS witness
situation/witnesses/P-000007/W-000006-hmac-sha2-and-es384-jws-verification.md,
observed 2026-09-22 at head
`6bc21cdbcf933bcaf52a9f7f7ef33e3dda851bb3`: its two scoped declared Cargo
tests passed every Oracle Pass leg. The imported implementation originates in
commit `eeb0e6d847d7665cc8b0c9e46b9716323ded3c6a`; commit
`95990b41bc4152eea8568ede79dcf3712eaf2451` aligns the public enum
documentation with those dispatches. G-000007 records the documentation
correction's original concern.

## Residual

This promise does not assure signing, ID-token end-to-end behavior, a provider
deployment, or ES512/P-521 support. It does not broaden P-000002's separately
assured ES256 ID-token-verification contract.

## References

- situation/gaps/G-000007-es256-public-docs-contradict-implementation.md
- src/core/jwk/mod.rs
- src/core/jwk/tests.rs::test_hmac_sha256_verification
- src/core/jwk/tests.rs::test_ecdsa_verification
- situation/witnesses/P-000007/W-000006-hmac-sha2-and-es384-jws-verification.md
