# Candidates

A candidate records an evidence-derived possible response to a gap or a plan
step. It is a possibility, not a commitment; it becomes behavior only
through a decision that promotes it into a falsifiable promise with an
oracle.

## File naming

```
C-<six digits>-<kebab-case-name>.md
```

## Required headings

In this order:

- `State` — the lifecycle state
- `What` — the possibility, stated directly
- `Approach` — tasks or technical approach, when worked out
- `Evidence` — deriving evidence from prior work, when carried
- `Dependencies` — ordering with other candidates or records, when any exist
- `References` — optional links

## States

```
proposed     recorded possibility, not under qualification
qualifying   a plan is actively qualifying it
promoted     a Decision turned it into a Promise and Oracle; link all three
rejected     a Decision declined it; link the Decision
merged       folded into another candidate; link the survivor
superseded   replaced by a more accurate candidate; link the replacement
```

## Rules

- A candidate names the evidence that suggests it; a candidate without
  evidence is speculation, not a candidate.
- Promotion is a decision that creates the promise and oracle atomically and
  links all of them; until then the candidate stays a possibility.
- Candidates do not assign work; a plan does that. Recording a candidate
  neither commits to it nor obligates qualification.
- Rejected, merged, and superseded candidates are retained; they are
  evidence.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
