# oidc-client

`oidc-client` is a maintained continuation of the
[`openidconnect`](https://github.com/ramosbugs/openidconnect-rs) crate: an
OpenID Connect relying-party (client) library for Rust. It supports OpenID
Connect discovery, authorization-code flow with PKCE, ID token verification
(RS256/384/512, PS256/384/512, EdDSA), UserInfo, refresh tokens, end-session
(logout), and dynamic client registration.

The code is carried forward verbatim from the upstream `4.0.1` release
(imported at commit
[b639b5d39eac6903238867aeb2b29326502e6b26](https://github.com/ramosbugs/openidconnect-rs/tree/b639b5d39eac6903238867aeb2b29326502e6b26),
tag [`4.0.1`](https://github.com/ramosbugs/openidconnect-rs/releases/tag/4.0.1))
and released as `4.1.0` to signal that continuity. Planned work on this
continuation includes ES256 (ECDSA P-256) ID-token verification support.

## Status

Maintained continuation. The upstream project
([ramosbugs/openidconnect-rs](https://github.com/ramosbugs/openidconnect-rs))
has not been actively maintained; this repository carries the library
forward and continues its maintenance. The upstream `README.md` is
preserved unmodified as [`README.upstream.md`](README.upstream.md), and the
upstream upgrade notes as [`UPGRADE.md`](UPGRADE.md).

## Attribution

This crate exists because of the work of David A. Ramos (ramosbugs) and
contributors on `openidconnect`. The upstream code is licensed under the
MIT license; the original [`LICENSE`](LICENSE) file is preserved unmodified
in this repository, and [`NOTICE.md`](NOTICE.md) carries the formal
attribution.

## License

MIT — see [`LICENSE`](LICENSE) and [`NOTICE.md`](NOTICE.md).
