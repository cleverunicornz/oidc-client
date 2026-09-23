# G-000029 — Hardening records cite stream head SHAs that do not resolve on the branch

## State

open

## Gap

Two pre-publication hardening records cite stream working-head commit SHAs
that do not resolve in this repository's object database, so the
`<commit>:<path>` resolution the reference discipline requires fails for
those citations. The gap is the broken commit citations inside records that
are otherwise immutable; the observations themselves are separately
supported at resolvable heads.

## Relevance

Citations to historical bytes must resolve (`situation/AGENTS.md`, reference
discipline). The concern arose while attaching the 2026-09-23 gate witnesses
(W-000011–W-000017) for the pre-publication hardening on branch
`fix/prepublication-hardening` and checking the resolvability of the heads
those witnesses cite.

## Evidence

Observed 2026-09-23 at head `b96b920f52e0d8b392edcf0b5d752fa570c2b356`
(clean worktree):

- `git cat-file -t cdfe1ea` and `git rev-parse --verify cdfe1ea^{commit}`
  both fail: the object does not exist. The SHA is cited as the jwk
  stream's implementing commit by
  situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md
  (State evidence) and
  situation/decisions/D-000014-ec-key-type-doc-correction-and-scoped-import-fidelity-supersession.md
  (Evidence and Provenance). The branch history carries the jwk stream work
  as commit `5ab6d7b` ("jwk: redact symmetric and private key material from
  CoreJsonWebKey Debug; correct EllipticCurve capability docs"), which
  lands the same records and code.
- `git cat-file -t e0860535f99aa50ad3d45befcea7ed1644864cc0` fails
  likewise. That SHA is the declared Head of
  situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md and the
  proof commit cited by
  situation/decisions/D-000016-msrv-baseline-is-compilation-proven.md. The
  branch history carries the manifest work as commit `609899a` ("manifest:
  declare compilation-proven MSRV 1.96 and retire Cargo-1.65.lock").
- Interpretation, distinct from the observations above: the two SHAs
  appear to be stream working heads that were rewritten when the stream
  commits landed in the current linear history. The retained evidence files
  under `situation/witnesses/evidence/` are unaffected either way, because
  they are content-addressed by `SHA256SUMS` digests rather than by commit
  identifiers.

## Impact

A reader following the citation discipline cannot resolve `cdfe1ea` or
`e0860535` to bytes, which weakens provenance for P-000013's implementing
commit, D-000014's focused-validation head, W-000018's observation head,
and D-000016's proof head. No behavioral claim is known to be invalidated:
every O-000014 leg was re-executed at `b96b920f52e0d8b392edcf0b5d752fa570c2b356`
by the parent's gate run (witnessed by
situation/witnesses/P-000013/W-000016-jwk-debug-secret-redaction.md), and
W-000018's evidence logs are retained with digests.

## Resolution

none — the affected records are immutable outside a forward commit on the
open pull request, which is separately assigned work; this run records the
concern only.

## References

- situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md
- situation/decisions/D-000014-ec-key-type-doc-correction-and-scoped-import-fidelity-supersession.md
- situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md
- situation/decisions/D-000016-msrv-baseline-is-compilation-proven.md
- situation/gaps/G-000017-stale-final-head-labels-in-retained-evidence.md —
  related earlier, closed concern about imprecise head labels; this gap is
  about unresolvable commit identifiers, not labels.
