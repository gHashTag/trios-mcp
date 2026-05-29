# Rule 07 — MCP Server Registration (Claude Code & peers)

Mirror of Rule 08 in `gHashTag/trios-mcp-rag` — applied to the
`trios` (CLI-wrapper) MCP server published by this repo.

## 07.1 Use `-s user` (global) scope

`claude mcp add` defaults to **local scope** (per-project, invisible
elsewhere). Always pass `-s user` so the entry lives in the user
config and every project / agent session can see it.

```bash
claude mcp add trios -s user -- \
  sh -c 'cd /ABS/PATH/trios-mcp && set -a && . ./.env && exec ./target/release/trios-mcp'
```

## 07.2 Absolute path in the wrapper's `cd`

User-scope entries run from arbitrary cwd. `./target/...` and
`./.env` will fail outside the build folder. Pin the directory with
`cd /ABS/PATH/trios-mcp &&` before sourcing the env or exec-ing the
binary.

## 07.3 Never pipe `claude mcp add`

Piping closes stdin and the confirmation prompt is silently
swallowed. Run the command directly in an interactive shell. Do not
document piped variants.

## 07.4 Verify, then restart

```bash
claude mcp list                    # entry present
claude mcp get trios               # Status: ✓ Connected; Scope: User
```

MCP servers are launched at session start — restart the Claude Code
session before declaring success.

## 07.5 Reset both scopes before re-adding

```bash
claude mcp remove trios
claude mcp remove trios --scope local
```

A stale local-scope entry shadows the correct user-scope entry
inside its owning directory.

## 07.6 What to report on failure

1. `claude mcp list`
2. `claude mcp get trios`
3. Exact `add` command (paths / DSNs redacted to env-var names per
   Rule 05)
4. `claude --version`

Without these four items, no diagnosis is possible.

## 07.7 Wrapper DSN handling stays Rule 05–compliant

The `set -a && . ./.env && exec ...` pattern remains required: the
DSN and any `TRIOS_TRI_BIN` / `TRIOS_IGLA_BIN` paths must be
resolved from `./.env` at server-start time only, never written to
Claude Code's MCP config file.
