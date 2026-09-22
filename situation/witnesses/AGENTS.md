# Witnesses

A witness retains one observation from one real run. It is retained
evidence, not a living status. A witness proves an observation happened,
never that a behavior always holds.

## File naming

```
witnesses/P-<promise six digits>/W-<six digits>-<kebab-case-name>.md
```

Witnesses are grouped by the promise they observe.

## Required headings

In this order:

- `Promise` — the observed promise
- `Oracle` — the oracle applied
- `Result` — `PASS`, `FAIL`, `INVALID`, or `BLOCKED`
- `Head` — the exact commit the observation ran against
- `Observed` — the date of the observation
- `Evidence` — URLs, digests, and committed artifact references
- `Oracle legs` — one row per oracle Pass leg, naming the evidence that
  decided it

## Results

- `PASS` — the oracle passed on this observation.
- `FAIL` — the oracle failed; this is evidence, not garbage.
- `INVALID` — the observation could not exercise the oracle meaningfully.
- `BLOCKED` — the observation could not run to completion.

## Rules

- A witness is created only after its parent Promise and Oracle exist, and
  observes one promise under one oracle at one head.
- A witness is immutable from the first closing checkpoint that follows its
  creation; after that, corrections are new witnesses. Until then, on the
  open pull request, it may be corrected in place by a forward commit.
- Evidence must be retained and retrievable in or through the repository: a
  workflow run URL, an artifact digest, or a committed result file. A
  witness never cites only a local scratch path.
- Every Pass leg is independently evidenced; the artifact whose provenance
  is being judged cannot serve as evidence for its own provenance. A PASS
  witness that omits an oracle leg is INVALID, not partial PASS.
- A failed witness explains how a promise's state changed; keep it. Mixed
  outcomes stay mixed; failures stay failures.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
