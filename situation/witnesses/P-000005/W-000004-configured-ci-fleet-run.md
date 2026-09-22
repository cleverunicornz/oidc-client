# W-000004 — Passing configured fleet run of the `ci` job

## Promise

situation/promises/P-000005-configured-ci-gate-route.md

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## Result

PASS — the configured `ci` job was executed on the configured fleet runner by
explicit dispatch on `bank2/assurance` and every named gate command ran
successfully.

## Head

7fe8265c8166e16ea4da77b5722a35503fa67665

## Observed

2026-09-22

## Evidence

- Retained workflow run (retrievable):
  https://github.com/cleverunicornz/oidc-client/actions/runs/35743515109 —
  event `workflow_dispatch`, ref `bank2/assurance`, head
  `7fe8265c8166e16ea4da77b5722a35503fa67665`, conclusion `success`
  (run database id 35743515109; job id 106799076856, 55s).
- Step conclusions observed on the run: "Required native capabilities"
  (rustc, cargo, cargo fmt, cargo clippy, cargo deny versions) succeeded
  first, then "Formatting" (`cargo fmt --all --check`), "Clippy"
  (`cargo clippy --all-targets -- -D warnings`), "Tests"
  (`cargo test --all-features`), and "Dependency policy" (`cargo deny check`)
  each succeeded.
- The runner label `cvu-test-runner-x64`, the declared routes
  (`pull_request` types [opened, reopened, ready_for_review] and
  `workflow_dispatch`), and the step layout are read from
  `.github/workflows/ci.yml` at the same head.
- This same head re-executes the O-000002 fixture suite
  (`verification::tests::test_es256_id_token_verified_claims` among the 72
  passing unit tests), corroborating W-000003 on fleet infrastructure.
- Corroborating retained run at the final branch head (correction added on
  the open PR #2 after the validator's public-entry finding moved the
  fixture): https://github.com/cleverunicornz/oidc-client/actions/runs/35746649645 —
  event `workflow_dispatch`, ref `bank2/assurance`, head
  `5dbfc4b46204ca7c36c39283103e4efbe88e1c56` (final branch head),
  conclusion `success`, same job layout and steps (capability checks plus
  the four named commands). Between the primary run and this one, a
  dispatch at `38796c0` also succeeded:
  https://github.com/cleverunicornz/oidc-client/actions/runs/35745613778
  (conclusion `success`, 55s); its suite is the first fleet execution of
  the corrected public-entry ES256 fixture.
- An earlier dispatched run of this PR branch
  (https://github.com/cleverunicornz/oidc-client/actions/runs/35743127287)
  failed at Clippy (`clippy::manual_strip` in the new ES256 test) at head
  `c0d702e`; the correction is commit `7fe8265`, re-dispatched and green as
  retained above. A same-branch `pull_request`-event run for PR #2 also
  exercised the route (run 35743042564, same failure at the pre-correction
  head).
- Correction (added on the open PR #2, CodeRabbit thread
  PRRT_kwDOUlFWIM6kzMgq): the `ci` job's `if:` guard admits it only for
  same-repository pull requests
  (`github.event.pull_request.head.repo.full_name == github.repository`) on
  routes [opened, reopened, ready_for_review] plus `workflow_dispatch`; fork
  pull requests never run this job. Every retained PASS run above is a
  manual `workflow_dispatch` run — this witness is manual-dispatch evidence
  for the configured route, not pull-request-event evidence. The one
  retained same-branch `pull_request`-event run (35743042564) failed at the
  pre-correction head, so no passing pull-request-event run is retained.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `.github/workflows/ci.yml` at head `7fe8265` declares only the `pull_request` routes [opened, reopened, ready_for_review] plus `workflow_dispatch`, guards the `ci` job to same-repository pull requests (`github.event.pull_request.head.repo.full_name == github.repository`) or manual dispatch, and runs the job on `cvu-test-runner-x64`; the retained run executed on that runner by manual dispatch. |
| P2 | PASS — the run's "Required native capabilities" step (rustc, cargo, cargo fmt, cargo clippy, cargo deny) succeeded before the job configured the four named gate commands, in declared order. |
| P3 | PASS — retained run 35743515109 (conclusion `success`) executed `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-features`, and `cargo deny check` successfully on the configured route. |
