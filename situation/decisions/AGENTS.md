# Decisions

A decision records why a choice collapsed. It preserves rationale so the
same path is not relitigated without new evidence. Decisions are append-only:
supersede, never edit.

## File naming

```
D-<six digits>-<kebab-case-name>.md
```

## Required headings

In this order:

- `Status` — `accepted`, `superseded`, or `reversed`
- `Date` — when the decision was reached or materialized
- `Context` — the situation that forced a choice
- `Evidence` — links to the records and facts that informed the choice
  (witnesses, oracles, gaps, references)
- `Decision` — what was chosen, stated directly
- `Why` — the reason the choice collapsed this way
- `Rejected alternatives` — what was not chosen and why
- `Consequences` — what follows from the decision
- `Revisit when` — optional; the condition under which this decision may be
  reopened

Superseded decisions additionally link `Supersedes` and `Superseded by`.

## Relationship to other records

A decision states the why; an invariant states the resulting rule — when a
decision produces a binding rule, write the invariant and link this
decision as its Basis. When a promise is refuted or superseded, a decision
records why and the promise links it in its state evidence. A candidate
becomes behavior only through a decision that promotes it into a
falsifiable promise with an oracle; that promotion links the candidate and
every record it creates.

## Rules

- One decision per file, immutable from the first closing checkpoint that
  follows its creation or change. Until then, on the open pull request, it
  may be corrected in place by a forward commit.
- Supersession replaces a decision with a new record; both remain.
- A decision without evidence is a preference, not a decision; say which it
  is.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
