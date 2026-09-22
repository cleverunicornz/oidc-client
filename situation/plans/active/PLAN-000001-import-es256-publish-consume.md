# PLAN-000001 — Establish imported OIDC behavior and gate evidence

## Candidates

- situation/candidates/C-000003-resolve-rustsec-2023-0071-rsa-marvin-disposition.md
- situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md

## Promises

- situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md
- situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md

## Dependencies

- C-000004's retained fleet-run evidence is needed before the configured CI
  route can support a gate claim.
- C-000003 remains qualified until the dependency-policy route is observed or
  its candidate is otherwise settled.
- P-000001 and P-000002 each require their own Oracle-complete PASS Witness
  before assurance.

## Completion

Completes when C-000003 and C-000004 are each promoted, rejected, merged, or
superseded, and P-000001 and P-000002 are assured by their named Oracles and
PASS Witnesses.
