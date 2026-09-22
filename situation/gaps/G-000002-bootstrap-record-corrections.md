# G-000002 — Bootstrap record corrections pending on context.md

## State

open

## Gap

The bootstrap-written situation/context.md carries three factual drifts
against the pinned evidence in R-000001:

1. It states openidconnect 4.0.1 was "released 2026-07-06"; the actual
   crates.io release is 2025-07-06T22:08:11Z.
2. It carried no upstream commit SHA; the 4.0.1 tag is now pinned to commit
   b639b5d39eac6903238867aeb2b29326502e6b26.
3. The repository blocks claim the code is "MIT OR Apache-2.0" while the
   tag is MIT-only. RESOLVED BY EVIDENCE: the Phase A upstream-facts scout
   verified at pin b639b5d that the tag carries a single MIT LICENSE file,
   Cargo.toml license "MIT", and zero Apache references (R-000001). Only
   the wording in the Project #20 bodies and this repository's repository
   block remains to be corrected; those corrections are routed through
   PR #1 by Main per the Phase A steering (only the protocol and
   organization blocks are agent-protected).

## Relevance

situation/context.md is closure state written only by checkpoint commits
(situation/AGENTS.md), so corrections cannot be applied by ordinary record
edits; they must ride a closure. Other records (I-000001, D-000001,
C-000001) already carry the corrected MIT-only fact with one-line
correction notes.

## Evidence

Direct comparison on 2026-09-22 of situation/context.md against R-000001
(git ls-remote, crates.io API, tag Cargo.toml; scout verification of the
45 tracked files at pin b639b5d).

## Impact

Until corrected, context.md misstates the release year, omits the commit
pin, and carries a wrong license characterization in the document other
records treat as the phase map.

## Resolution

none — corrections 1, 2, and the context.md license wording ride the FIRST
closure checkpoint commit that writes situation/context.md after the Phase
A records land. context.md is not edited by this batch.

## References

- situation/references/R-000001-upstream-pin.md — the verified pin facts
  the corrections carry into context.md.

## Provenance

Materialized 2026-09-22 by the Phase A records lane from direct inspection
of situation/context.md against the assignment's pinned upstream facts and
the Phase A upstream-facts scout verification.
