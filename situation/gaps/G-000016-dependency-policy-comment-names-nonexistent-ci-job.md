# G-000016 — Dependency-policy comment names a nonexistent CI job

## State

closed

## Gap

The opening comment in `deny.toml` says the configuration is enforced by a CI
`deny` job, but `.github/workflows/ci.yml` declares one `ci` job whose
`Dependency policy` step runs `cargo deny check`.

## Relevance

P-000005/O-000006 identify the configured CI boundary. Accurate job naming
helps readers trace the dependency-policy command to that route without
implying an independent workflow job.

## Evidence

- `deny.toml` lines 1–2 name the CI `deny` job.
- `.github/workflows/ci.yml` declares `jobs.ci` and names its final step
  `Dependency policy`; no `deny` job is declared.

## Impact

The command remains configured, but a reader may search for a separate job or
misunderstand the scope of the configured CI route.

## Resolution

closed — the `deny.toml` opening comment now names the actual configuration:
the "Dependency policy" step of the single `ci` job in
`.github/workflows/ci.yml`, which runs `cargo deny check`. No separate `deny`
job exists or is implied.

## References

- `deny.toml`
- `.github/workflows/ci.yml`
- situation/promises/P-000005-configured-ci-gate-route.md
- situation/oracles/O-000006-judge-configured-ci-gate-route.md
