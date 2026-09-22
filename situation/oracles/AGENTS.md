# Oracles

An oracle defines the judgment rule for a promise. It states what inputs are
examined and what constitutes pass or fail. It does not execute itself; an
oracle may be designed before its executable implementation exists.

## File naming

```
O-<six digits>-<kebab-case-name>.md
```

## Required headings

In this order:

- `State` — `designed` or `implemented`
- `Judges` — link to the promise
- `Inputs` — what evidence the oracle examines
- `Pass` — the conditions that pass
- `Fail` — the conditions that fail
- `Implementation` — present only when state is `implemented`
- `Implementation coverage` — present when state is `implemented`; one row
  per Pass and Fail leg, naming the executable decision or marking it
  `manual`

## States

- `designed` — the judgment rule is stated; no executable check exists yet.
- `implemented` — a test, workflow, or checker command exists and runs, and
  the `Implementation` section links it. Requires at least one executable
  leg; a wholly manual oracle remains `designed`.

## Rules

- Pass and fail conditions must be decidable from the stated inputs.
- The oracle judges the promise's stated contract inside its declared Scope.
  It must not broaden the promise or judge outside-Scope behavior; incidental
  concerns surface as Gaps. Outside-Scope behavior is not a failed leg.
- The oracle does not record results; witnesses do that.
- One oracle judges one promise. A judgment rule serving multiple promises
  becomes one oracle per promise, linked in References.
- An executable check may not be credited with a leg it does not decide; a
  manual leg is valid, but witnesses must carry direct evidence for it.
- Changing a pass or fail condition after witnesses exist requires a new
  oracle superseding the old one.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
