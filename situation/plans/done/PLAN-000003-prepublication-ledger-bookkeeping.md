# PLAN-000003 — Pre-publication ledger bookkeeping

## Candidates

None.

## Promises

- situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md —
  the documented-limitation boundary for discovery caching is decided here
  and its gap disposition completed under this plan.
- situation/promises/P-000005-configured-ci-gate-route.md — the stale
  test-count annotation inside this promise's declared `ci.yml` surface is
  corrected here.

## Dependencies

- The repository owner ruled on 2026-09-22 that nothing publishes while this
  bookkeeping is open: publication (P-000003 under PLAN-000002) is ordered
  after this plan completes.
- The eight open gaps at base commit `4b6e0ad` — G-000005, G-000017,
  G-000018, G-000019, G-000020, G-000021, G-000022, and G-000023 — close on
  branch `bank3/bookkeeping`. G-000001 and P-000003 close by publication and
  remain with PLAN-000002.
- P-000001's live-flow parity residual remains C-000007's consumer-adoption
  scope and is not touched by this plan.

## Completion

Completes when P-000001 reaches `implemented` and P-000005 reaches `assured`.
