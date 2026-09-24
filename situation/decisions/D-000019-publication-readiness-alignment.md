# D-000019 — Publication-readiness alignment of README and packaging for crates.io

## Status

accepted

## Date

2026-09-24

## Context

The crate is preparing for its first publication on crates.io, and `README.md`
is the registry-rendered surface: crates.io renders `README.md` outside this
repository, so every repository-internal link in it must resolve without a
repository checkout. The pre-review release-boundary check that preceded this
decision found no `include` or `exclude` in `Cargo.toml`, so the source tarball
shipped the full repository — 232 files under `situation/` plus `.github/`,
`AGENTS.md`, `README.upstream.md`, and `deny.toml`. The same check found that
`README.md` carried a point-in-time claim ("the manifest declares version
4.1.0, but the crate is not yet published on crates.io") that becomes false at
publication, repository-relative links that cannot resolve when rendered on
crates.io, and a `description` ("OpenID Connect library") too thin for
registry discovery.

## Evidence

- Pre-change packaging observation, reproduced on main at `8ebe5da` with
  `cargo package --list | cut -d/ -f1 | sort | uniq -c | sort -rn`:

  ```
      232 situation
       30 src
        3 tests
        3 examples
        1 deny.toml
        1 UPGRADE.md
        1 README.upstream.md
        1 README.md
        1 NOTICE.md
        1 LICENSE
        1 Cargo.toml.orig
        1 Cargo.toml
        1 Cargo.lock
        1 AGENTS.md
        1 .gitignore
        1 .github
        1 .cargo_vcs_info.json
  ```

- `8ebe5da:Cargo.toml` — no `include` or `exclude` key present; `description`
  was `"OpenID Connect library"`.
- `8ebe5da:README.md` — the unpublished-on-crates.io claim in the Status
  section, and repository-relative link targets
  (`situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`,
  `LICENSE`, `NOTICE.md`, and six more `situation/` paths) that cannot
  resolve on the crates.io rendering.
- situation/promises/P-000003-published-on-crates-io-as-oidc-client.md — the
  publication promise; this work does not change its state.
- situation/decisions/D-000017-prepublication-hardening-maintenance-delta-registration-expiry-surface.md
  and situation/decisions/D-000018-reject-client-secret-expiration-never-expires-sentinel-collisions.md
  — the selected `ClientSecretExpiration` semantics that the refreshed README
  capability list summarizes.
- Post-change verification on this branch: `cargo package --list` and the
  packed `target/package/oidc-client-4.1.0.crate` contain no `situation/`,
  `.github/`, `AGENTS.md`, `README.upstream.md`, or `deny.toml` entries;
  `cargo package --allow-dirty --offline` built the package successfully.

## Decision

1. `README.md` is aligned to its crates.io rendered target: every
   repository-internal link is an absolute
   `https://github.com/cleverunicornz/oidc-client/blob/main/<path>` URL, the
   point-in-time unpublished claim is removed, and the Status section states
   the crate is version 4.1.0, a maintained continuation of the
   openidconnect 4.0.1 baseline.
2. The README capability summary is refreshed to the current release,
   including one orientation-level bullet for the deliberate API divergence
   in which `client_secret_expires_at` is `Option<ClientSecretExpiration>`
   with explicit `NeverExpires` / `ExpiresAt` semantics.
3. `Cargo.toml` gains exactly
   `exclude = ["/situation", "/.github", "/AGENTS.md", "/README.upstream.md", "/deny.toml"]`,
   and the description becomes
   `"OpenID Connect relying-party library — maintained continuation of openidconnect"`.
4. `UPGRADE.md` ships in the package: it is upstream's 2.x→4.x migration
   guide, useful to arriving consumers — release documentation, not
   repository-operational knowledge.
5. Version, authors, license, keywords, `rust-version`, features, and docs.rs
   metadata are untouched.

## Why

crates.io renders `README.md` outside the repository, so repository-relative
links break on the rendered page and repository-internal content ships
needlessly. The source tarball is the published product surface and should
carry product files only.

## Rejected alternatives

- Shipping `situation/` in the package: it is agent-operational knowledge,
  not consumer documentation, and it would bloat the tarball by hundreds of
  files (232 at the time of this decision).
- Excluding `UPGRADE.md`: strips the 2.x→4.x migration guidance arriving
  consumers need; it is release documentation, not repo-internal orientation.
- Keeping repository-relative links: they break on the crates.io rendering.
- Keeping the unpublished-on-crates.io claim: it is false the moment the
  crate publishes.
- A changelog-style README: the README is orientation, not a changelog.

## Consequences

The package ships product files only, and README links resolve on crates.io.
No situation record state changes:
situation/promises/P-000003-published-on-crates-io-as-oidc-client.md remains
`hypothesis` until publication has its own witness. Future file additions
must re-check the exclude list.

## Revisit when

The rendered target changes (crates.io link semantics), the repository gains
files that must ship or must not ship, or the README template changes.
