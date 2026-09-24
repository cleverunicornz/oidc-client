# O-000016 — Judge fresh-resolution compiles at the declared MSRV floor

## State

implemented

## Judges

situation/promises/P-000015-fresh-resolution-compiles-at-declared-floor.md

## Inputs

- The root `Cargo.toml`'s declared `rust-version` at the judged head.
- `rustup toolchain list` on the verification host (the installed
  toolchain inventory, stable set included).
- For each stable installed toolchain with release >= the declared
  `rust-version`: the retained output and exit status of
  `rm -f Cargo.lock && cargo +<toolchain> check --offline` and
  `rm -f Cargo.lock && cargo +<toolchain> check --offline
  --all-features`.

## Pass

- P1 — every such toolchain exits 0 on both commands at the judged head.
- P2 — the declared `rust-version` equals the oldest stable toolchain
  installed on the verification host, so the declaration does not exceed
  what local evidence can prove.

## Fail

- F1 — any such toolchain exits non-zero on either command.
- F2 — the declared `rust-version` is higher than the oldest stable
  toolchain installed on the verification host, or no installed stable
  toolchain's release matches the declaration.

## Implementation

The command battery decides P1/F1: run the two commands per verified
toolchain with no `Cargo.lock` present and inspect exit statuses,
retaining full logs under `situation/witnesses/evidence/` (as in
W-000018's `msrv-check-*.log` files).

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Both fresh-resolution commands exit 0 for every installed stable toolchain >= the declared rust-version | `rm -f Cargo.lock && cargo +<v> check --offline [--all-features]` |
| P2 | Declared rust-version equals the oldest installed stable toolchain | manual — compare `Cargo.toml` with `rustup toolchain list` (witness retains the inventory) |
| F1 | Non-zero exit on any toolchain or profile | same commands as P1 |
| F2 | Declaration exceeds installed evidence | manual — same comparison as P2 |
