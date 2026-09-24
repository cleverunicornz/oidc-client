# G-000033 — at_hash Oracle omits supported asymmetric algorithms

## State

open

## Gap

P-000009 promises one documented `at_hash` verification path for shared-secret
and asymmetric signature algorithms as categories. O-000019's asymmetric Pass
and Fail legs judge only ES256 and ES384, so it does not decide that path for
the other supported asymmetric families (RS256/384/512, PS256/384/512, and
EdDSA). A complete-scope successor now states every Promise clause, but no
valid retained PASS Witness decides that successor at one head.

## Relevance

The reviewed DELTA adds W-000028 as a current listed-leg PASS observation.
O-000025 was initially written as a supplemental, non-superseding rule for
the omitted asymmetric families. This correction makes O-000025 the
complete-scope successor of O-000019, but W-000035 remains only a retained
supplemental fixture observation and is INVALID for the corrected successor.
The repository must not compose W-000028 and W-000035, which observe different
rules and heads, into complete-Promise evidence.

## Evidence

- `situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md`
  promises a uniform documented path "for both shared-secret and asymmetric
  signature algorithms" and says that, for asymmetric algorithms (with ES256
  given as an example), the path resolves the matching provider JWK.
- `situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md`
  names ES256 in P4, ES384 in P5, and those two algorithms only in F2. It has
  no Pass or Fail leg for the supported RSA, RSA-PSS, or EdDSA paths.
- `situation/witnesses/P-000009/W-000028-documented-at-hash-oracle-leg-pass.md`
  executes ES256 and ES384 and supplies structural evidence for those same EC
  paths, but supplies no Oracle leg deciding RS256/384/512, PS256/384/512, or
  EdDSA through `IdToken::verification_key` and `AccessTokenHash::from_token`.
- At reviewed head `bbd8b169c9010aeb7d9f2288b19b670a405b526c`, O-000025
  expressly judged only the formerly omitted asymmetric families and did not
  supersede O-000019. Its retained W-000035 fixture run passed those listed
  supplemental legs at `48761b267c479b38918e4a139ed5c8fd530f2236`.
- The validator docket for closure run
  `20260924T111724Z-295cb33dbeb3b837300c31b0b1a17517095f2393` identifies that
  partial, non-superseding record as an Oracle-contract failure:
  https://github.com/cleverunicornz/oidc-client/pull/4#issuecomment-5813485966.
- This forward correction makes O-000025 a self-contained complete-scope
  successor and classifies W-000035 as INVALID because it omits the
  successor's shared-secret, ES256/ES384, `signing_key`, and tutorial legs.
  No retained witness presently decides every successor leg at one head.

## Impact

O-000019 is insufficient to judge P-000009's complete declared Scope.
O-000025 now supplies a valid complete-scope judgment rule, but W-000035 is
not adequate PASS evidence for it. P-000009 remains `implemented` and
unassured while G-000032 is unresolved. This Gap remains open until a valid
complete-scope successor Witness supplies adequate PASS evidence; the retained
supplemental fixture result is not that evidence.

## Resolution

none

## References

- situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md
- situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md
- situation/witnesses/P-000009/W-000028-documented-at-hash-oracle-leg-pass.md
- situation/gaps/G-000032-current-pass-evidence-outpaces-frozen-promise-states.md
- situation/oracles/AGENTS.md
- situation/oracles/O-000025-judge-documented-at-hash-rsa-pss-eddsa-families.md
- situation/witnesses/P-000009/W-000035-at-hash-rsa-pss-eddsa-oracle-leg-pass.md
