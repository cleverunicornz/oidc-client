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

## Dependencies

- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
  authorizes the maintenance deltas carried by P-000008, P-000009, and
  P-000010.
- situation/decisions/D-000014-ec-key-type-doc-correction-and-scoped-import-fidelity-supersession.md
  authorizes the JWK-surface delta carried by P-000013.
- situation/decisions/D-000015-prepublication-hardening-maintenance-deltas-http-header-surface.md
  authorizes the HTTP-header-surface deltas carried by P-000011 and
  P-000012.
- situation/decisions/D-000016-msrv-baseline-is-compilation-proven.md
  promotes P-000015, assured by its stream-recorded witness
  situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md.
- situation/decisions/D-000017-prepublication-hardening-maintenance-delta-registration-expiry-surface.md
  authorizes the registration-expiry delta carried by P-000014.
- P-000008 through P-000014 are ordered after the parent's 2026-09-23 gate
  run at `b96b920f52e0d8b392edcf0b5d752fa570c2b356`, whose witnesses
  W-000011–W-000017 are attached; the disposition of each promise remains
  the parent's.

## Completion

Completes when P-000008, P-000009, P-000010, P-000011, P-000012, P-000013,
P-000014, and P-000015 each reach the state their named oracle's verdict on
the attached witnesses supports (expected `assured`, with the recorded
residuals), and when the pull-request CI gate run URL for branch
`fix/prepublication-hardening` is recorded as evidence under
situation/promises/P-000005-configured-ci-gate-route.md at the time the
parent opens the pull request — that recording is the remaining step and is
not yet done.
