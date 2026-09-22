# References

References hold retained depth: pinned external facts, research, long-form
material, rejected paths, and historical context that a record points to but
that is too large or too precise to live inline.

## File naming

```
R-<six digits>-<kebab-case-name>.md
```

Depth owned by a single record may instead live in an owned subdirectory:

```
references/P-000001/
```

## Required headings

In this order:

- `Subject` — what the reference retains
- `Owner` — the record or invariant that cites it
- `Facts` — the retained material, with sources and retrieval dates
- `Verification` — what has been verified, by what means and when, and what
  remains unverified or pending
- `References` — optional pointers to sources

## Rules

- A reference must be linked from at least one record; unowned files here
  are misplaced.
- References are depth, not law. They never override a promise, oracle,
  decision, or invariant.
- External public files use full URLs; external private files use declared
  `Private: owner/repo@<ref>#<path>` coordinates. Inability to fetch a
  declared-private reference never grounds inventing its contents.
- Facts carry their retrieval date; claims not yet verified stay marked as
  such. A pending verification is named together with the work that owns it.
- Git is the canonical store for historical bytes; copy donor material here
  only when active retrieval needs an in-tree reference.

## Reference discipline

Reference discipline is defined in the root `AGENTS.md` protocol block.
