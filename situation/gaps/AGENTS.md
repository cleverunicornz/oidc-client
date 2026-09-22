# Gaps

A gap records an absence, concern, or uncertainty encountered during
repository work, including minor or tentative concerns. Recording it
establishes that the concern was raised, not that a defect exists or that
a remedy is necessary.

## File naming

```
G-<six digits>-<kebab-case-name>.md
```

## Required headings

In this order:

- `State` — lifecycle state
- `Gap` — the absence, concern, or uncertainty at its actual certainty
- `Relevance` — the repository behavior or work in which it arose
- `Evidence` — what prompted it, distinguishing observation from
  interpretation
- `Impact` — known or possible consequences; an unresolved effect is valid
- `Resolution` — current resolution, or explicitly `none`
- `References` — optional retained depth

## States

```
open         recorded and unresolved
addressing   a linked Candidate, Promise, or Plan is actively resolving it
closed       cited evidence or a Decision settles the absence or uncertainty
accepted     a Decision explicitly tolerates it
superseded   replaced by a more accurate Gap
```

`addressing`, `closed`, `accepted`, and `superseded` link the record,
commit, or Decision that justifies the state.

## Rules

- Surface concerns encountered in the assigned work; this is incidental
  reporting, not an assignment to hunt for gaps. An open gap alone neither
  invalidates a passing oracle nor assigns its investigation.
- Prefer an admitted gap over invented certainty. A gap preserves
  uncertainty without narrowing a promise or creating a promise, candidate,
  decision, or remedy.
- Observations are append-only after a closing checkpoint: later work may
  append attributed observations under Evidence, Impact, or References.
  Reporting leaves State and Resolution unchanged; separately assigned
  disposition work may update those two sections by forward commit.
- Create a new gap in state `open` with Resolution explicitly `none`.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
