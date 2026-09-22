# D-000008 — Promote dependency-policy contract

## Status

accepted

## Date

2026-09-22

## Context

C-000003's exact-advisory-disposition approach was selected in D-000003 and
D-000005 and materialized in `deny.toml`, but the Candidate was left without
the Promise and Oracle required for promotion. The existing configuration and
its decisions need local, falsifiable lineage without claiming an unobserved
fleet run.

## Evidence

- situation/candidates/C-000003-resolve-rustsec-2023-0071-rsa-marvin-disposition.md
- situation/decisions/D-000003-rustsec-2023-0071-rsa-disposition-scoped-ignore.md
- situation/decisions/D-000005-unmaintained-dev-path-advisory-ignores.md
- `deny.toml`
- Commit `8160e19c82e76aadc967ce470d4a4285da5c1617`, which added the current
  configuration.

## Decision

Promote C-000003 into P-000004 with O-000005. The Promise is limited to the
recorded local dependency-policy configuration; it does not select or assert a
CI workflow observation.

## Why

The existing Decisions already select exact exceptions and the configuration
exists. Promotion makes that selected local behavior traversable while keeping
assurance dependent on O-000005 evidence.

## Rejected alternatives

- Treat the selected configuration as a Candidate indefinitely: rejected
  because D-000003 and D-000005 already chose and materialized it.
- Promote it without a Promise and Oracle: rejected by the Candidate promotion
  contract.
- Broaden the contract to future advisories or a fleet-run claim: rejected
  because neither is established by the recorded configuration.

## Consequences

C-000003 is promoted. P-000004 is implemented but unassured until a Witness
applies O-000005; the independent configured-CI behavior is P-000005.

## Revisit when

The advisory set, `deny.toml`, resolved dependency tree, or dependency-policy
command changes materially.
