# D-000003 — RUSTSEC-2023-0071 (rsa Marvin): scoped advisory ignore, verification-primary use

## Status

accepted

## Date

2026-09-22

## Context

The imported crate depends on `rsa = "0.9.2"` for RSA-family ID-token
signature verification. RUSTSEC-2023-0071 (the Marvin timing side-channel)
is open against the `rsa` crate with no patched release, so the cargo-deny
advisories gate would fail every run without a recorded disposition. The
disposition must state why the ignore is scoped, what it covers, and when
it must be re-examined.

## Evidence

- Advisory text (https://rustsec.org/advisories/RUSTSEC-2023-0071) and the
  OSV record (https://api.osv.dev/v1/vulns/RUSTSEC-2023-0071): the named leak
  paths are private-key-side operations (timing leak via decrypt / sign /
  unpad). The advisory re-confirmed 2026-09-14 lists no patched version;
  `patched = []` is intentional.
- https://github.com/RustCrypto/RSA/issues/626 — the upstream tracking
  issue; the fix work is unreleased RustCrypto/RSA PRs
  https://github.com/RustCrypto/RSA/pull/680 and
  https://github.com/RustCrypto/RSA/pull/702. Both `rsa 0.9.10` (stable)
  and `0.10.0-rc.18` are affected, so no bump escapes the advisory.
- situation/references/R-000001-upstream-pin.md — the pin declares
  `rsa = "0.9.2"`.
- The crate's byte-level rsa surface at the pin: public-key signature
  verification in `src/core/crypto.rs` (`rsa::RsaPublicKey` construction
  and verify); the upstream-exposed private-signing wrapper
  `CoreRsaPrivateSigningKey` in `src/core/jwk/mod.rs` (struct at line 549)
  signs through `rsa::RsaPrivateKey` (PKCS#1 v1.5 and PSS) for
  private_key_jwt-style client assertions.
- situation/candidates/C-000003-resolve-rustsec-2023-0071-rsa-marvin-disposition.md
  — the candidate this decision resolves.

## Decision

`deny.toml` ignores exactly RUSTSEC-2023-0071 with a scoped reason
carrying this fact set: no patched version exists (`patched = []` is
intentional); the advisory names private-key-side operations only; the
crate's primary and default rsa path is public-key signature verification,
which is not a named leak path; the exposed `CoreRsaPrivateSigningKey`
wrapper additionally routes private-key signing through `rsa`, so
consumers using that wrapper exercise the advisory's named surface and are
directed to this record. Revisit triggers: a fixed `rsa` release shipping
the RustCrypto/RSA #680/#702 work (then bump and drop the ignore), or an
advisory update that function-scopes the finding or names verification-side
leak paths. No other advisory is ignored.

## Why

The dependency boundary is the assurance boundary: a permanently red
advisories gate would either block CI dishonestly or invite blanket
ignores. With no fixed release available, the only honest disposition is a
single scoped ignore that names the residual risk — including the
private-signing wrapper — and self-arms its own re-examination triggers.

## Rejected alternatives

- Bumping `rsa`: impossible — no fixed release exists (0.9.10 and
  0.10.0-rc.18 both affected).
- Dropping `rsa`: removes RS256/384/512 verification, breaking the
  upstream-parity promise
  situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md.
- A function-scoped exemption: the advisory does not function-scope the
  finding, and inventing an exemption would overstate our assurance.
- Ignoring advisories broadly: disproportionate and dishonest.

## Consequences

`cargo deny check` passes with exactly one advisory ignore. Consumers who
use `CoreRsaPrivateSigningKey` to sign client assertions inherit the
advisory's named attack surface; this record and the `deny.toml` reason say
so plainly. Every other advisory fails the gate.

## Revisit when

A fixed `rsa` release carrying the RustCrypto/RSA #680/#702 work ships
(bump, drop the ignore), or the advisory is updated to function-scope the
finding or to name verification-side leak paths.

## Provenance

Recorded 2026-09-22 by the Phase B import lane, ahead of the `deny.toml`
disposition it governs. Corrected against evidence: the Phase A fact set
states this crate's only rsa use is public-key verification; byte-level
reading at the pin shows the public `CoreRsaPrivateSigningKey` wrapper
(`src/core/jwk/mod.rs`) also routes private-key signing through the `rsa`
crate — the correction is carried in Evidence and Consequences.
