# Invariants

An invariant states a binding repository rule: behavior or policy every
change must preserve. Invariants do not prove themselves; a rule that needs
proving is a promise with an oracle.

## File naming

```
I-<six digits>-<kebab-case-name>.md
```

## Required headings

In this order:

- `Invariant` — the rule, stated directly and checkably
- `Basis` — the decision, promise, or obligation that grounds the rule
- `Priority` — `standard` or `critical`
- `References` — optional links

## Relationship to other records

- A decision states the why; an invariant states the resulting rule. When a
  decision produces a binding rule, the invariant links that decision as its
  Basis.
- Every assured promise is invariant behavior without a separate record;
  this namespace carries rules that bind independently of any single
  promise's assurance state.

## Rules

- One rule per file, stated so a reviewer can check a change against it.
- An invariant without a Basis is a preference, not a rule; say which it is.
- Changing an invariant requires a decision explaining the change; the prior
  statement remains in git history. The root `AGENTS.md` repository block
  summarizes critical rules; these records are the canonical detail.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
