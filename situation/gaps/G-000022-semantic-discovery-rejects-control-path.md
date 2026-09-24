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
- Validator observation for closure run
  `20260922T183045Z-72a7dd2ce3a328bb5eb238e632d55ccf0cd74d6d` at fixed
  reviewed head `463cdca78619fb6f41ebe2aaac056f5fdca87b9e` (2026-09-22):
  both semantic-index status discovery and a concept-focused semantic search
  again returned `rejected unsafe input: tree path must not contain '.', '..',
  or empty components: "control/"`. Validation continued with bounded exact
  search and direct reads; this additive observation does not change the
  Gap's State or Resolution.
- Corrector observation for closure run
  `20260922T183045Z-72a7dd2ce3a328bb5eb238e632d55ccf0cd74d6d` at correction-base
  head `8844bda1a87ea2795d312977541dbea522b9e278` (2026-09-22): a
  concept-focused semantic search for PLAN-000003, P-000001, and P-000005
  returned `rejected unsafe input: tree path must not contain '.', '..', or
  empty components: "control/"`. The corrector proceeded with the relayed
  docket and bounded direct reads; this additive observation does not change
  the Gap's State or Resolution.
- Validator observation for closure run
  `20260924T063914Z-cd10a6d6107e401ba795c255cfb2bc4df2b7e8e2` at fixed
  reviewed head `731f85ed1be41e8b6a937bea7bcb68b6c60991ec` (2026-09-24):
  semantic-index status discovery and a changed-work concept search both
  returned the same unsafe-`control/`-path rejection. Validation continued
  with the exact substantive diff, exact search, and direct reads; this
  additive observation leaves the Gap's State and Resolution unchanged.

- Closer observation for closure run
  `20260924T094258Z-50bf95f61913526682a3550d07fa6b1d96d78935` at opening
  checkpoint `fc107c1f60fab2b0ac2c168fefd2193a4945bc11` (2026-09-24): both
  semantic-index status and a concept-focused semantic search returned
  `rejected unsafe input: tree path must not contain '.', '..', or empty
  components: "control/"`. The closer continued with the exact DELTA diff,
  exact searches, and direct reads; this additive observation leaves the
  Gap's State and Resolution unchanged.

- Closer observation for closure run
  `20260924T111724Z-295cb33dbeb3b837300c31b0b1a17517095f2393` at opening
  checkpoint `c9e07d9eb28282fd56ffbcce87cc33f4093a3f9b` (2026-09-24): both
  exposed semantic-index status and a concept-focused semantic search again
  returned `rejected unsafe input: tree path must not contain '.', '..', or
  empty components: "control/"`. The closer continued with the exact DELTA
  diff, bounded exact search, and direct reads; this additive observation
  leaves the Gap's State and Resolution unchanged.

- Closer observation for closure run
  `20260924T125359Z-021faa52ab84b38324b49b36cef7dad45c18aa38` at opening
  checkpoint `8a328d05b7702788e84b139d0204b30c3941c413` (2026-09-24): both
  exposed semantic-index status and a concept-focused semantic search returned
  `rejected unsafe input: tree path must not contain '.', '..', or empty
  components: "control/"`. The closer continued with the exact DELTA diff,
  bounded exact search, and direct reads; this additive observation leaves the
  Gap's State and Resolution unchanged.

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
