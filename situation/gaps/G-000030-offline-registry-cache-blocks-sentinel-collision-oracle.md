# G-000030 — Offline registry cache blocks the sentinel-collision oracle

## State

closed

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
- Validator observation for closure run
  `20260924T094258Z-50bf95f61913526682a3550d07fa6b1d96d78935` at fixed
  reviewed head `8e61dc268952aa7f434cd164f7d0569a31113530` (2026-09-24):
  the exact O-000017 command, `cargo test --offline --lib --features
  accept-rfc3339-timestamps client_secret_expiration_`, exited `101` before
  compilation with `no matching package named base64 found` in the offline
  crates.io index. This later environment-specific recurrence does not alter
  W-000034's successful observation at `d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba`
  or this Gap's retained State and Resolution.
- Validator observation for closure run
  `20260924T111724Z-295cb33dbeb3b837300c31b0b1a17517095f2393` at fixed
  reviewed head `bbd8b169c9010aeb7d9f2288b19b670a405b526c` (2026-09-24):
  the exact O-000025 fixture command, `cargo test --offline --lib --quiet --
  test_id_token_verification_key_at_hash_rsa_pss_eddsa`, exited `101` before
  compilation with `no matching package named base64 found` in the offline
  crates.io index. This later environment-specific recurrence does not alter
  W-000035's retained successful observation at
  `48761b267c479b38918e4a139ed5c8fd530f2236` or this Gap's recorded State and
  Resolution.

## Impact

No O-000017 Pass or Fail leg was decided. P-000016 remains `implemented` and
unassured; the blocked command is not evidence that the source behavior fails.
The existing P-000014 three-state assurance and the root verification route are
outside this Gap's scope.

## Resolution

closed — W-000034 records the exact predeclared O-000017 command succeeding
offline at `d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba`: all ten selected
fixtures pass, so the previously missing `base64` index entry did not block
dependency resolution or execution on that observation. This resolves the
execution-environment absence only; W-000034 retains the separate behavior
judgment.

## References

- situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md
- situation/oracles/O-000017-judge-client-secret-expiration-sentinel-collisions.md
- situation/witnesses/P-000016/W-000019-client-secret-expiration-sentinel-collision-oracle-blocked.md
- situation/witnesses/P-000016/W-000034-client-secret-expiration-sentinel-collisions-pass.md
