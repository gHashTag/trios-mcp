# Rule 05 — QA Checklist

Run before tagging a release, merging a PR that touches `src/`, or
declaring "the wrapper is ready".

## Build & test

- [ ] `cargo build --release` clean
- [ ] `cargo test` clean
- [ ] `cargo clippy -- -D warnings` clean (or document each waived
      lint in the PR body)

## Tool-catalog drift

- [ ] Every command currently documented in the
      [`trios-trainer-igla` README §Commands](https://github.com/gHashTag/trios-trainer-igla#commands)
      has a corresponding MCP tool in `src/`.
- [ ] No MCP tool exists that no longer maps to a CLI command (orphans
      removed or marked deprecated).
- [ ] The README "Tool catalog (15)" table is updated to match the
      actual count.

## Protocol smoke test

- [ ] `printf '{"jsonrpc":"2.0","id":1,"method":"initialize", ...}\n' \
        | ./target/release/trios-mcp` returns a well-formed JSON-RPC
      response.
- [ ] `tools/list` enumerates exactly the catalog above.
- [ ] One read-only `tools/call` per family (`igla_list`,
      `tri_deploy_status`, `tri_race_status`) returns
      `isError: false` against a configured environment.

## Output discipline

- [ ] Inspect stderr / stdout separately. Nothing non-JSON on stdout.
- [ ] `RUST_LOG=trace` produces logs only on stderr.

## Embargo predicate sanity

- [ ] `igla_check` against a known-embargoed sha returns exit code 1
      and the wrapper surfaces `isError: true` plus the byte-verbatim
      stderr.

## Secret scan

- [ ] `pre-commit run --all-files` passes.
- [ ] `gitleaks detect --config=.gitleaks.toml` clean against the
      working tree.
- [ ] No new file in the PR contains a `postgresql://` literal with an
      embedded password.

## Claim-status pass

- [ ] Every new empirical claim in README / CHANGELOG / commit
      messages carries a label from rule 04.
- [ ] Every Open-conjecture claim carries a `falsification_path:`.
