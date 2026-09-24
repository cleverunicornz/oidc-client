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

- Corrected closer reconciliation observation for closure run
  `20260924T125359Z-021faa52ab84b38324b49b36cef7dad45c18aa38` at reviewed
  head `79b53c748dd4c9ae88002b993513f5f591b69fb9` (2026-09-24):
  `situation/witnesses/P-000009/W-000037-at-hash-complete-scope-successor-pass.md`
  retains a successful focused test and a verified evidence manifest at
  `c5260aeb4a426f7881c8261ef2101d5a30710c29`, but is INVALID under O-000025.
  Its ES256 and ES384 hand-signed payloads omit embedded `at_hash`, leaving
  P4, P5, P14, and F3 undecided; it does not supply complete-scope PASS evidence
  and is not composed with W-000028 or W-000035.
- Validator observation for closure run
  `20260924T125359Z-021faa52ab84b38324b49b36cef7dad45c18aa38` at fixed
  reviewed head `79b53c748dd4c9ae88002b993513f5f591b69fb9` (2026-09-24):
  W-000037's P4 and P5 rows expressly say that the hand-signed ES256 and
  ES384 payloads carry no embedded `at_hash`. The cited fixtures confirm that
  omission at `src/verification/tests.rs:1892-1893` and
  `src/verification/tests.rs:2377-2378`; they compare computed hashes only
  with a separately computed expected value. They therefore do not decide
  O-000025 P4/P5 or the every-family P14/F3 clauses against the token's
  `at_hash`. A PASS Witness that omits those legs is INVALID rather than a
  complete-scope PASS.

## Impact

O-000019 is insufficient to judge P-000009's complete declared Scope.
O-000025 now supplies a valid complete-scope judgment rule, but W-000035 is
not adequate PASS evidence for it. P-000009 remains `implemented` and
unassured while G-000032 is unresolved. This Gap remains open until a valid
complete-scope successor Witness supplies adequate PASS evidence; the retained
supplemental fixture result is not that evidence.

- Corrected closer impact addendum (2026-09-24): the original absence of a
  valid retained complete-scope successor PASS Witness remains for P-000009.
  W-000037 is INVALID because its ES256/ES384 fixtures omit the claims required
  to decide O-000025 P4/P5/P14/F3. O-000019's historical omission and the
  frozen canonical `implemented` Promise State remain recorded; this correction
  leaves this Gap's State and Resolution unchanged while G-000032 retains the
  state-reconciliation concern.
- Validator impact addendum (2026-09-24): the original absence of adequate
  complete-scope successor evidence remains for P-000009 at the fixed
  reviewed head. W-000037 cannot settle it while its ES256/ES384 fixtures
  omit the claim named by O-000025 P4/P5/P14/F3. This additive observation
  leaves the Gap's State and Resolution unchanged.

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
- situation/witnesses/P-000009/W-000037-at-hash-complete-scope-successor-pass.md
