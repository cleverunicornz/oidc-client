# oidc-client — situation context

## Current phase

`IMPLEMENTATION` — the imported Rust crate and its test surface are present.
Its behavior records are being reconciled; no behavior is assured unless a
named Promise records a passing Witness.

## Implementation map

This repository contains one standalone Rust OIDC relying-party crate named
`oidc-client`, carrying the openidconnect 4.0.1 baseline. The crate is not yet
published on crates.io. It includes ES256 (ECDSA P-256) verification, assured
under situation/oracles/O-000002-judge-es256-verification.md by witness
situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md;
Poda Chat adoption remains a separate downstream possibility rather than this
repository's implementation state.

## Origin evidence

Initial scoping evidence was gathered in the Poda Chat repository
(cleverunicornz/poda-chat, Banks 1–6, 2026-09-17 through 2026-09-22):

- The `openidconnect` crate (v4.0.1, ramosbugs/openidconnect-rs) is the
  full-featured OIDC RP donor carried here. It was effectively unmaintained by
  July 2025 (zero 2026 commits and no pipeline for a next version).
- The 4.0.1 donor contains EC P-256 and P-384 signature-verification paths and
  parses ECDSA signing-algorithm metadata. The prior assertion that ECDSA was
  dropped in the 2.x→4.x rewrite, and that no Rust OIDC library supports ES256,
  is refuted by the admitted donor. The donor's public enum documentation
  called ECDSA unsupported; the PR #2 assurance lane corrected the enum
  documentation (closing G-000007) and P-000002 is now assured under
  O-000002 by witness W-000003.
- No maintained Rust OIDC RP library was identified for Poda Chat's ES256
  need. The `openid` alternative (kilork/openid, v0.24.0) also does not support
  ES256 (elliptic-curve JWKs are explicitly rejected at client.rs:276).
- Kanidm 1.11.0 (the selected IdP for Poda) defaults to ES256; RS256 is
  labeled "legacy" behind a `warning-enable-legacy-crypto` flag.
- Poda Chat currently uses RS256 under this legacy flag.

## Upstream coordinate

`https://github.com/ramosbugs/openidconnect-rs` — standalone donor material
imported at tag `4.0.1`, commit
`b639b5d39eac6903238867aeb2b29326502e6b26` (crates.io release
2025-07-06T22:08:11Z). The tag is MIT-only. This repository is not a GitHub
fork; it carries the donor forward with attribution.

## Repository ownership

`OWNED` — this repository's operational trunk is its own default branch. The
upstream coordinate is donor provenance, not a fork authority.

## Dependency baseline

The upstream crate is built on `oauth2-rs` (ramosbugs/oauth2-rs, v5.0.0,
51.2M downloads, actively maintained — the underlying protocol library is
healthy). The OIDC layer on top is the abandoned part.

## RUSTSEC context

`deny.toml` records a scoped `RUSTSEC-2023-0071` disposition together with
the two dev-path informational dispositions; D-000003, D-000005, and P-000004
state its boundary. The policy route is assured: P-000005 is assured under
O-000006 by witness
situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md, whose
dispatched run (https://github.com/cleverunicornz/oidc-client/actions/runs/35743515109)
executed `cargo deny check` on the configured fleet runner.

## Closure state

- Current run: none
- Last completed closure: run `20260924T111724Z-295cb33dbeb3b837300c31b0b1a17517095f2393`, opened at `c9e07d9eb28282fd56ffbcce87cc33f4093a3f9b`
- Transcript: `https://github.com/cleverunicornz/infrastructure/actions/runs/35991901418`
