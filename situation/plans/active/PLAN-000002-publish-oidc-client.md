# PLAN-000002 — Publish oidc-client with selected release gates

## Candidates

None.

## Promises

- situation/promises/P-000003-published-on-crates-io-as-oidc-client.md
- situation/promises/P-000004-dependency-policy-advisory-exceptions.md
- situation/promises/P-000005-configured-ci-gate-route.md

## Dependencies

- P-000004 and P-000005 require their own Oracle-complete PASS Witnesses before
  they can support the release boundary for P-000003.
- P-000003 requires a released package and its own O-000003 PASS Witness; it
  does not include downstream Poda Chat adoption.

## Completion

Completes when P-000003, P-000004, and P-000005 are assured by their named
Oracles and PASS Witnesses.
