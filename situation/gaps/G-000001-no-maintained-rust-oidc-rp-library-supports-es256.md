# G-000001 — No maintained Rust OIDC RP library supports ES256

## State

open

## Gap

No Rust OIDC relying-party library supports ES256 (ECDSA P-256 SHA-256) for
ID token verification. openidconnect dropped it in a rewrite; openid
(kilork) rejects EC JWKs at client.rs:276. The jsonwebtoken crate supports
ES256 but is not an OIDC flow library. Kanidm defaults to ES256.

## Relevance

This gap is the reason this repository exists: its core deliverable
(P-000002) closes it. Poda Chat currently uses RS256 under Kanidm's
legacy-crypto flag because it is the only algorithm both sides speak.

## Evidence

Poda Chat Banks 1–6 scoping evidence (2026-09-17 through 2026-09-22),
retained in situation/context.md ("Origin evidence"): openidconnect
implemented ECDSA in 2020 (issue #32, v1.1.0) then dropped it in the
2.x→4.x rewrite with no open issue requesting its return; kilork/openid
v0.24.0 explicitly rejects elliptic curve JWKs; Kanidm 1.11.0 labels RS256
"legacy".

## Impact

Any Rust project consuming an ES256-defaulting IdP must either use RS256
(legacy flag on Kanidm) or hand-roll the verification outside an OIDC
library. This blocks modern crypto adoption in the Rust OIDC ecosystem.

## Resolution

none — this gap closes when P-000002 is assured.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item G-000001
(project Status: Todo).
