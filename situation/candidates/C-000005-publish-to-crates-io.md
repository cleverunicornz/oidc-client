# C-000005 — Publish to crates.io

## State

proposed

## What

Publish the crate to crates.io under the name oidc-client. This requires a
crates.io account/token and the crate being fully ready (complete, tested,
documented).

## Approach

1. Ensure Cargo.toml metadata is complete (name, version, description,
   license, repository, keywords, categories)
2. Write comprehensive rustdoc comments on the public API
3. Add examples in examples/ directory
4. Write a README.md with usage examples
5. cargo publish (requires a crates.io token)

## Dependencies

C-000002, C-000003, C-000004 (publish a complete, clean crate)

## References

- situation/references/R-000001-upstream-pin.md — the target crate name is
  verified available.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item C-000005
(project Status: Todo).
