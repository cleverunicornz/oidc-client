# situation/ knowledge rules

Read the root AGENTS.md protocol block before relying on anything here.

## Namespace rules

Each record namespace below carries its own AGENTS.md; read it before creating
or editing records in that namespace.

- `decisions/` — append-only; supersede, never edit.
- `gaps/` — append-only observations; separately assigned State/Resolution updates.
- `promises/` — falsifiable behavior with lifecycle state.
- `oracles/` — how Promises are judged; require ≥1 executable leg for implemented state.
- `witnesses/` — immutable observations from actual runs; created after their parent Promise and Oracle.
- `plans/` — group Candidates and Promises into work without restating them.
- `references/` — supporting depth owned by their citing record.
- `context.md` — closure state; only checkpoint commits write it.

## Initial state

This repository is newly created. The initial knowledge base captures the
scoped delivery contract for the oidc-client crate, derived from evidence
gathered in the Poda Chat repository (cleverunicornz/poda-chat).

## Context

See `situation/context.md` for the current phase and implementation map.
