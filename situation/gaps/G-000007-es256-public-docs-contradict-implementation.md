# G-000007 — ES256 public documentation contradicts implementation

## State

closed

## Gap

The public `CoreJwsSigningAlgorithm::EcdsaP256Sha256` documentation says
"currently unsupported", while the carried source dispatches that enum variant
to P-256 verification and P-000002 records an implemented ES256 path.

## Relevance

Consumer-facing documentation and repository orientation must not make a
stronger or weaker ES256 claim than the recorded implementation and assurance
state. This conflict is distinct from the missing full-ID-token fixture in
G-000006.

## Evidence

- `src/core/mod.rs` lines 634–636 label the ES256 enum variant "currently
  unsupported".
- `src/core/jwk/mod.rs` lines 308–315 dispatch the same variant with a P-256
  key to `crypto::verify_ec_signature`.
- `src/core/crypto.rs` lines 99–145 construct and use a P-256 verifying key.
- `situation/decisions/D-000007-bound-es256-support-claims-to-evidence.md`
  records the evidence-bound claim decision.

## Impact

A consumer reading generated API documentation could conclude that the ES256
path is unavailable, while an unqualified repository claim could incorrectly
call it first-class. Neither interpretation accurately expresses the current
implemented-but-unassured state.

Corrector observation in closure run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`: P-000002 later
became assured for ES256, while P-000007 separately assures the named HMAC
SHA-2 and ES384 verification paths.

## Resolution

closed — the enum documentation was corrected per variant against the
dispatch code: `HmacSha256`, `HmacSha384`, and `HmacSha512` use the `hmac`
arms; `EcdsaP256Sha256` and `EcdsaP384Sha384` dispatch to
`crypto::verify_ec_signature` with their matching P-256 and P-384 arms.
Those five variants dropped the false "(currently unsupported)" label.
`EcdsaP521Sha512` retains the label: it has no verification arm and falls
through to `SignatureVerificationError::UnsupportedAlg`, and no P-521 crate
is a declared dependency. The documentation reports the implemented surface
without asserting assurance: P-000002 separately assures ES256, and P-000007
separately assures HMAC SHA-2 and ES384 verification.

## References

- `situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`
- `situation/decisions/D-000007-bound-es256-support-claims-to-evidence.md` —
  the claims-bounding decision this correction executes.
- situation/promises/P-000007-hmac-sha2-and-es384-jws-verification.md
- situation/oracles/O-000008-judge-hmac-sha2-and-es384-jws-verification.md
- situation/witnesses/P-000007/W-000006-hmac-sha2-and-es384-jws-verification.md
- `src/core/jwk/mod.rs` and `src/core/crypto.rs` — the dispatch code each
  documentation comment was verified against.

