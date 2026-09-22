# P-000007 — HMAC SHA-2 and ES384 JWS verification

## State

implemented

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

State `implemented` is supported by the imported implementation commit
`eeb0e6d847d7665cc8b0c9e46b9716323ded3c6a`: its
`src/core/jwk/mod.rs` dispatches HS256, HS384, HS512, and ES384 to the matching
verification implementations, and `src/core/jwk/tests.rs` retains their fixed
verification fixtures. Commit
`95990b41bc4152eea8568ede79dcf3712eaf2451` aligns the public enum
Documentation with those dispatches. G-000007 records why the documentation
correction requires a capability lineage rather than a claim by comment alone.

## Residual

This promise does not assure signing, ID-token end-to-end behavior, a provider
deployment, or ES512/P-521 support. It does not broaden P-000002's separately
assured ES256 ID-token-verification contract.

## References

- situation/gaps/G-000007-es256-public-docs-contradict-implementation.md
- src/core/jwk/mod.rs
- src/core/jwk/tests.rs::test_hmac_sha256_verification
- src/core/jwk/tests.rs::test_ecdsa_verification
