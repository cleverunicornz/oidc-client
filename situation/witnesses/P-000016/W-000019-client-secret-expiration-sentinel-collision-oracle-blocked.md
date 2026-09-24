# W-000019 — Client-secret expiry sentinel-collision oracle blocked

## Promise

situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md

## Oracle

situation/oracles/O-000017-judge-client-secret-expiration-sentinel-collisions.md

## Result

BLOCKED — the predeclared focused oracle command could not resolve the
repository's `base64` dependency in the host's offline crates.io index, so it
exited before compiling or executing any oracle leg.

## Head

226ef0be1a7da6969d7840b80a426500b6e27df5

## Observed

2026-09-24

## Evidence

- `situation/witnesses/evidence/W-000019/cargo-test-client-secret-expiration.log`
  retains the sanitized command output and exit status (`101`); its SHA-256 is
  `84ea36e4e69988655907b0764aca3cc94ecf98cb35a7e551ace63e03cc5e0fc6` in
  `situation/witnesses/evidence/W-000019/SHA256SUMS`.
- The command is exactly the implemented O-000017 command:
  `cargo test --offline --lib --features accept-rfc3339-timestamps client_secret_expiration_`.
  Cargo reported `no matching package named base64 found` in the offline
  crates.io index before test compilation.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | BLOCKED — dependency resolution failed before the exact-epoch or subsecond tests could compile. |
| P2 | BLOCKED — dependency resolution failed before the fractional-input test could compile. |
| P3 | BLOCKED — dependency resolution failed before the feature-gated RFC 3339 test could compile. |
| P4 | BLOCKED — dependency resolution failed before the `NeverExpires` round-trip test could compile. |
