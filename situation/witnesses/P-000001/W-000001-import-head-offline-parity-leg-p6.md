# W-000001 — Import head offline parity: oracle leg P6 plus full gate suite

## Promise

situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md

## Oracle

situation/oracles/O-000001-judge-full-oidc-rp-flow-parity-with-upstream.md

## Result

INVALID — a partial observation, recorded honestly: oracle leg P6 is PASS
with committed evidence; legs P1–P5 were NOT executed. Per the witnesses
law, a PASS that omits an oracle leg is INVALID, not a partial PASS. Legs
P1–P5 (live-flow parity against a real OIDC provider) remain unexecuted
pending the ES256 lane's fixtures
(situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md)
and later qualification; this
witness does not claim them. The observation additionally recorded the full
standing gate suite (fmt, clippy, test, deny) green at the same head.

## Head

24835e4b44caa8a0baae2ac5b865bbd6bdf355ba (working tree clean at run time;
git is not installed inside the pinned container, so the head is recorded
here and in the evidence SHA256SUMS header rather than in the raw log)

## Observed

2026-09-22

## Evidence

All retained in-repo under situation/witnesses/evidence/W-000001/ with
sha256 digests in situation/witnesses/evidence/W-000001/SHA256SUMS:

- gates-final.log — consolidated run at the head above, inside the pinned
  container rust:1.98.0 (digest in image-digest.txt, itself
  rust@sha256:620dbcd124499c59e2406d3741574b5c5838cf9eb9656f0c3a03948f79b02959),
  CARGO_TARGET_DIR=/tmp/target:
  `cargo fmt --all --check` exit 0; `cargo clippy --all-targets --
  -D warnings` exit 0; `cargo test --all-features` exit 0 with
  70 unit tests passed / 0 failed, doctests 7 passed / 2 ignored, and the
  21 live-network certification tests all compiled and still `#[ignore]`d
  (19 + 2 across the two certification binaries);
  `cargo deny check` exit 0.
- deny-check.log — the first cargo-deny bring-up run (cargo-deny 0.20.2),
  retained FAILED as it stood before the dispositions in
  situation/decisions/D-000003-rustsec-2023-0071-rsa-disposition-scoped-ignore.md
  and
  situation/decisions/D-000005-unmaintained-dev-path-advisory-ignores.md
  (`advisories FAILED, bans ok, licenses FAILED, sources ok`).
- image-digest.txt — the pinned container image digest.

Delta classification vs upstream pin
b639b5d39eac6903238867aeb2b29326502e6b26 (tag 4.0.1), per the categories
predeclared in
situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md:

- metadata (3 lines, root Cargo.toml): `name` → `oidc-client`;
  `version` → `4.1.0`
  (situation/decisions/D-000002-version-oidc-client-4-1-0-continuing-upstream-lineage.md);
  `repository` → `https://github.com/cleverunicornz/oidc-client`.
- rename (34 lines, compilation-required only): 16 `use` lines —
  tests/rp_certification_code.rs 10, 16, 17; tests/rp_common.rs 4, 8, 100;
  examples/gitlab.rs 19, 23, 24, 25; examples/google.rs 17, 23, 24;
  examples/okta_device_grant.rs 18, 24, 25 (`openidconnect::` →
  `oidc_client::`); 17 doc-comment import-path lines —
  src/lib.rs 14, 24, 136, 151, 159, 281, 291, 373, 374, 415, 426, 507,
  522, 529 (line 14 renames the heading text and its
  `#importing-openidconnect-...` anchor in lockstep), src/client.rs 36–37,
  src/jwt/mod.rs 62; plus the Cargo dependency example src/lib.rs 82,
  which renames to the Cargo package name `oidc-client` (a dependency key
  resolves the package name, not the import identifier — confirmed by a
  `cargo add` probe during validation).
- mechanical-lint (22 items, behavior-neutral, commits d51e1f4, 12d22ab,
  36f761d5): mismatched_lifetime_syntaxes ×11 (src/client.rs 642, 765, 855,
  868, 989, 1005, 1124, 1281, 1366, 1454; tests/rp_certification_code.rs
  183); needless_lifetimes ×4 (src/id_token/mod.rs 348, 366;
  src/user_info.rs 425, 444); deprecated `GenericArray::as_slice` ×3
  (src/core/jwk/mod.rs 426, 433, 440 — deref-to-slice then `to_vec()`);
  empty_line_after_doc_comments ×1 (src/core/crypto.rs 99 removed);
  doc_overindented_list_items ×2 followed by doc_lazy_continuation
  correction (src/lib.rs 69–70, 3-space continuation); useless_conversion
  ×1 (src/jwt/mod.rs 237 — identity `map_err(Into::into)` removed).
- allow-addition (2, each with a justification comment): clippy::to_string_trait_impl
  on `IdToken` (src/id_token/mod.rs 175 — upstream's serde-based
  `ToString` with panic-on-failure semantics kept verbatim); dead_code on
  the shared `PanicIfFail` helper (tests/rp_common.rs 116 — used by
  rp_certification_code only, removal would break that suite); commits
  159e9a7 and 12d22ab.
- authored new files (not upstream deltas): README.md, NOTICE.md,
  .github/workflows/ci.yml, deny.toml, and the situation/ records and
  evidence of this lane.
- path-level move (byte-identical): upstream README.md preserved as
  README.upstream.md.
- verbatim by policy (zero delta): everything else, expressly including
  the rp.certification.openid.net/openidconnect-rs URL fixtures
  (src/discovery/tests.rs, src/logout.rs), src/lib.rs 124, the
  src/id_token/tests.rs 468 issue URL, tests/rp_common.rs 19
  (`RP_NAME`), the `admin@openidconnect.local` fixtures
  (src/registration/tests.rs 29, 101, 328, 431), examples/google.rs 48
  prose, UPGRADE.md, LICENSE (byte-identical), and Cargo-1.65.lock
  (inert, unchanged, root entry `openidconnect 4.0.1`).

## Oracle legs

- P1 Discovery fetched and parsed — NOT EXECUTED at this head (requires
  the live-flow fixture matrix; pending).
- P2 Authorization-code flow with PKCE S256 — NOT EXECUTED (same).
- P3 ID token verification RS256/384/512, PS256/384/512, EdDSA — NOT
  EXECUTED as a live flow (same); the offline unit suite covering these
  algorithms passes inside leg P6.
- P4 UserInfo endpoint — NOT EXECUTED (same).
- P5 Refresh token exchange — NOT EXECUTED (same).
- P6 All upstream test suite tests pass unchanged (excluding renamed
  imports) — PASS: `cargo test --all-features` exit 0 at the head with
  70 passed / 0 failed and the certification tests compiled and ignored;
  evidence situation/witnesses/evidence/W-000001/gates-final.log.

## Provenance

Recorded 2026-09-22 by the Phase B import lane from the gate run against
head 24835e4b44caa8a0baae2ac5b865bbd6bdf355ba in the pinned rust:1.98.0
container; evidence committed the same day.
