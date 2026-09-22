# G-000011 — Upstream reference ownership and caller migration

## State

closed

## Gap

Before correction, R-000001 was a flat file at the root of
`situation/references/` and named a set of citing records rather than one
owning record. The References contract requires retained depth to be a child
of the one record that links it.

## Relevance

R-000001 is the central provenance dependency for the changed import lineage.
Its owner and stable coordinate had to be corrected together with every direct
caller so the resulting graph remained traversable.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`.
- `situation/references/AGENTS.md` under "Structure" requires paths such as
  `references/P-000001/` or `references/D-000014/` and states that a Reference
  is a child of the record that links it.
- At correction base, the only retained depth file was
  `situation/references/R-000001-upstream-pin.md`; it was directly beside the
  namespace contract and its `## Owner` section named an incomplete citing-record
  inventory instead of one owner.

## Impact

The provenance content was usable, but its authority could not be resolved
through the owning-record structure required by the protocol. A relocation
required every direct caller to move in the same change.

## Resolution

closed — R-000001 now lives at
`situation/references/D-000004/R-000001-upstream-pin.md` as retained depth
owned by D-000004, and every direct caller was migrated in the same commit.

## References

- `situation/references/AGENTS.md`
- `situation/references/D-000004/R-000001-upstream-pin.md`
