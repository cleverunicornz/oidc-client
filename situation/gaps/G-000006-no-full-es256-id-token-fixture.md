# G-000006 — No full ES256 ID-token fixture is retained

## State

closed

## Gap

No retained test or witness exercises the public ID-token verifier with a
compact token whose protected header declares `ES256` and whose signature,
issuer, audience, nonce, and expiry are all checked. The existing P-256 unit
test directly supplies `CoreJwsSigningAlgorithm::EcdsaP256Sha256` to a raw
signature verifier, but its encoded protected header decodes to `{"alg":"RS256"}`.

## Relevance

P-000002 claims ES256 ID-token verification. O-000002 deliberately leaves its
full-ID-token P4/F4 legs manual, so this gap records the missing evidence rather
than narrowing the implemented behavior or implying assurance.

## Evidence

- `src/core/jwk/tests.rs` lines 291–376 define
  `test_ecdsa_verification`, use a P-256 key and the ES256 enum directly, and
  pass the fixed signing input at lines 296–301.
- Decoding the fixed input's first base64url segment during this closure yields
  `{"alg":"RS256","kid":"bilbo.baggins@hobbiton.example"}`.
- `situation/oracles/O-000002-judge-es256-verification.md` records P4/F4 as
  manual; W-000002 is INVALID for those unexecuted legs.
- Corrector observation in closure run
  `20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`: O-000002's
  formerly manual P4/F4 behavior is exercised by the public-verifier fixture
  retained in W-000003.

## Impact

The retained unit evidence establishes low-level P-256 signature behavior but
cannot alone assure the complete ES256 ID-token contract or its public
algorithm-header path.

## Resolution

closed — a full ES256 ID-token fixture is retained:
`src/verification/tests.rs::test_es256_id_token_verified_claims` drives a
compact token whose protected header declares `ES256` through the public
`CoreIdTokenVerifier` path (public client verifier with ES256 selected via
the public `set_allowed_algs` builder), checking signature, issuer,
audience, nonce, and expiry, and rejecting wrong issuer, wrong audience,
wrong nonce, expired, tampered-signature, and wrong-key variants. Witness
situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md
records the PASS run at head
`00d45a7c06f3704915550ad96610a9f9d5e68279`; P-000002 moved to `assured`.

## References

- `situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`
- `situation/oracles/O-000002-judge-es256-verification.md`
- `situation/witnesses/P-000002/W-000002-import-head-es256-unit-observation.md`
- situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md —
  the complete PASS witness that resolves this gap.
