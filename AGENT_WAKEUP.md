# AGENT_WAKEUP.md — Single-Page Wake-Up Card

Read this first when starting any AI session against this repo. It is a
one-page summary of [`AGENTS.md`](AGENTS.md) +
[`docs/agent-rules/`](docs/agent-rules/), plus host-specific connection
commands.

`trios-mcp` is the Rust MCP server that wraps `tri` and `trios-igla`
from
[`gHashTag/trios-trainer-igla`](https://github.com/gHashTag/trios-trainer-igla).

## 8 Hard Rules

1. **CLI byte-for-byte fidelity (R5).** Wrapper forwards `exit_code`,
   `stdout`, `stderr` verbatim. No paraphrase.
2. **R7 triplet stays a property of the wrapped binary.** Do not
   recompute in `trios-mcp`.
3. **R9 embargo via `igla_check` only.** Exit code 1 ⇒ embargoed —
   propagate.
4. **Postgres SSOT read-only by default.** Writes require backup-first
   plan + dry-run + explicit "go ahead".
5. **No DSN / token leakage.** `TRIOS_DATABASE_URL`,
   `MATRIX_DATABASE_URL`, `DATABASE_URL`, `RAILWAY_TOKEN` — env-var
   name only.
6. **stdout = JSON-RPC only.** Logs to stderr (`RUST_LOG`).
7. **Claim-status framing.** Verified / Empirical fit / Open
   conjecture / High-risk / Retracted. `igla_gate` PASS / NOT YET is
   the only authoritative gate statement.
8. **English public content.** Bilingual TRIOS PhD block at the top of
   README is the scoped exception. Chat with the maintainer may be
   Russian.

Full rules: [`docs/agent-rules/`](docs/agent-rules/).

## How to wake up an agent

Common prerequisites:

```bash
# 1. Build the MCP server binary
cargo build --release
# binary at: ./target/release/trios-mcp

# 2. Locate the wrapped CLIs (from trios-trainer-igla)
#    tri          — Railway deploy + local training
#    trios-igla   — read-only ledger query
# Set absolute paths in .env (NEVER commit it)
cp .env.example .env
# edit .env, fill TRIOS_TRI_BIN and TRIOS_IGLA_BIN
```

### Claude Desktop / Claude Code

`~/Library/Application Support/Claude/claude_desktop_config.json` (or
the equivalent for Claude Code):

```json
{
  "mcpServers": {
    "trios": {
      "command": "/abs/path/to/trios-mcp/target/release/trios-mcp",
      "env": {
        "TRIOS_TRI_BIN":  "/abs/path/trios-trainer-igla/target/release/tri",
        "TRIOS_IGLA_BIN": "/abs/path/trios-trainer-igla/target/release/trios-igla"
      }
    }
  }
}
```

A copy lives at [`examples/claude_desktop_config.json`](examples/claude_desktop_config.json).

For Claude Code CLI:

```bash
claude mcp add trios \
  -- sh -c 'set -a && . ./.env && exec ./target/release/trios-mcp'
claude mcp list
# Restart the Claude Code session — MCP tools load at session start.
```

The `sh -c '… set -a && . ./.env && exec …'` wrapper keeps DSNs and
binary paths out of the host config.

### Cursor / Windsurf / opencode

Use the host's MCP-server settings UI. Add an entry pointing at
`./target/release/trios-mcp` and populate `TRIOS_TRI_BIN` /
`TRIOS_IGLA_BIN` (and any DSN env vars) from `./.env` at launch.

### Perplexity Computer

1. Open [Manage skills](https://www.perplexity.ai/computer/skills).
2. Upload [`docs/skills/trios-mcp.zip`](docs/skills/trios-mcp.zip)
   under "User skills".
3. The skill description includes trigger phrases; Perplexity
   auto-loads it when relevant tasks arrive.

Perplexity Computer does not directly run the local MCP binary — it
uses the skill to follow the operating rules and references the repo
via its GitHub connector.

### Generic agentskills-compatible host

Unzip `docs/skills/trios-mcp.zip`, point your host's skill loader at
the result, and register `./target/release/trios-mcp` with your MCP
layer per host docs.

## Smoke test

```bash
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke","version":"0"}}}' \
  '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
  | ./target/release/trios-mcp
```

You should see well-formed JSON-RPC on stdout and `RUST_LOG`-controlled
logs on stderr. If logs leak to stdout, rule 02 is violated.

---

If anything in this card conflicts with chat, the rules win unless the
maintainer explicitly overrides them in the same session for the
specific change.
