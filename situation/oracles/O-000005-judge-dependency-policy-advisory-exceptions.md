# O-000005 — Judge recorded dependency-policy advisory exceptions

## State

implemented

## Judges

situation/promises/P-000004-dependency-policy-advisory-exceptions.md

## Inputs

The current `deny.toml`, D-000003, D-000005, the resolved dependency tree, and
`cargo deny check` output.

## Pass

- P1: The advisory ignore list contains exactly RUSTSEC-2023-0071,
  RUSTSEC-2025-0056, and RUSTSEC-2024-0375, each with the disposition recorded
  by D-000003 or D-000005.
- P2: `cargo deny check` succeeds against the resolved dependency tree with that
  configuration.

## Fail

- F1: The advisory ignore list adds an unrecorded exception, omits a named
  exception, or no longer matches its recorded disposition.
- F2: `cargo deny check` reports an advisory, license, source,
  yanked-package, or other dependency-policy failure.

## Implementation

`cargo deny check` evaluates the repository's `deny.toml` against the resolved
dependency tree. Its configured CI execution is a separate P-000005 boundary.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | The configured ignore identifiers and their Decision links are compared against the declared three-exception contract. | manual |
| P2 | The checker exits successfully for the resolved tree and configuration. | `cargo deny check` |
| F1 | An extra, missing, or mismatched exception is detected by comparison with D-000003 and D-000005. | manual |
| F2 | The checker exits unsuccessfully when a dependency-policy finding remains. | `cargo deny check` |
