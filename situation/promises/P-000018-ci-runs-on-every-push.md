# P-000018 — CI runs on every push to a pull request; a newer pull request run supersedes the older one; no other run is cancelled

## State

implementing

## Promise

For the workflow in Scope, on same-repository pull requests:

1. **Light on every push.** A push to a pull request whose changed files are not all `**.md`, `docs/**`, `situation/**` or `LICENSE*` starts a `pull_request` run that executes the job,
   draft or not; so do `opened`, `reopened` and `ready_for_review`.
2. **Supersession.** While a `pull_request` run for a pull request is queued or in progress, a newer `pull_request`
   run of the workflow for the same pull request cancels it; the older run ends `cancelled`, and the newer run reaches
   its own conclusion.
3. **Isolation.** A run for any other event (`workflow_dispatch`) is in a group of its own with `cancel-in-progress`
   false; no other run cancels or replaces it.
4. **Gate.** infra-v2's `scripts/heavy-build-green.sh cleverunicornz/oidc-client <number>` (pinned `0f6aa5cc`) exits 0
   only when the pull request is not a draft and the workflow, where its filters admit the pull request, has a green
   run on the exact head.
5. **The job.** The `ci` job runs the required native-capability checks, then `cargo fmt --all --check`,
   `cargo clippy --all-targets -- -D warnings`, `cargo test --all-features` and `cargo deny check`, on runner group
   `ci`, label `automation-test-s`, for same-repository pull requests and manual dispatch; fork heads never run it.

## Scope

`CI` (`.github/workflows/ci.yml`). This Promise carries locally the organization's model, infra-v2
P-000095: `Private: cleverunicornz/infra-v2@0f6aa5cc#situation/promises/P-000095-build-trigger-model.md`
(https://github.com/cleverunicornz/infra-v2/blob/0f6aa5cc/situation/promises/P-000095-build-trigger-model.md;
organization access; [#498](https://github.com/cleverunicornz/infra-v2/pull/498)). The repository has no heavy `pull_request` workflow.

Out of Scope: whether the job's commands pass; the merge itself (GitHub does not enforce the gate;
G-000035).

## Oracle

[O-000028](situation/oracles/O-000028-ci-runs-on-every-push.md)

## State evidence

- `implementing`: [D-000021](situation/decisions/D-000021-ci-runs-on-every-push.md); observed in [W-000041](situation/witnesses/P-000018/W-000041-ci-runs-on-every-push.md).
- Supersedes [P-000017](situation/promises/P-000017-ci-route-supersedes-pull-request-runs.md) under D-000021.

## Residual

- **Isolation** is judged from the file and GitHub's documented concurrency semantics; no dispatch is run.
- **Drafts** are judged from the trigger (GitHub runs `synchronize` for drafts); the proof pull request is not a draft.
- **Shape enforcement** is by hand (G-000035).
