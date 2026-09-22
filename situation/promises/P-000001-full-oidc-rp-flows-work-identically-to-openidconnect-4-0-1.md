# P-000001 — Full OIDC RP flows work identically to openidconnect 4.0.1

## State

hypothesis

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

None — state is `hypothesis`; no feasibility evidence exists yet.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item P-000001
(project Status: Todo).
