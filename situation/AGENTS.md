# situation/ knowledge rules

Read the root AGENTS.md protocol block before relying on anything here.

## Namespace rules

Each record namespace below carries its own AGENTS.md; read it before creating
or editing records in that namespace.

- `decisions/` — append-only; supersede, never edit.
- `invariants/` — binding repository rules grounded in a stated Basis.
- `gaps/` — append-only observations; separately assigned State/Resolution updates.
- `candidates/` — evidence-derived possibilities; behavior only via promotion Decision.
- `promises/` — falsifiable behavior with lifecycle state.
- `oracles/` — how Promises are judged; require ≥1 executable leg for implemented state.
- `witnesses/` — immutable observations from actual runs; created after their parent Promise and Oracle.
- `plans/` — group Candidates and Promises into work without restating them.
- `references/` — supporting depth owned by their citing record.
- `context.md` — closure state; only checkpoint commits write it.

## Record ID alignment

Repository record IDs align with the cleverunicornz Project #20 IDs
(projectV2 #20) for materialized concepts. Genuinely new repository-local
records take the next free ID in their record class.

## Record provenance

Every record ends with a `Provenance` section stating its materialization
date and source.

## Initial state

This repository is newly created. The initial knowledge base captures the
scoped delivery contract for the oidc-client crate, derived from evidence
gathered in the Poda Chat repository (cleverunicornz/poda-chat).

## Context

See `situation/context.md` for the current phase and implementation map.
