# D-000009 — Promote configured CI gate route

## Status

accepted

## Date

2026-09-22

## Context

C-000004's configured CI route exists in `.github/workflows/ci.yml`, but its
Candidate was left qualifying while P-000003/O-000003 incorrectly treated that
independent route as part of publication. The selected configuration needs its
own Promise and Oracle without turning YAML presence into a passing workflow
observation.

## Evidence

- situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md
- `.github/workflows/ci.yml`
- situation/gaps/G-000004-no-assured-ci-witness-route.md
- Commit `8160e19c82e76aadc967ce470d4a4285da5c1617`, which added the route.

## Decision

Promote C-000004 into P-000005 with O-000006. The configured single `ci` job
is the selected route; O-000006 remains designed until retained configured-run
evidence exists.

## Why

The configuration is an existing selected local behavior, while a successful
fleet execution remains unobserved. A dedicated contract prevents publication
from borrowing CI assurance and preserves that distinction.

## Rejected alternatives

- Leave C-000004 qualifying while treating CI as a P-000003 clause: rejected
  because it makes the same route both unselected and committed.
- Treat the configuration file as a passing workflow witness: rejected because
  no retained configured-run output exists.
- Split the configured route into unselected jobs: rejected because the current
  single job is the selected configuration.

## Consequences

C-000004 is promoted. P-000005 is implemented but unassured; it must gain a
Witness applying O-000006 before it supports a CI gate claim. Publication
remains P-000003's separate future behavior.

## Revisit when

The workflow triggers, runner, capability policy, or gate-command set changes.
