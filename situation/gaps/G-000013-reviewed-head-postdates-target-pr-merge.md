# G-000013 — Reviewed head postdates the target pull-request merge

## State

open

## Gap

Pull request #1 was merged while this Bedrock closure was still open, at a head
that precedes the assigned reviewed head. The reviewed reconciliation commits
therefore exist on `bank1-integration` but are not part of the pull request that
this run names as its admission boundary.

## Relevance

Run `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` is assigned
to pull request #1 and the `bank1-integration` branch. The Bedrock protocol uses
one pull request branch as the closure container and the organization requires
every trunk change to land through a pull request.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c`.
- https://github.com/cleverunicornz/oidc-client/pull/1 records a merge at
  2026-09-22T11:09:05Z with pull-request head
  `53162acbd3ae67ea8b4f511b9634482ccecb9b72`.
- Assigned reviewed head `5a8580f3394f853c995e7ccb1148fe86c040321d`
  was committed at 2026-09-22T11:27:17Z and is reachable from
  `origin/bank1-integration`, not from the merged pull-request head.
- The range
  `53162acbd3ae67ea8b4f511b9634482ccecb9b72..5a8580f3394f853c995e7ccb1148fe86c040321d`
  contains 35 closure commits, including most of the final record and README
  reconciliation.

## Impact

A closing checkpoint cannot truthfully make those post-merge commits part of
pull request #1. Treating this run as a completed closure on that pull request
would conflate the branch's later history with the bytes actually admitted and
merged through the named review boundary.

## Resolution

none

## References

- https://github.com/cleverunicornz/oidc-client/pull/1
- https://github.com/cleverunicornz/infrastructure/actions/runs/35718383340
