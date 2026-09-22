# G-000022 — Semantic discovery rejects the workspace control path

## State

closed

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
- Bookkeeping observation (PR #3 branch `bank3/bookkeeping`, 2026-09-22):
  `git log --all -- control/` is empty at this branch — no commit in the
  repository's history contains a `control/` path. The current worktree does
  contain an untracked `control/` scratch directory (`git status --short`
  reports `?? control/`), so the rejected path is workspace-local rather than
  committed repository content.

## Impact

Targeted semantic discovery cannot narrow correction reads in this workspace.
Exact search and exact reads remain available, so the correction can continue;
the tool failure may make future targeted discovery less efficient or omit
semantic ranking.

## Resolution

closed — environmental/tooling disposition. The rejected `control/` path was
untracked worktree-local scratch (the workspace's initial
`git status --short` reported `?? control/`), never repository content: the
repository's history contains no `control/` path. The semantic index
rejection described that workspace scratch, not committed bytes, and no
repository record depends on the unavailable discovery capability — the
correction work proceeded with exact search and direct reads. No repository
change is required; the observation above is retained as the record of the
tool failure.

## References

- situation/AGENTS.md
- situation/gaps/AGENTS.md
