# oidc-client — situation context

## Current phase

`IMPLEMENTATION` — the imported Rust crate and its test surface are present.
Its behavior records are being reconciled; no behavior is assured unless a
named Promise records a passing Witness.

## Implementation map

This repository contains one standalone Rust OIDC relying-party crate named
`oidc-client`, carrying the openidconnect 4.0.1 baseline. The crate is not yet
published on crates.io. It includes implemented-but-unassured ES256 (ECDSA
P-256) verification; Poda Chat adoption remains a separate downstream
possibility rather than this repository's implementation state.

## Origin evidence

Initial scoping evidence was gathered in the Poda Chat repository
(cleverunicornz/poda-chat, Banks 1–6, 2026-09-17 through 2026-09-22):

- The `openidconnect` crate (v4.0.1, ramosbugs/openidconnect-rs) is the
  full-featured OIDC RP donor carried here. It was effectively unmaintained by
  July 2025 (zero 2026 commits and no pipeline for a next version).
- The 4.0.1 donor contains EC P-256 and P-384 signature-verification paths and
  parses ECDSA signing-algorithm metadata. The prior assertion that ECDSA was
  dropped in the 2.x→4.x rewrite, and that no Rust OIDC library supports ES256,
  is refuted by the admitted donor. Its public enum documentation still calls
  ECDSA unsupported; P-000002 records the implementation and its unassured
  evidence boundary.
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

`deny.toml` in Poda Chat carries `RUSTSEC-2023-0071` (rsa Marvin timing
side-channel) ignored because openidconnect depends on the unpatched `rsa`
crate. This crate should resolve or properly scope that disposition.

## Closure state

- Current run: `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` (open)
- Last completed closure: none
- Transcript: none
