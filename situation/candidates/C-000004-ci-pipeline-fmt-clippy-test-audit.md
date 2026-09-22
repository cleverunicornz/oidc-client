# C-000004 — CI pipeline for formatting, lint, tests, and dependency policy

## State

qualifying

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

- Does the configured runner provide the pinned native capabilities?
- Does the route execute all four required commands under the organization
  trigger policy?
- Can a retained workflow run evidence the route without treating its YAML
  presence as assurance?

The first real fleet-run Witness remains absent.

## Candidate approaches

- Use the configured single `ci` job on `cvu-test-runner-x64`.
- Split the checks across independent workflow jobs.
- Rely only on local or container gate runs.

## Disposition

Qualification continues under
`situation/plans/active/PLAN-000001-import-es256-publish-consume.md`.
The current route is evidence, not a promotion: P-000003/O-000003 retain the
publication-facing CI contract and G-000004 retains the missing assurance
evidence.
