# D-000002 — Version oidc-client 4.1.0, continuing the upstream version lineage

## Status

accepted

## Date

2026-09-22

## Context

The faithful import carries the openidconnect 4.0.1 code forward under the
new crate name `oidc-client`
(situation/decisions/D-000001-create-oidc-client-as-a-standalone-public-crate-not-a-vendored-fork.md).
The first version declared in the
root `Cargo.toml` must choose between continuing the upstream version
lineage and starting a fresh line. Existing consumers of openidconnect
4.0.x — including Poda Chat — need a migration signal that maps their
dependency line onto this crate without guessing compatibility.

## Evidence

- situation/references/D-000004/R-000001-upstream-pin.md — the upstream pin: the
  4.0.1 tag declares `version = "4.0.1"`, and the crates.io name
  `oidc-client` was unregistered as of 2026-09-22.
- situation/candidates/C-000001-import-openidconnect-v4-0-1-code-with-attribution.md
  — the import this version number labels.
- situation/decisions/D-000001-create-oidc-client-as-a-standalone-public-crate-not-a-vendored-fork.md
  — the standalone-crate decision that makes the version a real choice.

## Decision

The root `Cargo.toml` declares `name = "oidc-client"` and
`version = "4.1.0"`: one minor step above the upstream 4.0.1 the code was
taken at. The version line continues from the upstream lineage; subsequent
releases follow semver from 4.1.0.

## Why

Continuity signaling. The code IS openidconnect 4.0.1 plus this
repository's additions; `4.1.0` communicates a minor-step upgrade from
4.0.1, so a consumer reading `oidc-client = "4.1.0"` immediately knows the
behavioral baseline and the expected drop-in compatibility. The version
lineage carries the compatibility story the README tells.

## Rejected alternatives

- `0.1.0` (fresh line): discards the lineage and miscommunicates maturity —
  the code is a battle-tested 4.x codebase, not an early experiment. It
  also breaks the semver expectations of existing 4.0.1 consumers, who
  would read 0.x as unstable-API churn.
- Re-issuing `4.0.1` under the new name: collides conceptually with the
  upstream's exact release and leaves no version space for this
  repository's additive changes to be distinguished.

## Consequences

The crate starts at 4.1.0; breaking changes require 5.0.0. The historical
reference lock `Cargo-1.65.lock` retains its root entry
`openidconnect 4.0.1` verbatim as an inert artifact — cargo does not read
that filename, and rewriting it would violate the import fidelity rules in
situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md.

## Revisit when

The first breaking change ships (next major), or publication collisions
with the upstream version history on crates.io appear.

## Provenance

Recorded 2026-09-22 by the Phase B import lane, ahead of the version
declaration it governs; upstream version fact verified at the pin the same
day (situation/references/D-000004/R-000001-upstream-pin.md).
