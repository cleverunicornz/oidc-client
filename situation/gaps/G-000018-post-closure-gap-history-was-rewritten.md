# G-000018 — Post-closure Gap history was rewritten

## State

open

## Gap

Several Gap records that were present at the prior closing checkpoint replace
pre-existing text outside State and Resolution in the current substantive
interval. The Gap contract permits later State/Resolution disposition and
additive attributed observations, but says earlier statements and observations
remain unchanged.

## Relevance

The affected files are changed records in closure run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`. Preserving their
historical observations is part of the bounded validator review; this concern
does not dispute the new evidence or the authority to update State and
Resolution.

## Evidence

Validator observation for run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52` at fixed reviewed
head `4a05b4511348644c65352a5a0373c11370bd1fd3`:

- `ac85f14355ff197d07b8a85b2dbcfb13155743db` is the prior completed
  closure checkpoint and contains each affected Gap's earlier bytes.
- `situation/gaps/G-000001-no-maintained-rust-oidc-rp-library-supports-es256.md`
  replaces the earlier Gap statement that the continuation was "not yet
  published or assured" with a current assurance statement. Its appended
  provenance then says the correction occurred "before any closing
  checkpoint" and that historical observations are unchanged.
- `situation/gaps/G-000007-es256-public-docs-contradict-implementation.md`
  replaces the earlier Impact phrase "implemented-but-unassured" with
  "implemented and assured" and alters an existing reference annotation. Its
  appended provenance likewise says historical observations are unchanged.
- `situation/gaps/G-000003-invalid-request-object-serialization-typo.md`,
  `situation/gaps/G-000004-no-assured-ci-witness-route.md`, and
  `situation/gaps/G-000006-no-full-es256-id-token-fixture.md` alter existing
  References text in addition to their permitted State/Resolution updates and
  newly added references.
- `situation/gaps/AGENTS.md` under "Additive observations" says earlier Gap
  statements and observations remain unchanged after a closing checkpoint;
  later observations may be appended under Evidence, Impact, or References,
  while separately assigned disposition work may update State and Resolution.

## Impact

The reviewed head no longer retains the exact historical statements that the
prior checkpoint froze, and two provenance notes contradict the observable
checkpoint sequence. The new State/Resolution dispositions may still be
correct; the concern is loss of the earlier record while adding current facts.

## Resolution

none

## References

- ac85f14355ff197d07b8a85b2dbcfb13155743db:situation/gaps/G-000001-no-maintained-rust-oidc-rp-library-supports-es256.md
- ac85f14355ff197d07b8a85b2dbcfb13155743db:situation/gaps/G-000003-invalid-request-object-serialization-typo.md
- ac85f14355ff197d07b8a85b2dbcfb13155743db:situation/gaps/G-000004-no-assured-ci-witness-route.md
- ac85f14355ff197d07b8a85b2dbcfb13155743db:situation/gaps/G-000006-no-full-es256-id-token-fixture.md
- ac85f14355ff197d07b8a85b2dbcfb13155743db:situation/gaps/G-000007-es256-public-docs-contradict-implementation.md
- situation/gaps/AGENTS.md
