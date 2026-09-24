# W-000031 — Content-Type essence complete-scope observation passes

## Promise

situation/promises/P-000012-content-type-essence-matches-rfc7231-optional-whitespace.md

## Oracle

situation/oracles/O-000022-judge-content-type-essence-complete-scope.md

## Result

PASS — every oracle leg is decided at this head. The retained `content_type`
battery executes the five finite helper/consumer fixtures, and the universal
legal-OWS legs (P1/F1), which the oracle designates structurally manual
because the grammar is unbounded, are decided by code inspection of the
trimming implementation named below at this head.

## Head

d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000031/cargo-test-content-type.log` retains
  `cargo test --offline --lib --quiet -- content_type` (`exit=0`): `test
  result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 105 filtered out`.
  The selected tests are
  `test_content_type_has_essence_accepts_case_and_optional_whitespace`,
  `test_content_type_has_essence_rejects_unexpected_and_malformed_values`,
  `test_check_content_type_tolerates_missing_header_and_accepts_ows`,
  `test_registration_sends_bearer_header_and_accepts_ows_content_type`, and
  `test_user_info_response_routes_on_content_type_with_optional_whitespace`.
  SHA-256
  `7c621398de377dcbe8042a9971d1dc8b147e88613405afc792c63010743dd625`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness observes one promise under one oracle at one head;
  promise state transitions remain with the closure corrector.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — structural: `content_type_has_essence` (`src/http_utils.rs` lines 15-33) splits the header value at the first `;` (lines 24-27), trims the essence slice with `trim_matches(|c: char| c == ' ' || c == '\t')` (lines 28-29), and compares with `eq_ignore_ascii_case` (line 30). `str::trim_matches` removes the maximal leading and trailing runs of exactly the two legal OWS bytes (SP, HTAB) with no length bound, so every legal sequence of SP/HTAB around the type/subtype — and therefore every arrangement before a parameter delimiter — reduces to the bare essence and matches. Bytes inside the essence are never trimmed, so only the declared optional whitespace is ignored. |
| P2 | PASS — `test_content_type_has_essence_accepts_case_and_optional_whitespace` covers `Application/JSON` among the accepted values (`src/http_utils.rs` lines 75-91). |
| P3 | PASS — `test_content_type_has_essence_rejects_unexpected_and_malformed_values` rejects the lookalike `application/jsonx`, the wrong `application/jwt` and `text/plain`, and the internal-whitespace `appl ication/json` (lines 93-108); internal whitespace survives because only the slice ends are trimmed (structural point above). |
| P4 | PASS (negative executed) — the same fixture rejects the non-UTF-8 header value `[0xff, 0xfe]` (lines 110-114); `to_str().ok()` (lines 16-18) makes any non-UTF-8 value fail closed. |
| P5 | PASS — `test_check_content_type_tolerates_missing_header_and_accepts_ows` keeps missing-header tolerance (`map_or(Ok(()), ...)` at `src/http_utils.rs` lines 35-54; fixture lines 135-150), accepts the OWS-bearing expected value, and rejects `text/plain`. |
| P6 | PASS — `test_registration_sends_bearer_header_and_accepts_ows_content_type` completes registration against a `APPLICATION/JSON ; charset=UTF-8` response (`src/registration/tests.rs` lines 1109-1146; the consumer is `check_content_type` in `register_response`, `src/registration/mod.rs` lines 643-649). |
| P7 | PASS — `test_user_info_response_routes_on_content_type_with_optional_whitespace` routes OWS-bearing JSON to `from_json` and OWS-bearing JWT to JWT verification rather than rejecting (`src/user_info.rs` lines 1045-1098; the router matches `content_type_has_essence` arms at lines 229-258 and defaults a missing Content-Type to JSON at line 227). |
| F1 | PASS (negative structural) — same structural decision as P1: for any legal SP/HTAB arrangement the trimmed essence equals the expected essence, so rejection cannot occur; the `is_some()` wrapper (line 32) yields `true` for every such arrangement. |
| F2 | PASS (negative executed) — internal-whitespace, lookalike, wrong, and non-UTF-8 values never match (P3/P4 fixtures). |
| F3 | PASS (negative executed) — `check_content_type` (P5), registration (P6), and UserInfo routing (P7) retain their declared response behavior on OWS-bearing Content-Types. |
| F4 | PASS (negative executed and structural) — missing-Content-Type tolerance executes in `check_content_type` (P5 fixture, `src/http_utils.rs` lines 135-138, via the `map_or(Ok(()), ...)` arm at lines 36-38). The UserInfo JSON default is structural: the router substitutes `MIME_TYPE_JSON` for a missing header (`src/user_info.rs` line 227), while the P7 fixture's recording clients always insert a Content-Type (lines 1070-1097), so the defaulting branch itself was inspected rather than dispatched. |
