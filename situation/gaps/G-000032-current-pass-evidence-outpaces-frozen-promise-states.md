# G-000032 — Current PASS evidence outpaces frozen Promise states

## State

open

## Gap

The DELTA adds eight current PASS Witnesses, but the eight Promises they observe
remain literally `implemented` and retain state evidence that says assurance is
absent. Six of those Witnesses pass Oracles whose listed legs decide their
respective complete Promise scopes; W-000028 and W-000032 pass every leg listed
by O-000019 and O-000023, respectively, but those frozen Oracles omit declared
P-000009 asymmetric-algorithm and P-000013 `Clone` clauses. `situation/AGENTS.md`
says a Promise's state records the current disposition after applying its Oracle
to available Witnesses, while the root protocol makes the earlier Promise
records immutable after the previous closing checkpoint. No current record
mechanism reconciles those facts without rewriting frozen Promise text or
creating retroactive replacement lineage.

## Relevance

This closure's substantive interval directly adds W-000027 through W-000034
for P-000008 through P-000014 and P-000016. All eight Witnesses are PASS for
their matching O-000018, O-000019, O-000020, O-000021, O-000022, O-000023,
O-000024, or O-000017 listed legs, respectively. W-000028 and W-000032 are
listed-leg PASS evidence only: O-000019 omits supported RSA, RSA-PSS, and EdDSA
paths, and O-000023 omits P-000013's `Clone` clause. The remaining six current
Oracles decide their respective Promises' complete declared Scopes. In every
case, the linked Promise is the canonical location for the assurance
disposition. The active prepublication-hardening plan therefore remains
literally incomplete, and neither the repository block nor a consumer should
claim these Promises as assured from the new Witnesses alone.

## Evidence

- The current State sections of
  `situation/promises/P-000008-signed-user-info-verification-policy.md`,
  `situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md`,
  `situation/promises/P-000010-user-info-flows-bind-verified-id-token-subject.md`,
  `situation/promises/P-000011-malformed-bearer-token-fails-request-preparation.md`,
  `situation/promises/P-000012-content-type-essence-matches-rfc7231-optional-whitespace.md`,
  `situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md`,
  `situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md`,
  and
  `situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md`
  each state `implemented` and describe a prior incomplete or blocked
  successor observation.
- The DELTA adds W-000027, W-000028, W-000029, W-000030, W-000031, W-000032,
  W-000033, and W-000034 under those Promises. Each has Result `PASS`, names
  its matching O-000018, O-000019, O-000020, O-000021, O-000022, O-000023,
  O-000024, or O-000017, respectively, and supplies one Oracle-legs row for
  every listed Pass and Fail leg. G-000033 records that O-000019 nevertheless
  omits the supported RSA, RSA-PSS, and EdDSA paths; G-000034 records that
  O-000023 omits P-000013's `Clone` clause. Those two Witnesses are listed-leg
  PASS evidence, not sufficient complete-Promise coverage.
- `d80c4555289f63c18a5df8d0bbe88bdbb3f30cdd:situation/context.md` records
  the prior completed closure. `AGENTS.md` states that a record is immutable
  from the first closing checkpoint that follows its creation or change;
  `situation/AGENTS.md` states both that a Promise state records disposition
  and that post-checkpoint records are immutable.

- Corrector correction for closure run
  `20260924T111724Z-295cb33dbeb3b837300c31b0b1a17517095f2393` published
  O-000025 in `857eb25` and O-000026 in `b75e2ea` as self-contained
  complete-scope successors of O-000019 and O-000023. Each has coverage for
  every listed Pass and Fail leg. W-000035 and W-000036 retain successful
  supplemental fixture runs at `48761b267c479b38918e4a139ed5c8fd530f2236`,
  but each is `INVALID` against its corrected successor because it omits
  independently required full-scope legs. Their retained evidence manifests
  remain available; neither record supplies current complete-scope PASS
  evidence.

- The frozen `## Oracle` sections of P-000009 and P-000013 still name only
  O-000019 and O-000023, respectively. O-000025 and O-000026 explicitly
  supersede those partial rules, but the prior-checkpoint immutability boundary
  prevents adding forward links to the Promise records. Neither corrected
  successor has a valid complete-scope PASS Witness, so no current record can
  justify a Promise State transition; this observation adds no automatic
  remedy.

- Closer reconciliation observation for closure run
  `20260924T125359Z-021faa52ab84b38324b49b36cef7dad45c18aa38` at reviewed
  head `8a328d05b7702788e84b139d0204b30c3941c413` (2026-09-24):
  `situation/witnesses/P-000009/W-000037-at-hash-complete-scope-successor-pass.md`
  records a PASS under O-000025 at reachable head
  `c5260aeb4a426f7881c8261ef2101d5a30710c29`, with one Oracle-legs row for
  P1–P16 and F1–F5. `situation/witnesses/P-000013/W-000038-jwk-debug-complete-scope-successor-pass.md`
  records a PASS under O-000026 at that same head, with one row for P1–P11 and
  F1–F8. Both retained evidence manifests verify with `sha256sum -c SHA256SUMS`
  from their respective evidence directories. These are each their successor
  Oracle's own observation, not a composition with an earlier Witness.
- This later observation replaces only the prior statement that neither
  successor has a valid complete-scope PASS Witness. The frozen Promise
  `State` and `Oracle` sections remain as recorded; this addendum neither
  changes this Gap's State or Resolution nor selects a canonical-state
  reconciliation mechanism.
- Validator observation for closure run
  `20260924T125359Z-021faa52ab84b38324b49b36cef7dad45c18aa38` at fixed
  reviewed head `79b53c748dd4c9ae88002b993513f5f591b69fb9` (2026-09-24):
  independent comparison of W-000037 against O-000025 found that the ES256
  and ES384 fixture payloads at `src/verification/tests.rs:1892-1893` and
  `src/verification/tests.rs:2377-2378` contain no `at_hash` claim. W-000037
  acknowledges that absence in its P4 and P5 rows, but substitutes a
  separately computed expected value. That evidence does not decide
  O-000025 P4/P5/P14/F3 against the token's `at_hash`, so W-000037 cannot
  supply a complete-scope PASS at this head. This observation does not
  dispute W-000038's independently complete O-000026 coverage.

## Impact

For the six lineages whose Oracles are sufficient, complete-Promise PASS
evidence remains visible but cannot change the frozen canonical Promise State.
For P-000009 and P-000013, W-000028 and W-000032 remain listed-leg PASS
evidence and are insufficient for assurance independently of that immutability
boundary. In no lineage can a consumer treat the Promise as assured while its
canonical State remains `implemented`; rewriting that State would violate the
record-immutability boundary. The appropriate reconciliation mechanism remains
unresolved.

- The corrected successor rules make the complete evidence requirement visible,
  but their INVALID retained Witnesses do not settle G-000033 or G-000034.
  Those Gaps remain open, P-000009 and P-000013 remain `implemented`, and this
  Gap's unresolved state-record mechanism remains. No consumer may treat
  either Promise as assured.

- Closer impact addendum (2026-09-24): complete-scope successor Witness
  coverage is now retained for both affected lineages. The outstanding concern
  is the frozen Promise-state and forward-link reconciliation mechanism, not
  the prior absence of successor-Witness coverage. This addendum leaves the
  Gap's State and Resolution unchanged.
- Validator impact addendum (2026-09-24): W-000038 retains complete-scope
  successor coverage for P-000013, but W-000037 does not yet retain adequate
  complete-scope evidence for P-000009. P-000009 therefore remains unassured
  on both evidence sufficiency and frozen-state grounds; P-000013 remains
  unassured on the frozen-state reconciliation ground recorded here. This
  additive observation leaves the Gap's State and Resolution unchanged.

## Resolution

none

## References

- AGENTS.md
- situation/AGENTS.md
- situation/plans/active/PLAN-000004-prepublication-hardening.md
- situation/witnesses/P-000008/W-000027-signed-user-info-complete-scope-pass.md
- situation/witnesses/P-000009/W-000028-documented-at-hash-oracle-leg-pass.md
- situation/witnesses/P-000010/W-000029-user-info-subject-binding-complete-scope-pass.md
- situation/witnesses/P-000011/W-000030-malformed-bearer-complete-scope-pass.md
- situation/witnesses/P-000012/W-000031-content-type-complete-scope-pass.md
- situation/witnesses/P-000013/W-000032-jwk-debug-redaction-oracle-leg-pass.md
- situation/witnesses/P-000014/W-000033-client-secret-expiry-complete-scope-pass.md
- situation/witnesses/P-000016/W-000034-client-secret-expiration-sentinel-collisions-pass.md
- situation/oracles/O-000025-judge-documented-at-hash-rsa-pss-eddsa-families.md
- situation/witnesses/P-000009/W-000035-at-hash-rsa-pss-eddsa-oracle-leg-pass.md
- situation/oracles/O-000026-judge-jwk-clone-contract.md
- situation/witnesses/P-000013/W-000036-jwk-clone-contract-oracle-leg-pass.md
- situation/gaps/G-000033-at-hash-oracle-omits-supported-asymmetric-algorithms.md
- situation/gaps/G-000034-jwk-redaction-oracle-omits-clone-contract.md
- situation/witnesses/P-000009/W-000037-at-hash-complete-scope-successor-pass.md
- situation/witnesses/P-000013/W-000038-jwk-debug-complete-scope-successor-pass.md
