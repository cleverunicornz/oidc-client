# C-000002 — Add ES256 verification via p256 crate

## State

rejected

## Candidate

Add an ES256 (ECDSA P-256 with SHA-256) ID-token verification implementation
to the imported crate through `p256`.

## Origin

Materialized from cleverunicornz Project #20 on 2026-09-22. The candidate was
rechecked against the admitted opening tree, specifically
`src/core/jwk/mod.rs`, `src/core/crypto.rs`, and their tests.

## Why consider it

The retained ecosystem research identified ES256 support as necessary for an
ES256-defaulting OIDC provider, and the candidate initially described the
expected implementation path after the upstream import.

## Qualification questions

- Does the donor already deserialize EC P-256 JWKs and verify ES256
  signatures?
- Does existing retained execution evidence support an implementation state
  without overstating assurance?

Both questions are settled by
`situation/decisions/D-000006-recognize-carried-es256-verification.md`.

## Candidate approaches

- Add a new P-256 JWK-to-verifying-key conversion and ES256 dispatch.
- Reuse the already carried path and record its actual state.

## Disposition

Rejected by
`situation/decisions/D-000006-recognize-carried-es256-verification.md`:
the proposed conversion, dispatch, and verification path already exist in the
donor import. P-000002 remains the retrospective behavior record; this
candidate does not promote it.
