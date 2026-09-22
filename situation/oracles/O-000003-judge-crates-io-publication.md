# O-000003 — Judge crates.io publication

## State

designed

## Judges

situation/promises/P-000003-published-on-crates-io-as-oidc-client.md

## Inputs

An empty Rust project consuming the released package, the `oidc-client`
crates.io page, `.github/workflows/ci.yml`, and a retained run of that
workflow on the configured fleet runner.

## Pass

- P1: `cargo add oidc-client` resolves the released package in an empty Rust
  project and that project compiles.
- P2: The crates.io page exposes package documentation, the repository URL,
  and the MIT license.
- P3: The retained CI run executes `cargo fmt --all --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test --all-features`,
  and `cargo deny check` according to the root organization workflow policy.
- P4: The released version parses as a semantic version.

## Fail

- F1: The package cannot be resolved from crates.io by the empty project.
- F2: The crates.io page omits its documentation, repository URL, or license.
- F3: The retained CI run omits a required command, runs on an unconfigured
  route, or fails a required command.
- F4: The released version does not parse as a semantic version.
