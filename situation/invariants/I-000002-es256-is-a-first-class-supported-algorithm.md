# I-000002 — ES256 is a first-class supported algorithm

## Invariant

ES256 is not an optional feature flag or an unstable path. It is a
first-class supported algorithm alongside RS256, with equal test coverage
and documentation. The supported-algorithms list in discovery metadata
includes ES256 by default.

## Basis

The entire motivation for this crate is that ES256 was missing from the
Rust OIDC ecosystem. Making it optional or second-class would defeat the
purpose.

## Priority

standard

## References

- situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md —
  the promise that implements this rule.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item I-000002
(project Status: Todo).
