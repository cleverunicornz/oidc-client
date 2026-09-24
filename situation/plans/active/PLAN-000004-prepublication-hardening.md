# PLAN-000004 — Pre-publication hardening

## Candidates

None.

## Promises

- situation/promises/P-000008-signed-user-info-verification-policy.md
- situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md
- situation/promises/P-000010-user-info-flows-bind-verified-id-token-subject.md
- situation/promises/P-000011-malformed-bearer-token-fails-request-preparation.md
- situation/promises/P-000012-content-type-essence-matches-rfc7231-optional-whitespace.md
- situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md
- situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md
- situation/promises/P-000015-fresh-resolution-compiles-at-declared-floor.md
- situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md

## Dependencies

- P-000014 establishes the non-colliding three-state registration-expiry
  representation before P-000016 can assure its epoch-collision rejection
  boundary.

## Completion

Completes when:

- P-000008 is `assured`;
- P-000009 is `assured`;
- P-000010 is `assured`;
- P-000011 is `assured`;
- P-000012 is `assured`;
- P-000013 is `assured`;
- P-000014 is `assured`;
- P-000015 is `assured`; and
- P-000016 is `assured`.
