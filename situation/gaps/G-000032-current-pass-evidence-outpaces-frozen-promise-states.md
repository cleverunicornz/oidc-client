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

## Impact

For the six lineages whose Oracles are sufficient, complete-Promise PASS
evidence remains visible but cannot change the frozen canonical Promise State.
For P-000009 and P-000013, W-000028 and W-000032 remain listed-leg PASS
evidence and are insufficient for assurance independently of that immutability
boundary. In no lineage can a consumer treat the Promise as assured while its
canonical State remains `implemented`; rewriting that State would violate the
record-immutability boundary. The appropriate reconciliation mechanism remains
unresolved.

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
