# G-000033 — at_hash Oracle omits supported asymmetric algorithms

## State

open

## Gap

P-000009 promises one documented `at_hash` verification path for shared-secret
and asymmetric signature algorithms as categories, but O-000019's asymmetric
Pass and Fail legs judge only ES256 and ES384. The Oracle does not decide that
path for the other supported asymmetric families (RS256/384/512,
PS256/384/512, and EdDSA), while W-000028 calls its observation
"complete-scope" and G-000032 treats it as complete evidence for P-000009.

## Relevance

The reviewed DELTA adds W-000028 as a current PASS observation intended to
complete P-000009's assurance evidence. The Oracle contract requires the
Oracle to decide every explicit Promise clause within Scope; passing every
listed O-000019 leg cannot establish the broader asymmetric-algorithm clause
that those legs omit.

## Evidence

- `situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md`
  promises a uniform documented path "for both shared-secret and asymmetric
  signature algorithms" and says that, for asymmetric algorithms (with ES256
  given as an example), the path resolves the matching provider JWK.
- `situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md`
  names ES256 in P4, ES384 in P5, and those two algorithms only in F2. It has no
  Pass or Fail leg for the supported RSA, RSA-PSS, or EdDSA paths.
- `situation/witnesses/P-000009/W-000028-documented-at-hash-complete-scope-pass.md`
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

## Impact

W-000028 can retain evidence that every leg actually listed by O-000019 passed,
but O-000019 is not sufficient to judge P-000009's complete declared Scope.
The DELTA therefore cannot truthfully treat W-000028 as complete-scope
assurance evidence for P-000009, and G-000032's blanket statement that all
eight new Witnesses cover their complete Promise scopes is false for this
lineage.

## Resolution

none

## References

- situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md
- situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md
- situation/witnesses/P-000009/W-000028-documented-at-hash-complete-scope-pass.md
- situation/gaps/G-000032-complete-scope-pass-witnesses-outpace-frozen-promise-states.md
- situation/oracles/AGENTS.md
