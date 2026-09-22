# D-000001 — Create oidc-client as a standalone public crate, not a vendored fork

## Status

accepted

## Date

2026-09-22 (materialized)

## Context

The retained research had not identified a maintained, published Rust OIDC
relying-party package for the ES256 need recorded in G-000001. Poda Chat
consumes an ES256-defaulting IdP (Kanidm) and currently speaks RS256 only under
a legacy flag; the upstream openidconnect project is recorded as unmaintained.

## Evidence

- situation/gaps/G-000001-no-maintained-rust-oidc-rp-library-supports-es256.md
  — the ecosystem gap that forced the choice
- situation/references/R-000001-upstream-pin.md — the upstream pin this
  decision carries forward
- Origin scoping evidence gathered in Poda Chat Banks 1–6
  (situation/context.md)

## Decision

Create a new public repository (cleverunicornz/oidc-client) carrying forward
the openidconnect 4.0.1 code with attribution, publish on crates.io as
oidc-client, and consume from Poda Chat via Cargo.toml.

## Why

Public-first axiom. Vendoring creates maintenance complexity inside the Poda
Chat monorepo. A public crate is cleaner to consume, can benefit the Rust
ecosystem, and could attract contributors.

The crate is MIT-licensed (verified at the 4.0.1 pin — R-000001), so carrying
forward with attribution is permitted.

## Rejected alternatives

- Vendoring into Poda Chat: creates workspace complexity, private
  maintenance burden, no ecosystem benefit
- GitHub fork: creates upstream network linkage we do not want
- Waiting for upstream: maintainer dark 14+ months, no pipeline, no open
  issue for ECDSA
- Using kilork/openid: does not support ES256 (verified at v0.24.0)

## Consequences

We own the maintenance of a public crate. Poda Chat can replace its abandoned
upstream dependency only after a published release and its own downstream
qualification; that adoption is not evidence in this repository.

## Revisit when

A maintained public Rust OIDC RP package satisfies the recorded ES256 need, or
the organization changes the intended ownership or distribution boundary.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item D-000001
(project Status: Todo). Corrected against evidence: the project text says
"dual-licensed MIT OR Apache-2.0"; the Phase A upstream-facts scout
verified at pin b639b5d that the tag is MIT-only (R-000001).
The Context section restates G-000001 and the bootstrap
situation/context.md origin evidence.
