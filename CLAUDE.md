# CLAUDE.md

Entry point for Claude Code sessions on this repo. Read
[`AGENTS.md`](AGENTS.md) first — it is the canonical rule set and
applies to all AI agents (Claude Code, Claude Desktop, Cursor,
opencode, Windsurf, MCP automation, RAG runs).

Quick summary of the hard rules:

- `tri` and `trios-igla` CLI output is forwarded byte-for-byte (R5).
  Never rewrite, summarise, or "fix" CLI output in the wrapper.
- The R7 triplet stays a property of the wrapped binary. Do not
  recompute it in `trios-mcp`.
- R9 embargo is enforced exclusively by `igla_check`. Exit code 1 means
  embargoed — propagate it.
- Railway Postgres SSOT is read-only by default. Writes need
  backup-first + dry-run + explicit in-session confirmation.
- Never log or commit DSNs / tokens. `TRIOS_DATABASE_URL`,
  `MATRIX_DATABASE_URL`, `DATABASE_URL`, `RAILWAY_TOKEN` are referenced
  by name only.
- stdout is JSON-RPC only. Logging goes to stderr.
- Use claim-status framing — `igla_gate` PASS / NOT YET is the only
  authoritative gate statement.
- Public artefacts are English (bilingual TRIOS PhD block in README is
  the documented exception). Chat with the maintainer may be Russian.

See:

- [AGENT_WAKEUP.md](AGENT_WAKEUP.md) — one-page wake-up card (rules +
  host-specific connection commands)
- [AGENTS.md](AGENTS.md) — full index of rules
- [docs/agent-rules/](docs/agent-rules/) — normative rule files

When a chat instruction conflicts with these rules, the rules win
unless the maintainer explicitly overrides them for the specific
change in the same session.
