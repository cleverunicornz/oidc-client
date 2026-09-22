# C-000007 — Re-qualify Poda Chat against standing deployment

## State

proposed

## What

After switching to oidc-client, re-run the Poda Chat qualification rig
against the standing local deployment to prove the swap is
behavior-identical. This is the same matrix used in Banks 4-6 (the
G-000006 route reference regenerates from the Poda Chat repository).

## Approach

1. Re-run the deployed-shape matrix (D-0000092 or D-0000111 equivalent)
2. Verify: login journeys, pending flows, redemption, freshness, key
   rotation all pass
3. If ES256 is now enabled on Kanidm (dropping the legacy-crypto flag),
   verify ES256 tokens
4. Record witnesses in Poda Chat situation/

## Dependencies

C-000006 (the swap must be in place first)

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item C-000007
(project Status: Todo).
