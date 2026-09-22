# Plans

A plan is a thin container that groups candidates being qualified and
promises being implemented or assured. It does not restate those records and
does not carry design detail.

## File naming

```
plans/<lifecycle>/PLAN-<six digits>-<kebab-case-name>.md
```

Lifecycle directories: `draft/`, `active/`, `done/`, `abandoned/`. The
directory is the lifecycle state; the file does not repeat it.

## Required headings

In this order:

- `Purpose` — one paragraph, when the grouping needs stating
- `Candidates` — links to possibilities being qualified
- `Promises` — links to behavior being implemented or assured
- `Dependencies` — ordering between candidates and promises, when any exists
- `Completion` — the target states that constitute plan completion

## Rules

- A plan contains at least one candidate or promise and a completion
  condition. Nothing else.
- Completion is written as a target condition ("completes when ..."),
  defined by record states, never as a present-tense claim that target
  states already hold.
- Qualification plans complete only when every candidate is promoted,
  rejected, merged, or superseded; promoted candidates link the resulting
  decision, promise, and oracle.
- Moving a plan between lifecycle directories is an explicit commit that
  explains the transition in its message.
- A done plan's records must have reached the states its Completion section
  requires.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
