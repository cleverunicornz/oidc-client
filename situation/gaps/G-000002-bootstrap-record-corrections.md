# G-000002 — Bootstrap context drift

## State

closed

## Gap

Before the corrector's context repair, the ordinary (non-Closure-state)
sections of `situation/context.md` carried factual drift against the admitted
tree and pinned evidence:

1. It stated openidconnect 4.0.1 was "released 2026-07-06"; the actual
   crates.io release is 2025-07-06T22:08:11Z.
2. It carried no upstream commit SHA; the 4.0.1 tag is pinned to
   `b639b5d39eac6903238867aeb2b29326502e6b26`.
3. It described the donor as `MIT OR Apache-2.0`, while the pinned tag is
   MIT-only with one `LICENSE` file.
4. Its current-phase text said the repository had "no code yet", while the
   admitted tree contained the imported Rust crate.

The same text omitted the required explicit `OWNED` classification and
repeated refuted ES256 research: the admitted 4.0.1 donor contains the
P-256/P-384 verification paths and ECDSA metadata parsing.

## Relevance

`situation/context.md` is the canonical phase and implementation map.
`situation/AGENTS.md` reserves only its `## Closure state` section to
checkpoint commits. The ordinary context facts therefore required a forward
record correction, while the closure-state lines remained untouched.

## Evidence

Direct comparison on 2026-09-22 of situation/context.md against R-000001
(git ls-remote, crates.io API, tag Cargo.toml; scout verification of the
45 tracked files at pin b639b5d).

- Backport closure observation, run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c`, at opening
  checkpoint `92946acf13d02a67caab38cb64444a902217fae4`: the current-phase
  text said the repository had "no code yet", while the admitted opening tree
  contained `Cargo.toml`, Rust source under `src/`, integration tests under
  `tests/`, and the imported crate's `README.upstream.md`.

- The root repository block had already been corrected in
  `fbaa1198070a45d80d55945f96551696a67da04e:AGENTS.md`; it did not make the
  canonical context accurate.

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`: the installed protocol reserves
  only `situation/context.md`'s `## Closure state` section to checkpoint
  commits (`situation/AGENTS.md`, "Closure state"; root `AGENTS.md`, lines
  56–60). The identity, phase, implementation map, origin evidence, upstream
  coordinate, and dependency sections are ordinary canonical context and may
  be corrected without touching the orchestrator-owned closure-state lines.
  The current context also retains two ES256 claims omitted from the four-item
  list above: lines 23–28 say openidconnect dropped ECDSA in the 2.x→4.x
  rewrite and that no Rust OIDC library supports ES256, while the admitted
  source and `README.upstream.md` show the 4.0.1 donor's P-256/P-384
  verification paths. It also does not record the explicit `OWNED`
  classification required by `situation/AGENTS.md` under "Repository
  ownership."

## Impact

Before correction, readers beginning at the canonical phase and implementation
map received an inaccurate orientation about the implementation, donor
provenance, license, ownership, and ES256 research boundary.

## Resolution

closed — `28fa45a` corrects the ordinary context sections listed above without
altering the checkpoint-owned `## Closure state` lines.

## References

- situation/references/D-000004/R-000001-upstream-pin.md — the verified pin facts
  the corrections carry into context.md.

## Provenance

Materialized 2026-09-22 by the Phase A records lane from direct inspection
of situation/context.md against the assignment's pinned upstream facts and
the Phase A upstream-facts scout verification.
