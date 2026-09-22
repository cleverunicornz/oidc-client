# G-000019 — CI Witness combines multiple runs and heads

## State

open

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

## Impact

The primary PASS observation remains identifiable, but the record's singular
Result and Head cannot describe every retained run. Readers cannot apply the
Witness lifecycle rules independently to the corroborating PASS observations
or the failed observations while they remain embedded in W-000004.

## Resolution

none

## References

- situation/promises/P-000005-configured-ci-gate-route.md
- situation/oracles/O-000006-judge-configured-ci-gate-route.md
- situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
- situation/witnesses/AGENTS.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md
