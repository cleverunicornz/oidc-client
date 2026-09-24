# G-000030 — Offline registry cache blocks the sentinel-collision oracle

## State

open

## Gap

The predeclared O-000017 command cannot execute in its required `--offline`
mode on the current verification environment because the available crates.io
index has no `base64` package entry. This is an execution-environment absence,
not a conclusion about the `ClientSecretExpiration` behavior.

## Relevance

P-000016's collision-rejection behavior is implemented in the DELTA but needs
a runnable O-000017 observation before it can carry any assurance claim. The
oracle intentionally uses offline resolution to match the recorded command and
must not be silently replaced with a different online command.

## Evidence

Observed 2026-09-24 at head
`226ef0be1a7da6969d7840b80a426500b6e27df5`:

- `cargo test --offline --lib --features accept-rfc3339-timestamps
  client_secret_expiration_` exited `101` before compiling tests with
  `no matching package named base64 found` in the offline crates.io index.
- situation/witnesses/P-000016/W-000019-client-secret-expiration-sentinel-collision-oracle-blocked.md
  retains the BLOCKED observation. Its sanitized command output is committed at
  `situation/witnesses/evidence/W-000019/cargo-test-client-secret-expiration.log`
  with the SHA-256 recorded in the adjacent `SHA256SUMS` file.

## Impact

No O-000017 Pass or Fail leg was decided. P-000016 remains `implemented` and
unassured; the blocked command is not evidence that the source behavior fails.
The existing P-000014 three-state assurance and the root verification route are
outside this Gap's scope.

## Resolution

none

## References

- situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md
- situation/oracles/O-000017-judge-client-secret-expiration-sentinel-collisions.md
- situation/witnesses/P-000016/W-000019-client-secret-expiration-sentinel-collision-oracle-blocked.md
