# P-000001 — Full OIDC RP flows work identically to openidconnect 4.0.1

## State

implemented

## Promise

This crate supports the complete OIDC relying-party surface of openidconnect
4.0.1: discovery (with caching), authorization-code flow with PKCE S256, ID
token verification (signature, issuer, audience, nonce, expiry, auth_time),
UserInfo endpoint, refresh tokens, and dynamic client registration. Behavior
is identical to the upstream for all existing algorithms (RS256/384/512,
PS256/384/512, EdDSA).

## Scope

The OIDC relying-party flows carried over from openidconnect 4.0.1, existing
algorithms only. ES256 verification is P-000002's behavior, not this
promise's.

## Oracle

situation/oracles/O-000001-judge-full-oidc-rp-flow-parity-with-upstream.md

## State evidence

State `implemented` per situation/promises/AGENTS.md — its criterion,
code exists and assurance is not yet complete, is met. Cited cause: the
imported crate exists and builds, the full offline gate suite is green at
witness head 24835e4b44caa8a0baae2ac5b865bbd6bdf355ba (`cargo fmt --all
--check`, `cargo clippy --all-targets -- -D warnings`, `cargo test
--all-features` 70 passed / 0 failed with the 21 live-network
certification tests compiled and `#[ignore]`d, `cargo deny check` —
situation/witnesses/evidence/W-000001/gates-final.log), and oracle leg P6
is PASS with committed evidence
(situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md).
Deliberately not `qualified` — that state covers feasibility-only
evidence, and code existence is established — and not `assured`, because
oracle legs P1–P5 are unexecuted.

## Residual

Oracle legs P1–P5 of
situation/oracles/O-000001-judge-full-oidc-rp-flow-parity-with-upstream.md
are unexecuted residual work — live-flow parity against a real OIDC
provider, pending the ES256 lane's fixture matrix
(situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md):
P1 discovery, P2 authorization-code flow with PKCE S256, P3 ID-token
verification as a live flow, P4 UserInfo endpoint, P5 refresh-token
exchange.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item P-000001
(project Status: Todo).

State transitioned `hypothesis` → `implemented` 2026-09-22 by forward
commit, responding to CodeRabbit round-1 review thread
PRRT_kwDOUlFWIM6kr1ft on PR #1; cause cited in State evidence.
