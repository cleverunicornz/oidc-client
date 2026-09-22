# G-000021 — CI test-count comment is stale

## State

closed

## Gap

The CI workflow's non-normative test-count comment says "70 offline unit + doc
tests", while retained runs execute a different unit/doc-test count. The
command itself is unchanged; the concern is the stale annotation.

## Relevance

The observation was first appended to G-000004 during the prior validator run,
but it is distinct from that Gap's missing assured CI-witness route. G-000004
is now closed by W-000004 while P-000005's Residual says the count comment
remains stale, so this record preserves the unresolved question at its own
boundary.

## Evidence

Validator observation for run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52` at fixed reviewed
head `4a05b4511348644c65352a5a0373c11370bd1fd3`:

- `.github/workflows/ci.yml` line 43 says "70 offline unit + doc tests; the 21
  live-network certification tests stay #[ignore]d."
- `situation/witnesses/evidence/W-000003/gates-es256-fixture.log` records 72
  passing unit tests, 7 passing doc tests, 2 ignored doc tests, and 21 ignored
  live-network certification tests.
- `situation/promises/P-000005-configured-ci-gate-route.md` names this comment
  as stale in Residual.
- `situation/gaps/G-000004-no-assured-ci-witness-route.md` retains the earlier
  validator observation, but its State and Resolution now close the separate
  missing-workflow-witness concern.
- Bookkeeping observation (PR #3 branch `bank3/bookkeeping`, 2026-09-22): the
  comment was replaced with count-agnostic wording — line 43 now reads
  "# The live-network certification tests stay #[ignore]d." The diff of
  `.github/workflows/ci.yml` against base commit `4b6e0ad` on this branch
  shows that one comment line as the file's only change; every command byte
  is identical.

## Impact

Readers may infer an obsolete suite size from the workflow comment. The
configured `cargo test --all-features` command and the assurance of the CI
route are unaffected.

## Resolution

closed — the annotation no longer states counts, so it cannot drift from
retained run evidence: `.github/workflows/ci.yml` line 43 now reads
"# The live-network certification tests stay #[ignore]d." The configured
`cargo test --all-features` command and every other workflow byte are
unchanged; the file's diff against base commit `4b6e0ad` on branch
`bank3/bookkeeping` is that single comment line. P-000005's Residual
sentence naming this comment as stale predates the fix — that record froze
at closing checkpoint `f78f641` and this Gap is the authoritative
disposition.

## References

- .github/workflows/ci.yml
- situation/promises/P-000005-configured-ci-gate-route.md
- situation/witnesses/evidence/W-000003/gates-es256-fixture.log
- situation/gaps/G-000004-no-assured-ci-witness-route.md
