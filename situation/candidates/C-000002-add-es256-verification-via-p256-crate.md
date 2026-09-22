# C-000002 — Add ES256 verification via p256 crate

## State

proposed

## What

Add ES256 (ECDSA P-256 SHA-256) ID token verification to the JWS
verification path. The upstream code already has the algorithm enum with
ECDSA variants marked unsupported — wire them up.

## Approach

The openidconnect crate uses the jsonwebtoken-style approach where JWS
verification dispatches on the algorithm header. The ECDSA variants
(EcdsaP256Sha256 = ES256, EcdsaP384Sha384 = ES384) already exist in the
CoreJwsSigningAlgorithm enum. The gap is in the verification path where EC
keys from the JWK are not handled.

1. Add p256 (and optionally p384) as dependencies
2. In the JWK-to-verifying-key conversion, handle the EllipticCurve key type
   with crv=P-256: extract x and y coordinates, construct a p256::PublicKey
3. In the JWS verification dispatch, add the ES256 case: use
   p256::ecdsa::VerifyingKey to verify the signature
4. Update discovery metadata parsing to accept ES256 in
   id_token_signing_alg_values_supported
5. Add integration tests: a provider signing with ES256 (can use Kanidm or
   a test fixture)

## Evidence

Kanidm 1.11.0 defaults to ES256; the Poda Chat qualification rig
(Private: cleverunicornz/poda-chat@main#situation/references/G-000006/native-auth-qualification.md)
provides a real ES256-signing provider for testing.

## Dependencies

C-000001 (need the imported code first)

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item C-000002
(project Status: Todo).
