# G-000010 — Candidate, Decision, and Promise boundaries conflict

## State

open

## Gap

Several reconciled Candidates remain non-commitments even though linked current
records present the same behavior as selected or promised, while the promoted
import Candidate's selecting Decision does not link the Promise and Oracle that
the Candidate says it created. The resulting learning-loop paths are not
internally consistent.

## Relevance

The BACKPORT reconciliation changed these Candidates and their related
Promises, Decisions, Oracles, and Plan. Candidate state is the boundary between
a possibility and accepted behavior; current records must be traversable
without treating the same behavior as both unselected and committed.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`.
- C-000005 remains `proposed` and says it has not been promoted, while accepted
  D-000001 explicitly chooses publication and P-000003/O-000003 already state
  and judge that publication behavior.
- C-000004 remains `qualifying` and calls the current CI route evidence rather
  than a promotion, while P-000003/O-000003 already include operation of that
  route in their contract.
- C-000003 remains `qualifying`, while D-000003 calls it "the candidate this
  decision resolves" and D-000003/D-000005 select the exact current advisory
  dispositions embodied in `deny.toml`.
- C-000001 says D-000004 promoted it into P-000001 and historical O-000001, but
  D-000004 links C-000001 without linking P-000001 or O-000001 as the selecting
  Decision contract requires.
- D-000001's Decision also says Poda Chat will consume the crate, while
  C-000006 retains that downstream action as a merely proposed possibility;
  its Consequences section uses the narrower, non-committal wording that Poda
  Chat "can" replace the dependency after its own qualification.

## Impact

A reader cannot determine which publication, CI, dependency-policy, and
consumer behaviors have actually crossed the Decision boundary. Promotion
lineage and active Plan completion can therefore produce conflicting answers
about what is committed and what remains only a Candidate.

## Resolution

none

## References

- `situation/candidates/C-000001-import-openidconnect-v4-0-1-code-with-attribution.md`
- `situation/candidates/C-000003-resolve-rustsec-2023-0071-rsa-marvin-disposition.md`
- `situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md`
- `situation/candidates/C-000005-publish-to-crates-io.md`
- `situation/candidates/C-000006-switch-poda-chat-to-consume-oidc-client.md`
- `situation/decisions/D-000001-create-oidc-client-as-a-standalone-public-crate-not-a-vendored-fork.md`
- `situation/decisions/D-000003-rustsec-2023-0071-rsa-disposition-scoped-ignore.md`
- `situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`
- `situation/promises/P-000003-published-on-crates-io-as-oidc-client.md`
