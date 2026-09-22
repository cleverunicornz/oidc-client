# C-000004 — CI pipeline for formatting, lint, tests, and dependency policy

## State

promoted

## Candidate

Operate a GitHub Actions CI route that runs formatting, linting, the full
feature test suite, and dependency-policy checks on the organization-approved
fleet runner.

## Origin

The imported crate needs a repeatable gate route. The root organization
workflow policy and `.github/workflows/ci.yml` provide the configuration
evidence; G-000004 records the missing workflow-run witness.

## Why consider it

Publication in P-000003 requires a defensible CI route. The repository already
has local gate evidence, but an actual configured-run observation is necessary
before that evidence can support a gate claim.

## Qualification questions

D-000009 selects the configured single `ci` job. The first real fleet-run
Witness remains an assurance question for P-000005/O-000006, not a reason to
represent the already configured route as unselected.

## Candidate approaches

- Use the configured single `ci` job on `cvu-test-runner-x64`.
- Split the checks across independent workflow jobs.
- Rely only on local or container gate runs.

## Disposition

Promoted by
`situation/decisions/D-000009-promote-configured-ci-gate-route.md` into
P-000005 and O-000006. P-000003/O-000003 no longer borrow this route as a
publication clause.
