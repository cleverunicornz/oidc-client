# G-000002 — Bootstrap record corrections pending on context.md

## State

open

## Gap

The checkpoint-owned `situation/context.md` carries four factual drifts
against the admitted tree and pinned evidence:

1. It states openidconnect 4.0.1 was "released 2026-07-06"; the actual
   crates.io release is 2025-07-06T22:08:11Z.
2. It carries no upstream commit SHA; the 4.0.1 tag is pinned to
   `b639b5d39eac6903238867aeb2b29326502e6b26`.
3. It describes the donor as `MIT OR Apache-2.0`, while the pinned tag is
   MIT-only with one `LICENSE` file.
4. Its current-phase text says the repository has "no code yet", while the
   admitted tree contains the imported Rust crate.

## Relevance

`situation/context.md` is closure state written only by checkpoint commits
(situation/AGENTS.md), so these corrections cannot be applied by ordinary
record edits; they must ride a checkpoint. The root `AGENTS.md` repository
block and the affected attribution records have already been corrected, but
they cannot correct the checkpoint-owned context.

## Evidence

Direct comparison on 2026-09-22 of situation/context.md against R-000001
(git ls-remote, crates.io API, tag Cargo.toml; scout verification of the
45 tracked files at pin b639b5d).

- Backport closure observation, run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c`, at opening
  checkpoint `92946acf13d02a67caab38cb64444a902217fae4`: the current-phase
  text says the repository has "no code yet", while the admitted opening
  tree contains `Cargo.toml`, Rust source under `src/`, integration tests
  under `tests/`, and the imported crate's `README.upstream.md`. This is an
  additional phase/map drift; the closer cannot edit `context.md`, which the
  checkpoint contract reserves to the orchestrator.

- This closure corrected the root repository block in
  `fbaa1198070a45d80d55945f96551696a67da04e:AGENTS.md`; its ownership,
  license, and ES256 wording no longer repeat the stale Project #20 claims.
  The unresolved portion is now the checkpoint-owned `context.md` text.

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

Until corrected, `context.md` misstates the release year, omits the commit
pin, mischaracterizes the license, and describes an implementation-free
repository. Readers who begin at the canonical phase and implementation map
receive an inaccurate orientation.

## Resolution

none — corrections 1–4 must ride the next closure checkpoint that writes
`situation/context.md`. The closer does not edit that checkpoint-owned file.

## References

- situation/references/R-000001-upstream-pin.md — the verified pin facts
  the corrections carry into context.md.

## Provenance

Materialized 2026-09-22 by the Phase A records lane from direct inspection
of situation/context.md against the assignment's pinned upstream facts and
the Phase A upstream-facts scout verification.
