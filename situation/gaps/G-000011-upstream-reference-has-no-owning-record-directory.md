# G-000011 — Upstream Reference has no owning-record directory

## State

open

## Gap

R-000001 remains a flat file at the root of `situation/references/` and names a
set of citing records rather than one owning record. The installed References
contract requires retained depth to be organized beneath the record that owns
it and says every Reference is a child of that record.

## Relevance

R-000001 is the central provenance dependency for the changed import Promise,
Candidate, Decision, Invariant, README orientation, and context-correction Gap.
Its ownership and stable coordinate are therefore part of the affected
lineage, not an unrelated register cleanup.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`.
- `situation/references/AGENTS.md` under "Structure" requires paths such as
  `references/P-000001/` or `references/D-000014/` and states that a Reference
  is a child of the record that links it.
- The only retained depth file is
  `situation/references/R-000001-upstream-pin.md`, directly beside the namespace
  contract.
- Its `## Owner` section says it is cited by D-000001, P-000003, C-000001,
  C-000005, I-000001, and G-000002, while additional current links from D-000002,
  D-000003, D-000004, G-000001, and P-000001 show that this is neither a unique
  owner nor a complete citing-record inventory.

## Impact

The provenance content is present and usable, but its authority cannot be
resolved through the owning-record structure required by the protocol. A move
without a clean update of every caller would also leave broken lineage links.

## Resolution

none

## References

- `situation/references/AGENTS.md`
- `situation/references/R-000001-upstream-pin.md`
