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

- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
  authorizes the maintenance deltas carried by P-000008, P-000009, and
  P-000010.
- situation/decisions/D-000014-ec-key-type-doc-correction-and-scoped-import-fidelity-supersession.md
  authorizes the JWK-surface delta carried by P-000013.
- situation/decisions/D-000015-prepublication-hardening-maintenance-deltas-http-header-surface.md
  authorizes the HTTP-header-surface deltas carried by P-000011 and
  P-000012.
- situation/decisions/D-000016-msrv-baseline-is-compilation-proven.md
  establishes P-000015, whose scope is witnessed by
  situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md.
- situation/decisions/D-000017-prepublication-hardening-maintenance-delta-registration-expiry-surface.md
  authorizes the registration-expiry delta carried by P-000014.
- situation/decisions/D-000018-reject-client-secret-expiration-never-expires-sentinel-collisions.md
  selects the complementary epoch-collision behavior carried by P-000016.
- W-000011 through W-000017 retain the parent-gate observations for P-000008
  through P-000014; W-000018 retains P-000015's fresh-resolution proof; and
  W-000019 records the blocked O-000017 attempt for P-000016.

## Completion

Completes when every listed Promise has a state justified by its named Oracle
and retained Witnesses; a Promise is `assured` only when that evidence covers
its complete declared Scope, and remaining limits stay explicit in its
Residual or a linked Gap. A current-branch successful P-000005 configured-gate
run URL must be retained in its own witness before any current gate claim cites
it.
