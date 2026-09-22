# G-000005 — No crate-provided discovery cache is identified

## State

accepted

## Gap

No crate-provided discovery-cache behavior is identified in the admitted source
tree. The earlier P-000001 wording included discovery "with caching", but the
repository has no source occurrence of `cache` or `Cache` and no retained test
or witness for cache behavior.

## Relevance

P-000001 describes the imported OIDC relying-party surface. Its scope must not
claim caching when the opening tree only establishes discovery parsing and
client construction; a consumer needing caching must provide it outside this
promise unless a later behavior change records it.

## Evidence

- Exact search of `src/` and `tests/` at the opening checkpoint
  `92946acf13d02a67caab38cb64444a902217fae4` found no `cache` or `Cache`
  occurrence.
- `src/discovery/mod.rs` defines provider-metadata discovery and parsing, and
  `src/discovery/tests.rs` covers deserialization, not storage or reuse of
  discovered metadata.
- P-000001's prior Promise text named discovery "with caching" despite this
  absence.
- Bookkeeping observation (PR #3 branch `bank3/bookkeeping`, 2026-09-22): the
  current P-000001 Promise and Scope carry no caching claim and Scope names a
  crate-provided discovery cache as excluded; case-insensitive
  `git grep -il cache` over `src/` and `tests/` returns no file at the donor
  boundary `92946acf13d02a67caab38cb64444a902217fae4` or at the current tree.

## Impact

A consumer cannot rely on this crate to persist or reuse a discovery document.
This is an unassigned product capability question, not evidence that discovery
parsing itself fails.

## Resolution

accepted — situation/decisions/D-000012-no-crate-provided-discovery-cache-in-the-4-1-lineage.md
records the upstream-parity-by-design choice: the 4.1 lineage carries no
crate-provided discovery cache, consumers own caching at the pluggable
HTTP-client boundary, and adding one would be a new feature outside the
continuity contract. The absence is a documented limitation of the published
contract, and P-000001's Promise, Scope, and Residual no longer claim
caching.

## References

- `situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`
- `src/discovery/mod.rs`
- `src/discovery/tests.rs`
- situation/decisions/D-000012-no-crate-provided-discovery-cache-in-the-4-1-lineage.md —
  the Decision accepting this gap as a documented limitation.
