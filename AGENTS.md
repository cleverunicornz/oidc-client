# AGENTS.md

<bedrock-protocol>
## Repository knowledge protocol

This repository operates under the Bedrock knowledge protocol. These policies
are repository law: follow them as written; do not readjudicate them during
ordinary work.

Before changing code, behavior, architecture, repository policy, documentation,
or planned work:

1. Read `situation/AGENTS.md`.
2. Read the relevant situation records, related Gaps, and qualifying Candidates.
3. Read the nested `AGENTS.md` governing every situation namespace you will
   modify.
4. Update affected records in the same work as the repository change.
5. Treat `situation/` as canonical repository knowledge. README is human-facing
   orientation; neither README, comments, plans, nor pretrained assumptions
   override situation records.

The record classes are:

- **Promises** state falsifiable behavior and carry lifecycle state.
- **Oracles** define how Promises are judged.
- **Witnesses** retain immutable observations from actual runs.
- **Decisions** preserve why choices were selected or rejected.
- **Invariants** state binding repository rules.
- **Gaps** preserve encountered absences, concerns, and uncertainties.
- **Candidates** record evidence-derived possibilities, not commitments.
- **Plans** group Candidates and Promises into work without restating them.
- **References** retain supporting depth.

Record concerns and uncertainties encountered during the work as Gaps, even
when minor or tentative. Follow `situation/gaps/AGENTS.md` to relate them to
existing Gaps and supporting records. Surfacing a Gap does not assign its
investigation or resolution to this task; continue the assigned
Promise/Oracle/Witness work.

A change affecting claimed behavior repairs missing local Promise/Oracle/Witness
lineage in its own closure; a Gap about unaffected behavior remains an observation.

Git is the run's append-only event log. A run performs one closure on one pull
request branch, bounded by an opening checkpoint commit and a closing checkpoint
commit on that branch. Agents commit and push completed units of work promptly;
corrections are new forward commits. Published history is never amended,
rebased, reset, or force-pushed. Only opening and closing checkpoints define the
run container; interior commit count and shape are not prescribed.

Every trunk change lands through a pull request. Passing branch protection and
satisfying review requirements make a pull request mergeable; they do not
authorize an agent to merge it. An agent opens or updates a pull request and
leaves it open unless the active task explicitly authorizes that agent to merge
that exact pull request.

Run reports — closer summary, validator docket, corrector summary — are pull
request comments, never repository files. Agent transcripts are archived outside
the repository; both checkpoint commits carry the archive URI in a
`Bedrock-Transcript` trailer alongside their other trailers. The checkpoint
commits are the only writers of the closure state in `situation/context.md`.

A failed run is never resumed. The orchestrator retries an invoked agent that
died by restarting that same agent with the same prompt, at most three times,
and never adjudicates or finishes that agent's work itself. A run that still
fails leaves its pull request open and its branch untouched: Bedrock never
opens, closes, merges, or rebranches a pull request under any circumstance.
Re-requesting Bedrock on the same pull request starts a new run; an opening
checkpoint with no closing checkpoint marks a failed closure and is superseded
by the next run's opening checkpoint.

A record is immutable from the first closing checkpoint that follows its
creation or change. Until then, on the open pull request, it may be corrected
in place by a forward commit.
Gaps permit append-only observations and separately assigned State/Resolution
updates under `situation/gaps/AGENTS.md`; earlier observations remain unchanged.

Every assured Promise is invariant behavior. Changing it requires a superseding
Promise, a Decision explaining the change, a replacement Oracle, and new
Witnesses.

Candidates are possible responses derived from evidence. A Candidate becomes
behavior only through a Decision that promotes it into a falsifiable Promise
with an Oracle. Plans qualify Candidates and implement or assure Promises;
recording a Gap does not assign that subsequent work.
Detailed requirements may be adopted by reference from a pinned specification;
changing it follows the assured-promise path. A promoted Promise is the selected
commitment stating the accepted behavior; implementation never redefines it.

The learning loop is:

```text
Promise -> implementation -> Oracle -> Witness -> disposition
        -> Gap -> Candidates -> Plan -> Decision
        -> promoted Promise + Oracle -> implementation
```

Repository files are referenced by repository-root-relative path. Historical
repository bytes use `<commit>:<path>`. External public files use full URLs.
External private files use declared `Private: owner/repo@<ref>#<path>`
coordinates; inability to fetch a declared-private reference is expected and
never grounds to stop, remove it, or invent its contents.

When repository orientation identifies an upstream fork, upstream
synchronization and contribution follow the organization's fork rules in the
root organization block. Bedrock records ownership and the upstream coordinate
and performs neither.

Root `AGENTS.md` carries three tagged blocks in this order: the protocol block
`bedrock-protocol`, the organization block `bedrock-organization`, and the
repository block `bedrock-repository`. The organization block is synchronized
by the closure automation and is optional; adopters whose automation supplies
none carry the other two blocks.

This protocol block is protocol-owned; the organization block is
organization-owned. Agents must not edit any byte inside either. Agents edit only the repository block, which holds all repository-specific
orientation in the shape given by the repository block template published with
the protocol release and reproduced in the closure automation.
</bedrock-protocol>

<bedrock-organization>
## Clever Unicorn operating axioms

These axioms apply to every managed repository. This block is
organization-owned and synchronized by Bedrock; agents do not edit it.
Repository-specific orientation belongs in the repository block that follows.

### How we work

- Judge what code can do under the Promise's supported conditions, not what its
  author intended. Establish its declared behavior and relevant failure
  behavior at that boundary; record the meaningful limits of the evidence.
- A gap has three suspects: the code, the requirement, or the instrument.
  Interrogate in the open before displacing any of them.
- Situate before acting; re-situate after. Interrogate a contradiction before
  displacing what it contradicts.
- Completion is behavior at the promised boundary. Nothing delivered is a
  stub, placeholder, or deferred branch.
- Make it first, prove it after: build the slice, prove it, fix, continue.
- Verify by regenerating from source, never by reading the claim. A gate claim
  cites a CI run URL, never a local attestation.
- Predeclare criteria before the run that answers them and judge only against
  them. A refuted hypothesis is a successful experiment; mixed outcomes stay
  mixed.
- The dependency boundary is the assurance boundary. Pinned versions are
  assured versions and bumps are deliberate acts. A missing capability at a
  consumed boundary is blocking: stop and escalate rather than work around it.
  Distinguish missing capability from limitations of evidence or the execution
  environment. New product responsibilities need a behavioral justification
  and a Decision; uncertainty in a Witness supplies neither.
- Public-first: before building inside, ask why it cannot be a public crate or
  repository.
- Decisions are append-only: supersede, never edit.
- Orchestration-only actors own scheduling and administrative reporting;
  specialists own substantive work and validation. A returned completion advances
  the assignment, with reporting defects soft-corrected from known facts. An
  interrupted invocation without a completed return gets a fresh invocation of
  the same role and original assignment within its retry bound; the replacement
  worker owns the existing work and its interpretation. PR workflows retain the
  assigned PR and branch throughout. Returned meaning establishes role completion;
  publication evidence and machine receipts serve administrative bookkeeping.

### Git and workflows

- Force push does not exist. Nothing pushed is deleted. One writer per ref.
  Every trunk change lands through a pull request.
- Passing branch protection and satisfying review requirements make a pull
  request mergeable; they do not authorize an agent to merge it. An agent
  opens or updates a pull request and leaves it open unless the active task
  explicitly authorizes that agent to merge that exact pull request.
- An owned repository uses `main` as its default working trunk.
- Pull requests are orchestrated. Workflows trigger on `pull_request` with
  `types: [opened, reopened, ready_for_review]`, on explicit dispatch, or on a
  Bedrock request. Branches carry no `push` trigger; `push` to main exists only
  for release and deployment witnesses. CI runs once when a pull request opens
  and once on its final head by dispatch before merge.
- All agent-driven build and test work runs on Linux through the five logical
  runner labels documented by the select-runner plugin skill; no other platform
  or label is valid for agents. Missing runner capabilities are requested by
  issue to the infrastructure repository, never by modifying runners. Fork
  pull requests never reach the fleet. A missing host tool is a P0 defect,
  never a hidden substitute. CI runs the real suite.
- One fixed toolchain per repository with canonical task names.
- A pull request that carried a Bedrock closure merges with a merge commit,
  never a squash or rebase, so its checkpoint commits stay reachable from the
  trunk.

- When a pull request into an active repository's working trunk is ready for
  review, agents must request Bedrock before final handoff: post exactly
  `@unicornz-integrity bedrock requested` as the whole PR comment. Agents have
  authority to make this request. Monitor it until closure completes and
  Integrity's review is published; follow an existing active run rather than
  duplicate it. Later substantive changes require a new closure. A failed or
  blocked closure is reported as incomplete, not ready for handoff. The
  repositories that source Bedrock itself — the infrastructure repository and
  the protocol source repository — are the sole exemptions; the controller
  refuses requests there, so do not make one. Closure completion does not
  authorize merging the PR.
- Internal reach uses the organization's private network; exceptional access
  follows the recorded incident procedure.

### Forks

- A fork of an upstream repository keeps `main` as its upstream trunk and
  works on `internal/main`, its default working trunk. Every work branch is
  `internal/<name>` or `upstream/<name>`; the organization rulesets admit no
  other name.
- Both `main` and `internal/main` accept updates only through pull requests.
- Advancing `main`, including an upstream sync, requires approval from a human
  fork maintainer. This review gate is independent of authority to execute the
  merge.
- `internal/main` has no standing human-review requirement. Bedrock runs there
  and nowhere else.
- Contributions travel outward only: cherry-pick from `internal/main` onto an
  `upstream/<name>` branch cut from `main`, open the pull request into `main`,
  and from `main` open the pull request to the parent. Bedrock-owned files are
  never cherry-picked; keep record changes in separate commits from code so a
  cherry-pick stays clean.
- During a Bedrock closure, upstream-owned files are not edited, removed, or
  rewritten merely to impose fork orientation; that orientation lives only in
  the root blocks and `situation/`. This does not restrict ordinary product
  work through `internal/main`.

### Tools and knowledge

<!-- cvu-native-capabilities:start -->
- Use Qualia semantic search as the default way to find code; it returns a few
  relevant, located results instead of flooding context with raw matches.
  Use exact-string search or a direct read only for a name, literal, or error
  you already know exactly. Semantic coverage excludes ignored and unsupported
  content; use normal read/search tools for those, and do not hunt for or
  install replacements when a capability is absent.
- Native LSP integration is disabled by user decision; there is no lsp tool,
  automatic formatting, or diagnostic feedback. Ordinary repository compiler
  and test checks establish correctness. Ordinary internal tasks isolate by
  default on the supported copy-on-write substrate, and child changes reach
  the parent through report merge; explicit shared-checkout workflows such as
  Bedrock retain their declared exception.
<!-- cvu-native-capabilities:end -->

- Tool-specific skills live at the harness user level and install with their
  plugin. Managed repositories carry no skills directory. A repository procedure
  is a Reference owned by the Invariant that requires it or the Promise it
  satisfies. Fleet-wide procedures are named plugin skills invoked by exact
  name and never restated.
- Keep scratch work outside the repository in the environment's designated
  temporary location; remove it when the task completes.
- Bedrock run evidence is retained outside the repository. Each closure's
  opening and closing checkpoints and its closure state carry that run's
  evidence reference; storage and retrieval procedures belong to
  infrastructure operations.
</bedrock-organization>

<bedrock-repository>
## oidc-client

- Identity: A standalone Rust OIDC relying-party client library carrying an attributed openidconnect 4.0.1 baseline and an implemented, unassured ES256 verification path.
- Ownership: `OWNED`.
- Phase and implementation map: `situation/context.md`.
- Critical invariants: none.
- Verification: Unassured — no assured witness route is presently recorded for repository gate claims; [G-000004](situation/gaps/G-000004-no-assured-ci-witness-route.md) retains the absence and [C-000004](situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md) remains qualifying.
- Tool priority: organization defaults.
- Donor boundary: `92946acf13d02a67caab38cb64444a902217fae4` (the BACKPORT opening checkpoint at which the trigger tree became historical donor material).
</bedrock-repository>
