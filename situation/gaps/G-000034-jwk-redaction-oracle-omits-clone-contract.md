# G-000034 — JWK redaction Oracle omits the Clone contract

## State

open

## Gap

P-000013 explicitly promises that `CoreJsonWebKey`'s `Clone` behavior is
unchanged alongside equality and serde behavior, but O-000023 has no Pass or
Fail leg for `Clone`. W-000032 therefore supplies listed-leg PASS evidence,
rather than complete-Promise evidence, because it has no Clone observation. A
complete-scope successor now states every Promise clause, but no valid retained
PASS Witness decides that successor at one head.

## Relevance

The reviewed DELTA adds W-000032 as a current listed-leg PASS observation.
O-000026 was initially written as a supplemental, non-superseding rule for
the omitted Clone clause. This correction makes O-000026 the complete-scope
successor of O-000023, but W-000036 remains only a retained Clone fixture
observation and is INVALID for the corrected successor. The repository must
not compose W-000032 and W-000036, which observe different rules and heads,
into complete-Promise evidence.

## Evidence

- `situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md` states
  in its Promise section that "Clone, `PartialEq`/`Eq`, and serde serialization
  and deserialization of those fields are unchanged."
- `situation/oracles/O-000023-judge-jwk-debug-redaction-complete-scope.md` P5/F3
  judge equality and serde behavior; no Pass or Fail leg names Clone.
- `situation/witnesses/P-000013/W-000032-jwk-debug-redaction-oracle-leg-pass.md`
  has one row for every listed O-000023 leg but contains no Clone evidence.
- `src/core/jwk/mod.rs` derives `Clone` for `CoreJsonWebKey`; that source fact
  shows a plausible manual decision is available, not that the earlier Oracle
  declared or W-000032 applied it.
- At reviewed head `bbd8b169c9010aeb7d9f2288b19b670a405b526c`, O-000026
  expressly judged only the omitted Clone clause and did not supersede
  O-000023. Its retained W-000036 fixture run passed those listed supplemental
  legs at `48761b267c479b38918e4a139ed5c8fd530f2236`.
- The validator docket for closure run
  `20260924T111724Z-295cb33dbeb3b837300c31b0b1a17517095f2393` identifies that
  partial, non-superseding record as an Oracle-contract failure:
  https://github.com/cleverunicornz/oidc-client/pull/4#issuecomment-5813485966.
- This forward correction makes O-000026 a self-contained complete-scope
  successor and classifies W-000036 as INVALID because it omits the
  successor's direct and container redaction, equality/serde, presence,
  non-secret-field, and derived-RSA legs. No retained witness presently
  decides every successor leg at one head.

- Closer reconciliation observation for closure run
  `20260924T125359Z-021faa52ab84b38324b49b36cef7dad45c18aa38` at reviewed
  head `8a328d05b7702788e84b139d0204b30c3941c413` (2026-09-24):
  `situation/witnesses/P-000013/W-000038-jwk-debug-complete-scope-successor-pass.md`
  records a self-contained PASS under O-000026 at reachable head
  `c5260aeb4a426f7881c8261ef2101d5a30710c29`. Its Oracle-legs table supplies
  one row for each P1–P11 and F1–F8, and its retained evidence manifest
  verifies with `sha256sum -c SHA256SUMS` from its evidence directory. This
  observation is not composed with W-000032 or W-000036.

## Impact

O-000023 is insufficient to judge P-000013's complete declared Scope.
O-000026 now supplies a valid complete-scope judgment rule, but W-000036 is
not adequate PASS evidence for it. P-000013 remains `implemented` and
unassured while G-000032 is unresolved. This Gap remains open until a valid
complete-scope successor Witness supplies adequate PASS evidence; the retained
Clone fixture result is not that evidence.

- Closer impact addendum (2026-09-24): the absence of a valid retained
  complete-scope successor PASS Witness no longer describes the available
  evidence. O-000023's historical Clone omission and the frozen canonical
  `implemented` Promise State remain recorded; this additive observation leaves
  this Gap's State and Resolution unchanged while
  G-000032 retains the state-reconciliation concern.

## Resolution

none

## References

- situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md
- situation/oracles/O-000023-judge-jwk-debug-redaction-complete-scope.md
- situation/witnesses/P-000013/W-000032-jwk-debug-redaction-oracle-leg-pass.md
- situation/gaps/G-000032-current-pass-evidence-outpaces-frozen-promise-states.md
- situation/oracles/AGENTS.md
- situation/oracles/O-000026-judge-jwk-clone-contract.md
- situation/witnesses/P-000013/W-000036-jwk-clone-contract-oracle-leg-pass.md
- situation/witnesses/P-000013/W-000038-jwk-debug-complete-scope-successor-pass.md
