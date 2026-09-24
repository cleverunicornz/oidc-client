# G-000032 — Complete-scope PASS witnesses outpace frozen Promise states

## State

open

## Gap

The DELTA adds eight current complete-scope PASS witnesses, but the eight
Promises they observe remain literally `implemented` and retain state evidence
that says assurance is absent. `situation/AGENTS.md` says a Promise's state
records the current disposition after applying its Oracle to available
Witnesses, while the root protocol makes the earlier Promise records immutable
after the previous closing checkpoint. No current record mechanism reconciles
those two facts without rewriting frozen Promise text or creating retroactive
replacement lineage.

## Relevance

This closure's substantive interval directly adds W-000027 through W-000034
for P-000008 through P-000014 and P-000016. Each Witness is a PASS of its
current complete-scope Oracle and names evidence for every Pass and Fail leg,
but the linked Promise is the canonical location for the assurance disposition.
The active prepublication-hardening plan therefore remains literally
incomplete, and neither the repository block nor a consumer should claim these
Promises as assured from the new Witnesses alone.

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
  every listed Pass and Fail leg.
- `d80c4555289f63c18a5df8d0bbe88bdbb3f30cdd:situation/context.md` records
  the prior completed closure. `AGENTS.md` states that a record is immutable
  from the first closing checkpoint that follows its creation or change;
  `situation/AGENTS.md` states both that a Promise state records disposition
  and that post-checkpoint records are immutable.

## Impact

The evidence remains visible and does not establish a source-code failure.
However, the current canonical Promise states do not express the disposition
that their current Oracle/Witness lineage supports. Treating those Promises as
assured would bypass their State sections; rewriting them would violate the
record-immutability boundary. The appropriate reconciliation mechanism remains
unresolved.

## Resolution

none

## References

- AGENTS.md
- situation/AGENTS.md
- situation/plans/active/PLAN-000004-prepublication-hardening.md
- situation/witnesses/P-000008/W-000027-signed-user-info-complete-scope-pass.md
- situation/witnesses/P-000009/W-000028-documented-at-hash-complete-scope-pass.md
- situation/witnesses/P-000010/W-000029-user-info-subject-binding-complete-scope-pass.md
- situation/witnesses/P-000011/W-000030-malformed-bearer-complete-scope-pass.md
- situation/witnesses/P-000012/W-000031-content-type-complete-scope-pass.md
- situation/witnesses/P-000013/W-000032-jwk-debug-redaction-complete-scope-pass.md
- situation/witnesses/P-000014/W-000033-client-secret-expiry-complete-scope-pass.md
- situation/witnesses/P-000016/W-000034-client-secret-expiration-sentinel-collisions-pass.md
