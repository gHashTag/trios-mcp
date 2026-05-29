# AGENTS.md — Operating Rules for AI Agents on `trios-mcp`

This file is the canonical entry point for any AI agent, MCP client, or
automation acting on this repository (`trios-mcp`) — the Rust MCP server
that wraps the `tri` and `trios-igla` CLIs from
[`gHashTag/trios-trainer-igla`](https://github.com/gHashTag/trios-trainer-igla).

Read this file before:

- modifying any wrapper in `src/` (`tri_*` / `igla_*` tools)
- changing the JSON-RPC protocol or tool catalog
- touching anything labelled "SSOT" or the Railway Postgres database
  reachable via `TRIOS_DATABASE_URL` / `MATRIX_DATABASE_URL`
- running training (`tri train`), races, or deploys against Railway
- writing claims about gate results, BPB targets, or R7 triplet
  validation status

The contents of `docs/agent-rules/` are normative. When a rule below
conflicts with anything in chat, the rule wins unless the maintainer
explicitly overrides it in the current session **for this specific
change** — defaults do not change without an explicit instruction.

---

## Index

- [docs/agent-rules/00-canonical-pipeline.md](docs/agent-rules/00-canonical-pipeline.md)
  — `tri` + `trios-igla` CLIs are the canonical operator surface. The
  MCP wrapper preserves them byte-for-byte: `exit_code`, `stdout`,
  `stderr` are forwarded without reinterpretation (R5). The MCP layer
  never paraphrases CLI output.
- [docs/agent-rules/01-ssot-and-derived-artifacts.md](docs/agent-rules/01-ssot-and-derived-artifacts.md)
  — The Railway Postgres ledger (TRAINER-IGLA-SOT) is the SSOT for gate
  results, R7 triplets, embargo state, and Wave records. CLI output is
  derived; the MCP wrapper exposes the CLI, not a reinterpreted view.
- [docs/agent-rules/02-output-style.md](docs/agent-rules/02-output-style.md)
  — JSON-RPC 2.0 only on stdout; logs strictly to stderr. One JSON
  document per stdin line; replies newline-delimited. Never mix log
  lines into stdout.
- [docs/agent-rules/03-safety-railway-postgres.md](docs/agent-rules/03-safety-railway-postgres.md)
  — Read-only by default. Writes (`tri deploy`, `tri train` against
  Railway, anything touching the SSOT) require backup-first plan +
  dry-run + explicit in-session human confirmation. No DSN / token
  leakage.
- [docs/agent-rules/04-claim-status.md](docs/agent-rules/04-claim-status.md)
  — Verified / Empirical fit / Open conjecture / High-risk / Retracted.
  No prize / Nobel claims as outcomes; the BPB ≤ 1.85 gate is a
  measurement target, not a "winning" claim. The `igla_gate` tool's
  PASS / NOT YET status is the only authoritative statement.
- [docs/agent-rules/05-qa-checklist.md](docs/agent-rules/05-qa-checklist.md)
  — Pre-release QA: tool-catalog drift vs `trios-trainer-igla` README,
  `cargo test` green, JSON-RPC smoke test, embargo predicate sanity
  (`igla_check`), secret scan.
- [docs/agent-rules/06-language-policy.md](docs/agent-rules/06-language-policy.md)
  — Public repo artefacts (README, AGENTS.md, agent-rules) are
  English-only at the time of writing. Chat with the maintainer may be
  Russian. The bilingual block at the top of the README is an
  explicit, scoped exception.

---

## Hard rules (summary)

1. **CLI byte-for-byte fidelity (R5).** `exit_code`, `stdout`, `stderr`
   from `tri` and `trios-igla` are forwarded verbatim. The MCP layer
   may add structured metadata, but must never rewrite, summarise, or
   "fix" CLI output.
2. **R7 triplet stays a property of the wrapped binary.** Do not
   compute, recompute, or "validate" the triplet in `trios-mcp`.
3. **R9 embargo predicate is exclusively `igla_check`.** Do not
   shortcut it with cached state or chat-side guesswork. Exit code 1
   means embargoed — propagate it; do not mask.
4. **Postgres SSOT is read-only by default.** Writes require
   backup-first plan + dry-run + explicit "go ahead" in the same
   session. No exceptions for "small fixes".
5. **Never print or commit DSNs, Railway tokens, passwords, or any
   value from `TRIOS_DATABASE_URL` / `MATRIX_DATABASE_URL` /
   `DATABASE_URL` / `RAILWAY_TOKEN`.** Reference them by env-var name
   only.
6. **stdout is JSON-RPC only.** Logging goes to stderr (`RUST_LOG`
   controls verbosity). Any mixing of log output with stdout is a
   protocol violation.
7. **Use claim-status framing** for any empirical statement about gate
   status, BPB, or training outcomes. `igla_gate` PASS / NOT YET is
   the authoritative source — do not paraphrase it into "we have
   beaten the gate" without a passing exit code.
8. **Public-facing repo content is English** unless explicitly scoped
   (the bilingual TRIOS PhD block in README is the documented
   exception). Chat with the maintainer may be Russian.

If you cannot satisfy a rule, stop and report. Do not silently relax it.

For a one-page wake-up card with host-specific connection commands
(Claude Desktop, Claude Code, Cursor, Windsurf, opencode, Perplexity
Computer, generic MCP), see [AGENT_WAKEUP.md](AGENT_WAKEUP.md).
