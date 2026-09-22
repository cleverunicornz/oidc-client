# P-000004 — Dependency policy admits only recorded advisory exceptions

## State

implemented

## Promise

The repository's dependency-policy configuration permits only the three explicit
advisory exceptions for RUSTSEC-2023-0071, RUSTSEC-2025-0056, and
RUSTSEC-2024-0375 under their recorded dispositions. `cargo deny check` remains
the gate for unresolved advisory, license, source, yanked-package, and other
policy findings.

## Scope

The current `deny.toml` advisory configuration and its `cargo deny check`
judgment against this repository's resolved dependency tree. This promise does
not claim that the configured GitHub Actions route has run, that a future
resolver produces the same tree, or that a later advisory needs the same
disposition.

## Oracle

situation/oracles/O-000005-judge-dependency-policy-advisory-exceptions.md

## State evidence

State `implemented` is supported by commit
`8160e19c82e76aadc967ce470d4a4285da5c1617`, which added `deny.toml` with the
recorded dispositions. D-000003 and D-000005 supply the selected rationale.
No PASS witness applies O-000005 to this promise.

## Residual

No Witness yet applies O-000005, and the promise does not pre-approve any
advisory, dependency, or configuration change outside its stated three
exceptions.

## References

- situation/candidates/C-000003-resolve-rustsec-2023-0071-rsa-marvin-disposition.md
- situation/decisions/D-000003-rustsec-2023-0071-rsa-disposition-scoped-ignore.md
- situation/decisions/D-000005-unmaintained-dev-path-advisory-ignores.md
- situation/decisions/D-000008-promote-dependency-policy-contract.md
