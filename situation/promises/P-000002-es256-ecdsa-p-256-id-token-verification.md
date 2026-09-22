# P-000002 — ES256 (ECDSA P-256) ID token verification

## State

hypothesis

## Promise

This crate verifies ID tokens signed with ES256 (ECDSA P-256 with SHA-256).
Elliptic curve JWKs (crv=P-256) are accepted during key selection and their
public keys are used for signature verification via the p256 crate. The
supported-algorithms discovery metadata includes ES256.

## Scope

ES256 (ECDSA P-256 with SHA-256) ID token verification. ES384 is an optional
stretch goal judged by O-000002 leg P5; it is not part of this promise's
assured behavior.

## Oracle

O-000002

## State evidence

None — state is `hypothesis`; no feasibility evidence exists yet.

## References

- situation/gaps/G-000001-no-maintained-rust-oidc-rp-library-supports-es256.md
  — the ecosystem gap this promise closes; the gap closes when this promise
  is assured.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item P-000002
(project Status: Todo).
