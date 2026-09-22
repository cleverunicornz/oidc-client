# G-000022 — Semantic discovery rejects the workspace control path

## State

open

## Gap

The semantic discovery service rejects this prepared workspace before returning
results because its index reports the `control/` path as unsafe. The correction
therefore cannot use the default semantic discovery path for its assigned
concepts.

## Relevance

The Bedrock correction protocol requires semantic discovery for docket concepts
when available. This closure used bounded exact search and direct reads after
the semantic service rejected the request; the concern is the unavailable
discovery capability, not the correctness of the records found through that
fallback.

## Evidence

- On 2026-09-22 during correction run
  `20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`, the exposed
  semantic search for the validator docket concepts returned: `rejected unsafe
  input: tree path must not contain '.', '..', or empty components: "control/"`.
- The workspace's initial `git status --short` reported `?? control/`.
- The completed validator docket independently records the same unavailable
  semantic discovery condition and its use of bounded exact search.

## Impact

Targeted semantic discovery cannot narrow correction reads in this workspace.
Exact search and exact reads remain available, so the correction can continue;
the tool failure may make future targeted discovery less efficient or omit
semantic ranking.

## Resolution

none

## References

- situation/AGENTS.md
- situation/gaps/AGENTS.md
