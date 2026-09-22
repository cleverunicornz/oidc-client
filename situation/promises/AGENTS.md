# Promises

A promise states falsifiable behavior the repository claims, intends, or is
attempting to provide; it does not prove itself and contains no design
rationale.

## File naming

```
P-<six digits>-<kebab-case-name>.md
```

Witnesses for a promise live under `witnesses/P-<same six digits>/`.

## Required headings

In this order:

- `State` — the lifecycle state
- `Promise` — the behavior, stated directly
- `Scope` — what the promise covers
- `Oracle` — link to the judgment rule
- `State evidence` — links justifying the current state; `hypothesis`
  states that no feasibility evidence exists yet
- `Residual` — what the promise deliberately does not assure; required at
  `assured`, earlier only when it narrows `Scope`
- `References` — optional links into `references/`

## States

```
hypothesis      stated, no feasibility evidence
qualifying      feasibility or evidence assessment in progress
qualified       evidence shows implementation is feasible
implementing    implementation work is active
implemented     code exists; assurance not yet complete
assuring        oracle being applied, witnesses being collected
assured         named oracle passed on a named witness; residual recorded
refuted         evidence shows the promise cannot or should not hold
withdrawn       intentionally abandoned without refutation
superseded      replaced by another promise; link the successor
```

## Rules

- A promise need not visit every state; simple work may move
  `hypothesis → implementing → implemented → assured`. It states behavior,
  not implementation detail; design rationale belongs in a decision.
  Granularity is a capability a consumer can rely on — never one per file
  or function, never too coarse to judge falsifiably.
- Every state transition cites its cause: `assured` cites the oracle and a
  passing witness covering every asserted behavior (anything unexercised is
  named in `Residual`); `refuted` cites a failing witness and a decision;
  `superseded` cites the replacement promise and a decision. State never
  rests on uncited judgment.
- Retrospective promises are valid: cite only evidence that exists; never
  backdate or invent a witness or a predeclared oracle.
- Every assured promise is invariant behavior; changing it requires a
  superseding promise, a decision, a replacement oracle, and new witnesses.
- Refuted and withdrawn promises are retained; they are evidence.
- A promise promoted from a Candidate links that Candidate and the
  selecting Decision; direct feature work may create a promise without a
  Candidate.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
