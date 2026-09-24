# G-000033 — at_hash Oracle omits supported asymmetric algorithms

## State

closed

## Gap

P-000009 promises one documented `at_hash` verification path for shared-secret
and asymmetric signature algorithms as categories, but O-000019's asymmetric
Pass and Fail legs judge only ES256 and ES384. The Oracle does not decide that
path for the other supported asymmetric families (RS256/384/512,
PS256/384/512, and EdDSA). W-000028 is therefore listed-leg PASS evidence
rather than complete-Promise evidence; earlier current-interval W-000028 and
G-000032 wording had characterized it as complete-scope evidence.

## Relevance

The reviewed DELTA adds W-000028 as a current listed-leg PASS observation. The
correction removes its complete-scope representation, but neither a successor
Oracle nor a valid Witness decides the omitted asymmetric families. The Oracle
contract requires the Oracle to decide every explicit Promise clause within
Scope; passing every listed O-000019 leg cannot establish the broader
asymmetric-algorithm clause that those legs omit.

## Evidence

- `situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md`
  promises a uniform documented path "for both shared-secret and asymmetric
  signature algorithms" and says that, for asymmetric algorithms (with ES256
  given as an example), the path resolves the matching provider JWK.
- `situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md`
  names ES256 in P4, ES384 in P5, and those two algorithms only in F2. It has no
  Pass or Fail leg for the supported RSA, RSA-PSS, or EdDSA paths.
- `situation/witnesses/P-000009/W-000028-documented-at-hash-oracle-leg-pass.md`
  executes ES256 and ES384 and supplies structural evidence for those same EC
  paths, but supplies no Oracle leg deciding RS256/384/512, PS256/384/512, or
  EdDSA through `IdToken::verification_key` and `AccessTokenHash::from_token`.
- `src/core/mod.rs` (`CoreJwsSigningAlgorithm`) and `src/core/jwk/mod.rs`
  (`CoreJsonWebKey::verify_signature` and `hash_bytes`) expose verification and
  access-token hashing for the omitted asymmetric families; they are not
  hypothetical algorithms outside the crate's supported surface.
- Validator observation for closure run
  `20260924T094258Z-50bf95f61913526682a3550d07fa6b1d96d78935` at fixed
  reviewed head `8e61dc268952aa7f434cd164f7d0569a31113530` (2026-09-24).

- Closer reconciliation for closure run
  `20260924T111724Z-295cb33dbeb3b837300c31b0b1a17517095f2393` reviewed the
  admitted DELTA
  `7f1025e1b91c0f367f05528e7efb93846d437a81..295cb33dbeb3b837300c31b0b1a17517095f2393`.
  O-000025 declares Pass and Fail legs for RS256/384/512, PS256/384/512, and
  EdDSA; W-000035 records PASS for every one of those legs at
  `48761b267c479b38918e4a139ed5c8fd530f2236`, with retained, checksum-verified
  fixture evidence.

## Impact

W-000028 retains evidence that every leg actually listed by O-000019 passed,
but O-000019 is not sufficient to judge P-000009's complete declared Scope.
P-000009 remains `implemented` and unassured until a sufficient successor
Oracle has a valid Witness. G-000032 now distinguishes this listed-leg PASS
from sufficient complete-Promise coverage; this Gap remains open because the
omitted paths are not yet decided.

- Closer reconciliation closes the omitted-family concern: O-000025 and
  W-000035 now decide the previously omitted supported asymmetric families.
  P-000009's frozen canonical `implemented` State is a distinct reconciliation
  concern retained by G-000032.

## Resolution

closed — O-000025 and W-000035 supply implemented-Oracle and PASS-Witness
evidence for every previously omitted RSA, RSA-PSS, and EdDSA family at
`48761b267c479b38918e4a139ed5c8fd530f2236`. P-000009's frozen canonical State
remains a separate concern in G-000032.

## References

- situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md
- situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md
- situation/witnesses/P-000009/W-000028-documented-at-hash-oracle-leg-pass.md
- situation/gaps/G-000032-current-pass-evidence-outpaces-frozen-promise-states.md
- situation/oracles/AGENTS.md
- situation/oracles/O-000025-judge-documented-at-hash-rsa-pss-eddsa-families.md
- situation/witnesses/P-000009/W-000035-at-hash-rsa-pss-eddsa-oracle-leg-pass.md
