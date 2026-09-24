# G-000031 — Retained witness logs expose machine-local workspace paths

## State

open

## Gap

Several retained compilation logs include the verification host's absolute
working-tree path in Cargo's package-status output. The logs are public
repository evidence, so whether retaining that environment-specific topology
is acceptable, or whether future evidence should replace it with a stable
repository-root marker while preserving verifiability, remains unresolved.

## Relevance

The concern arose while validating closure run
`20260924T063914Z-cd10a6d6107e401ba795c255cfb2bc4df2b7e8e2` at reviewed head
`731f85ed1be41e8b6a937bea7bcb68b6c60991ec`. It concerns evidence hygiene and
portable references in the newly retained W-000013 and W-000018 logs; it does
not by itself contradict their recorded command exits or compilation results.

## Evidence

Observed 2026-09-24 during the bounded validator review:

- `situation/witnesses/evidence/W-000013/gitlab-example-check-reqwest-blocking.log`
  line 3 includes an absolute machine-local worktree coordinate in Cargo's
  `Checking oidc-client` line.
- The W-000018 evidence set includes the same kind of coordinate in
  `msrv-check-1.96.0-at-f805688.log`,
  `msrv-check-1.96.0-recheck-at-f805688.log`,
  `msrv-check-1.96.0-recheck.log`, `msrv-check-1.98.0-at-f805688.log`,
  `msrv-check-1.98.0.log`, `msrv-check-nightly-informational-at-f805688.log`,
  and `msrv-check-nightly-informational.log`.
- By contrast,
  `situation/witnesses/evidence/W-000019/cargo-test-client-secret-expiration.log`
  uses the stable marker `[repository root]` for the package path.

Corrector addendum (2026-09-24, closure run
`20260924T063914Z-cd10a6d6107e401ba795c255cfb2bc4df2b7e8e2`):

- Initial non-quiet Cargo output for the bounded signed-UserInfo and
  feature-gated registration-expiry observations likewise included an absolute
  verification-worktree coordinate in its package-status line. The corrector
  did not retain those raw lines: W-000020 and W-000026 retain `--quiet`
  command output with digests instead, so their committed evidence contains no
  machine-local workspace path.

## Impact

The retained paths disclose otherwise irrelevant verification-host and
worktree-layout details, and readers cannot resolve them as repository
coordinates. Altering existing evidence would also change its recorded digest,
so any sanitization or future capture policy needs to preserve evidence
provenance rather than silently rewriting the logs.

## Resolution

none

## References

- situation/witnesses/P-000010/W-000013-user-info-subject-binding.md
- situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md
- situation/witnesses/P-000016/W-000019-client-secret-expiration-sentinel-collision-oracle-blocked.md
- situation/witnesses/P-000008/W-000020-signed-user-info-complete-scope-incomplete.md
- situation/witnesses/P-000014/W-000026-client-secret-expiry-complete-scope-incomplete.md
- situation/AGENTS.md (Reference discipline)
