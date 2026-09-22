# G-000019 — CI Witness combines multiple runs and heads

## State

closed

## Gap

W-000004 declares one Head and a PASS result but retains observations from
several successful and failed workflow runs at different heads. The Witness
contract says a Witness retains one observation from one real run and observes
one Promise under one Oracle at one head.

## Relevance

W-000004 is the cited evidence for P-000005's newly assured configured-CI-route
claim in closure run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`. The concern is
the evidence record's boundary, not the independently observed success of its
primary run.

## Evidence

Validator observation for run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52` at fixed reviewed
head `4a05b4511348644c65352a5a0373c11370bd1fd3`:

- `situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md` declares
  Result `PASS` and Head
  `7fe8265c8166e16ea4da77b5722a35503fa67665`, with primary workflow run
  35743515109.
- The same Evidence section also retains successful run 35746649645 at
  `5dbfc4b46204ca7c36c39283103e4efbe88e1c56`, successful run 35745613778
  at `38796c0752cc294bd58eafdac17e361d4f88ac7b`, failed dispatch
  35743127287 at `c0d702e`, and failed pull-request-event run 35743042564.
- `situation/promises/P-000005-configured-ci-gate-route.md` says those
  corroborating runs are retained in the same Witness.
- The primary run is independently retrievable at
  https://github.com/cleverunicornz/oidc-client/actions/runs/35743515109 and
  reports `workflow_dispatch`, head
  `7fe8265c8166e16ea4da77b5722a35503fa67665`, conclusion `success`, with
  successful capability, formatting, Clippy, test, and dependency-policy
  steps.
- W-000004 also says the primary head `7fe8265` corroborates corrected
  W-000003 by executing
  `verification::tests::test_es256_id_token_verified_claims`. The test name
  existed there, but commit
  `00d45a7c06f3704915550ad96610a9f9d5e68279` later changed that fixture
  from the crate-internal `verified_claims` method to the public
  `CoreIdToken::claims` entry. The primary run therefore predates the
  public-entry behavior W-000003 currently witnesses; run 35745613778 at
  `38796c0752cc294bd58eafdac17e361d4f88ac7b` is the first listed fleet
  execution after that correction.
- `situation/witnesses/AGENTS.md` requires one observation from one real run,
  says a Witness observes one Promise under one Oracle at one head, and
  requires failed Witnesses to be kept.
- Bookkeeping observation (PR #3 branch `bank3/bookkeeping`, 2026-09-22): the
  restructuring was verified on this branch — W-000004 declares Result PASS,
  Head `7fe8265c8166e16ea4da77b5722a35503fa67665`, and run 35743515109 only,
  with an explicit statement that it is bounded to that run and does not
  corroborate W-000003; the corroborating and failed observations each have
  their own Witness record (W-000007 through W-000010) with one Head, one
  Result, and evidenced Oracle legs apiece.

## Impact

The primary PASS observation remains identifiable, but the record's singular
Result and Head cannot describe every retained run. Readers cannot apply the
Witness lifecycle rules independently to the corroborating PASS observations
or the failed observations while they remain embedded in W-000004.

## Resolution

closed — W-000004 was restructured to a single observation: one Head
(`7fe8265c8166e16ea4da77b5722a35503fa67665`), one Result (PASS), and one run
(35743515109), with an explicit boundary note that its head predates the
public-entry fixture correction and therefore does not corroborate W-000003.
The corroborating and failed observations moved to their own Witness records:
W-000007 (run 35745613778 at `38796c0752cc294bd58eafdac17e361d4f88ac7b`,
PASS), W-000008 (run 35746649645 at
`5dbfc4b46204ca7c36c39283103e4efbe88e1c56`, PASS), W-000009 (run 35743127287
at `c0d702eb0968f174871f5932ad7d022c3d9ab4be`, FAIL), and W-000010 (run
35743042564 at `f0ee1c29fa32224b24487dc5322a8c6fc2b2535b`, FAIL). P-000005's
State evidence names each record's boundary, and the failed Witnesses are
kept per the Witness contract.

## References

- situation/promises/P-000005-configured-ci-gate-route.md
- situation/oracles/O-000006-judge-configured-ci-gate-route.md
- situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
- situation/witnesses/AGENTS.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md
- situation/witnesses/P-000005/W-000007-configured-ci-public-entry-fixture-run.md
- situation/witnesses/P-000005/W-000008-later-passing-configured-ci-run.md
- situation/witnesses/P-000005/W-000009-configured-ci-dispatch-clippy-failure.md
- situation/witnesses/P-000005/W-000010-configured-ci-pull-request-clippy-failure.md
