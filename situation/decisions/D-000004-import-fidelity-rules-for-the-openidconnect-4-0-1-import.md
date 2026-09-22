# D-000004 — Import fidelity rules for the openidconnect 4.0.1 import

## Status

accepted

## Date

2026-09-22

## Context

The faithful import (C-000001) must distinguish deliberate continuity
deltas from upstream bytes so any future upstream re-sync is a mechanical,
auditable operation. Every deviation from upstream bytes at the pin must
fall into a classified category, and everything not classified as a delta
must be byte-identical to upstream. The gate toolchain (rust 1.98.0) is
newer than the upstream clippy gate (1.65.0), so inherited code may hit
lint drift that needs a predeclared remediation policy.

## Evidence

- situation/references/R-000001-upstream-pin.md — pin commit
  b639b5d39eac6903238867aeb2b29326502e6b26 (tag 4.0.1) and the 41-file
  import-surface facts (src 30, tests 3, examples 3, plus `Cargo.toml`,
  `Cargo-1.65.lock`, `LICENSE`, `README.md`, `UPGRADE.md`; excludes
  `.github/`, `.gitignore`, `.codecov.yml`).
- situation/candidates/C-000001-import-openidconnect-v4-0-1-code-with-attribution.md
  — the import candidate whose approach these rules bind.
- situation/invariants/I-000001-upstream-attribution-is-preserved.md — the
  attribution invariant this decision implements.
- Upstream rename surface verified at the pin: crate-name and repository
  metadata in `Cargo.toml`; 16 real `use` lines in `tests/`
  (`rp_certification_code.rs` 10, 16, 17; `rp_common.rs` 4, 8, 100) and
  `examples/` (`gitlab.rs` 19, 23, 24, 25; `google.rs` 17, 23, 24;
  `okta_device_grant.rs` 18, 24, 25); doc-comment import paths in `src/`
  (`lib.rs` 15 lines including the `#importing-openidconnect-...` anchor
  at line 14, `client.rs` 36–37, `jwt/mod.rs` 62).

## Decision

1. Carried verbatim from the pin: `src/` (30 files), `tests/` (3 files),
   `examples/` (3 files), `Cargo.toml` (except the metadata deltas below),
   `Cargo-1.65.lock` (as-is; an inert reference lock — cargo does not read
   that filename), `LICENSE` (byte-identical), `UPGRADE.md` (verbatim),
   and the upstream `README.md` preserved byte-identical as
   `README.upstream.md`.
2. Metadata deltas (root `Cargo.toml` only): `name = "oidc-client"`,
   `version = "4.1.0"`
   (situation/decisions/D-000002-version-oidc-client-4-1-0-continuing-upstream-lineage.md),
   `repository` URL → `https://github.com/cleverunicornz/oidc-client`.
   Every other field stays verbatim: authors, description, keywords,
   `license = "MIT"`, edition 2021, rust-version 1.65, `readme`, badges,
   docs.rs metadata, all 10 features, dependency specifications. No
   `homepage`/`links`/`documentation` fields are added (none exist
   upstream).
3. Rename deltas (mechanical, compilation-required): the 16 `use` lines
   listed above become `oidc_client::`; the doc-comment import paths
   become `oidc_client::`, including the lib.rs table-of-contents anchor,
   which is renamed in lockstep with its heading so the intra-document
   link keeps resolving.
4. Preserved verbatim by policy (never deltas, never renamed):
   rp.certification.openid.net/openidconnect-rs URL fixtures
   (`src/discovery/tests.rs`, `src/logout.rs`), the lib.rs examples URL
   (line 124), the `src/id_token/tests.rs` issue URL (line 468),
   `RP_NAME = "openidconnect-rs"` (`tests/rp_common.rs` line 19),
   `admin@openidconnect.local` fixtures (`src/registration/tests.rs` lines
   29, 101, 328, 431), upstream prose comments that mention the upstream
   name (`examples/google.rs` line 48), and the `Cargo-1.65.lock` root
   entry. These are historical test data and upstream narrative; renaming
   them would falsify provenance. The 21 live-network certification tests
   stay `#[ignore]`d with fixtures verbatim.
5. Attribution: authored `README.md` and `NOTICE.md` credit David Ramos
   (ramosbugs) and contributors with the MIT notice; `LICENSE` stays
   byte-identical (situation/invariants/I-000001-upstream-attribution-is-preserved.md).
6. Clippy remediation policy at the pinned rust:1.98.0 gate: attempt
   `cargo clippy --all-targets -- -D warnings` on the imported code; lint
   drift against inherited code is remediated behavior-neutrally —
   minimal mechanical fixes itemized per lint in dedicated commits; where
   mechanical fixing is semantic-risky, a targeted `#[allow]` with a
   justification comment. Behavior never changes. Every deviation from
   upstream bytes is classified as exactly one of: rename / metadata /
   mechanical-lint / allow-addition; the exhaustive table lands in
   witness situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md.
7. Authored files (new, not upstream deltas): `README.md`, `NOTICE.md`,
   `.github/workflows/ci.yml`, `deny.toml`.

## Why

A continuation survives on re-syncability: if every delta is classified,
the next upstream sync is a mechanical rebase against this table. Fixture
URLs and historical data are the upstream project's certification history.
The clippy policy keeps lint conformance from smuggling behavior changes
into a "faithful" import.

## Rejected alternatives

- A GitHub fork of the upstream repository: rejected at
  situation/decisions/D-000001-create-oidc-client-as-a-standalone-public-crate-not-a-vendored-fork.md.
- Wholesale reformatting or API modernization of inherited code: violates
  faithfulness and produces unreviewable deltas.
- Renaming certification URL fixtures to match the new crate name:
  falsifies the recorded provenance of the upstream test data.
- Dropping `Cargo-1.65.lock`: loses the MSRV reference lock the upstream
  shipped for its 1.65.0 gate.

## Consequences

Any future diff against pin b639b5d39eac6903238867aeb2b29326502e6b26 must
classify into rename / metadata / mechanical-lint / allow-addition or be a
defect. `README.upstream.md` must stay byte-identical to the upstream
`README.md` at the pin. The remediation path actually taken at the 1.98.0
gate is appended to this record's Consequences once the gate run
completes.

## Revisit when

The next upstream sync (re-derive the delta table against the new pin), or
if a gate toolchain change forces remediation outside the predeclared
policy.

## Provenance

Recorded 2026-09-22 by the Phase B import lane, ahead of the import work
it governs; rename-surface facts verified at the pin the same day.
