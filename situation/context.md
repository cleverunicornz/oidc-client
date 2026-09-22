# oidc-client — situation context

## Current phase

`bootstrap` — repository created, project scoped, no code yet.

## Implementation map

This repository produces one deliverable: a public, maintained Rust crate
named `oidc-client` on crates.io, serving as a standalone continuation of the
abandoned `openidconnect` crate with ES256 verification. Poda Chat consumes
it as a normal Cargo dependency.

## Origin evidence

All scoping evidence was gathered in the Poda Chat repository
(cleverunicornz/poda-chat, Banks 1–6, 2026-09-17 through 2026-09-22):

- The `openidconnect` crate (v4.0.1, ramosbugs/openidconnect-rs) is the
  only full-featured OIDC RP library in Rust (13.4M downloads, 4M/quarter)
  but has been effectively unmaintained since July 2025 (zero 2026 commits,
  no pipeline for the next version).
- ECDSA verification was implemented in 2020 (issue #32, v1.1.0) then
  dropped in the 2.x→4.x rewrite; no open issue requests its return.
- The only alternative (`openid` / kilork/openid, v0.24.0) also does not
  support ES256 (elliptic curve JWKs explicitly rejected at client.rs:276).
- No Rust OIDC library supports ES256. The `jsonwebtoken` crate (12M+
  downloads) DOES support ES256 but is not a full OIDC RP library.
- Kanidm 1.11.0 (the selected IdP for Poda) defaults to ES256; RS256 is
  labeled "legacy" behind a `warning-enable-legacy-crypto` flag.
- Poda Chat currently uses RS256 under this legacy flag because it is the
  only algorithm both sides speak.

## Upstream coordinate

`https://github.com/ramosbugs/openidconnect-rs` — code taken at v4.0.1
(released 2026-07-06). License: MIT OR Apache-2.0. This is NOT a GitHub
fork; it is a standalone repository carrying forward the code with
attribution.

## Dependency baseline

The upstream crate is built on `oauth2-rs` (ramosbugs/oauth2-rs, v5.0.0,
51.2M downloads, actively maintained — the underlying protocol library is
healthy). The OIDC layer on top is the abandoned part.

## RUSTSEC context

`deny.toml` in Poda Chat carries `RUSTSEC-2023-0071` (rsa Marvin timing
side-channel) ignored because openidconnect depends on the unpatched `rsa`
crate. This crate should resolve or properly scope that disposition.
