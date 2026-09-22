# D-000005 — Unmaintained dev-path advisory ignores (adler, atty)

## Status

accepted

## Date

2026-09-22

## Context

Bringing the cargo-deny gate up at the pinned rust:1.98.0 container
(cargo-deny 0.20.2) surfaced two unmaintained-informational advisories
beyond RUSTSEC-2023-0071. Both sit exclusively on dev-dependency paths;
both upstream dev-dependencies (`color-backtrace 0.5`, `env_logger 0.9`)
are carried verbatim per the import fidelity rules. A gate that fails on
informational findings is dishonest CI, and ignoring them without a record
would be the blanket-ignore pattern the RUSTSEC disposition
(situation/decisions/D-000003-rustsec-2023-0071-rsa-disposition-scoped-ignore.md)
set out to avoid — so the ignores are recorded here, scoped, with their own
evidence.

## Evidence

- Gate log retained in-repo at
  situation/witnesses/evidence/W-000001/deny-check.log (cargo-deny 0.20.2,
  `cargo deny check`, sha256 in the neighboring digests file):
  `advisories FAILED, bans ok, licenses FAILED, sources ok` before this
  decision, with the two findings below.
- RUSTSEC-2025-0056 (https://rustsec.org/advisories/RUSTSEC-2025-0056):
  `adler` unmaintained, recommended alternative `adler2`, solution
  recorded as "no safe upgrade". Path: adler 1.0.2 <- miniz_oxide 0.7.4 <-backtrace 0.3.71 <- color-backtrace 0.5.1 <- (dev) oidc-client.
- RUSTSEC-2024-0375 (https://rustsec.org/advisories/RUSTSEC-2024-0375):
  `atty` unmaintained by official maintainer notice (superseded by
  `std::io::IsTerminal`, stable since 1.70), solution recorded as "no safe
  upgrade". Path: atty 0.2.14 <- color-backtrace 0.5.1 and env_logger 0.9.3<-(dev) oidc-client.
- The resolver held `backtrace` at 0.3.71 while 0.3.76 exists (cargo's
  `Adding backtrace v0.3.71 (available: v0.3.76)` line in the same log) —
  consistent with MSRV-aware fallback against this crate's
  `rust-version = "1.65"`. Interpretation, not verified mechanism.
- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
  — the dev-dependencies are carried verbatim from the pin.

## Decision

`deny.toml` additionally ignores RUSTSEC-2025-0056 (adler) and
RUSTSEC-2024-0375 (atty), each with a scoped reason: unmaintained-informational
only, no vulnerable code, dev-dependency paths only, upstream-pinned
dev-dependencies kept verbatim, revisit on deliberate dev-dependency
modernization. No other advisory classes are relaxed: every vulnerability
and every yanked-crate finding still fails the gate, and the license allow
list gains exactly `CDLA-Permissive-2.0` (webpki-roots root-store data via
reqwest) with unmatched allowances removed.

## Why

Unmaintained is not vulnerable: these advisories carry no vulnerability
finding, no production-reachable path, and no safe upgrade inside the
pinned upstream dev-dependencies. Recording them as scoped, evidenced
ignores keeps the gate strict where it matters and honest where it does
not.

## Rejected alternatives

- Bumping `color-backtrace`/`env_logger` to current majors: violates the
  verbatim dev-dependency rules in
  situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
  and adds unmandated scope.
- A global `severity-threshold` that skips informational advisories:
  weakens future vulnerability gating wholesale instead of naming the two
  findings.
- Leaving the gate red: dishonest CI for a standing pipeline.

## Consequences

The gate passes with exactly three advisory ignores: one vulnerability
disposition (situation/decisions/D-000003-rustsec-2023-0071-rsa-disposition-scoped-ignore.md)
and these two unmaintained-informational dev-path findings. Any further
advisory fails CI.

D-000008 promotes C-000003 into P-000004 and O-000005, which state the
resulting local dependency-policy contract without changing this Decision's
scoped rationale.

## Revisit when

The dev-dependencies are deliberately modernized past the upstream pins
(then re-derive the advisory surface), or either crate publishes a release
that clears the unmaintained notice.

## Provenance

Recorded 2026-09-22 by the Phase B import lane from the deny-gate bring-up
run at the pinned rust:1.98.0 container; gate log committed as witness
evidence the same day.
