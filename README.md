# trios-mcp

Rust **Model Context Protocol** server that wraps the `tri` and
`trios-igla` CLIs from
[`gHashTag/trios-trainer-igla`](https://github.com/gHashTag/trios-trainer-igla).

Anchor: `phi^2 + phi^-2 = 3`
Companion to the TRAINER-IGLA-SOT mission (Gate-2 deadline:
**2026-04-30 23:59 UTC**).
Tracking issue: [trios-mcp#1](https://github.com/gHashTag/trios-mcp/issues/1)
· Cross-ref:
[trios-trainer-igla#35](https://github.com/gHashTag/trios-trainer-igla/issues/35)

## TRIOS PhD — canonical context (RU + EN)

> **Не забыть / Do not forget:** генерация TRIOS PhD (`trios-phd`) реализована на **Rust** и является каноническим
> контекстом всей линии. The TRIOS PhD generation line is implemented in **Rust** and is canonical for this repo.

### Архитектура / Architecture

```
TRIOS MCP / Rust  ->  Postgres SSOT (Railway)  ->  PhD / article / brochure / PDF
```

- **TRIOS MCP (Rust)** — операторский слой, типизированные MCP-инструменты над `tri` / `trios-igla`.
- **Postgres SSOT on Railway** — единственный источник истины (single source of truth) для тезисов,
  Wave-результатов, refutation-логов и Admitted-учёта.
- **PhD / статья / брошюра / PDF / README** — **производные артефакты** (derived artifacts), генерируются
  из SSOT. Их нельзя править вручную как первоисточник — править надо данные в Postgres и Rust-генератор.

### Production / прод-правила

Обновление прода — **только** при выполнении всех трёх условий / update production **only** when:

1. **Backup-first** — свежий бэкап Postgres SSOT снят и проверен.
2. **Dry-run** — прогон миграции/генерации в dry-run режиме, diff просмотрен.
3. **Explicit confirmation** — явное подтверждение оператора (не «implicit OK»).

Никаких «горячих» правок прод-БД, никаких force-push, никаких авто-PR в основную линию без подтверждения.

### Статусы утверждений / Claim statuses

Любое утверждение в PhD/статье/брошюре должно нести один из статусов:

| Статус | Когда применяется |
|---|---|
| `verified`   | формально проверено (Coq/Rocq), подпись зафиксирована в SSOT |
| `empirical`  | подтверждено экспериментом (Wave-результат, `wave11results.md`) |
| `open`       | гипотеза, не доказано и не опровергнуто |
| `high-risk`  | вероятно неверно / зависит от шаткого предположения |
| `refuted`    | опровергнуто (теорема-опровержение, запись в `admittedlog.md`) |

Утверждения без статуса в публикацию **не идут** / claims without a status do not ship.

### Wave 10 / Wave 11, Admitted reduction, refutation theorems

- **Wave 10 / Wave 11** — итерации эмпирической валидации; результаты Wave 11 зафиксированы в
  `wave11results.md` (SSOT-derived).
- **Admitted reduction** — программа сокращения `Admitted` в Coq/Rocq-доказательствах; учёт ведётся в
  `admittedlog.md`. Каждое снятое `Admitted` — отдельная запись со ссылкой на доказательство.
- **Refutation theorems** — теоремы-опровержения сохраняются наравне с положительными результатами;
  они переводят соответствующие claims в статус `refuted` и являются полноправной частью корпуса.

### Воспроизводимость / Reproducibility

- **Dual Coq / Rocq** — доказательства должны собираться и в Coq, и в Rocq (двойная сборка, dual build).
  Расхождение между Coq и Rocq трактуется как regression и блокирует мерж.
- **Negative-results framing** — отрицательные результаты (refutations, неудачные Wave-прогоны,
  снятые гипотезы) публикуются явно. Это не «провалы», а первоклассные научные результаты, без
  которых PhD-линия не воспроизводима.

### Ссылки / References

- `wave11results.md` — итоги Wave 11 (empirical).
- `admittedlog.md`   — журнал снятий `Admitted` (verified / open transitions).
- Cross-ref: [trios-trainer-igla#35](https://github.com/gHashTag/trios-trainer-igla/issues/35).

## Why

`tri` (Railway deploy + local train) and `trios-igla` (read-only ledger
query) are the canonical operator surface for the TRAINER-IGLA-SOT
pipeline. Today every agent shells out manually, parses free-form text,
and reinvents the R7 triplet contract in its prompt. That is fragile and
silently bypasses R5/R7/R9.

`trios-mcp` exposes every command as a typed MCP tool over stdio. The
JSON-RPC layer never reinterprets CLI output:

- `exit_code`, `stdout`, `stderr` are forwarded byte-for-byte (R5)
- the R7 triplet stays a property of the wrapped binary (R7)
- `igla_check` is wired straight to the embargo predicate (R9)

## Build

```bash
cargo build --release
# binary: ./target/release/trios-mcp
```

## Quickstart — wake up an AI agent

Get any MCP-aware AI agent (Claude Desktop, Claude Code, Cursor,
Windsurf, opencode, Perplexity Computer) into the same operating
posture as the rules in [`AGENTS.md`](AGENTS.md).

> **TL;DR for new sessions:** read [`AGENT_WAKEUP.md`](AGENT_WAKEUP.md)
> for a one-page wake-up card with rules + host-specific connection
> commands + smoke test.

### Common prerequisites

```bash
# 1. Build the MCP server (done above)
cargo build --release

# 2. Provide environment locally via .env — NEVER commit it
cp .env.example .env
# edit .env, set TRIOS_TRI_BIN and TRIOS_IGLA_BIN to absolute paths
# and DSN env vars by NAME (no values in commits)
```

### Claude Desktop

See the dedicated [`Claude Desktop / Cursor / Computer`](#claude-desktop--cursor--computer)
section below — the JSON config and the canonical `mcpServers` block
live there. A copy of the config lives at
[`examples/claude_desktop_config.json`](examples/claude_desktop_config.json).

### Claude Code

> ⚠️ **Use `-s user` (global) — not the default local scope.**
> Local scope is per-project; an agent running from another directory
> won't see the server. `-s user` makes it available in all projects.
>
> ⚠️ **Do not pipe `claude mcp add`** — the confirmation prompt is
> swallowed and the entry is silently not written.
>
> ⚠️ **Use an absolute path** to the working directory — user-scope
> entries run from arbitrary cwd, so `./target/...` will fail.

```bash
# Replace /ABS/PATH/trios-mcp with the absolute path to your clone.
claude mcp add trios -s user -- \
  sh -c 'cd /ABS/PATH/trios-mcp && set -a && . ./.env && exec ./target/release/trios-mcp'

claude mcp list                    # trios … ✓ Connected
claude mcp get trios               # Scope: User (available in all your projects)
```

**Restart the Claude Code session** after registering — MCP tools are
loaded at session start.

**Reset** if a previous (broken) entry exists:

```bash
claude mcp remove trios                 # user-scope entry
claude mcp remove trios --scope local   # project-local leftover, if any
claude mcp list                          # confirm clean state
```

The `sh -c 'cd /ABS/PATH && set -a && . ./.env && exec …'` wrapper
keeps DSNs and binary paths out of the host configuration file.
This honours rule 5 of [`AGENTS.md`](AGENTS.md). The leading `cd`
is required because a user-scope entry has no fixed cwd.

### Cursor / Windsurf / opencode

In each host's MCP-server settings, add an entry that launches
`./target/release/trios-mcp` with env loaded from `./.env`. Verify
with the host's MCP-tools panel.

### Perplexity Computer

1. Open [Manage skills](https://www.perplexity.ai/computer/skills).
2. Upload [`docs/skills/trios-mcp.zip`](docs/skills/trios-mcp.zip)
   under "User skills".
3. The skill description includes trigger phrases; the platform
   auto-loads it when a relevant task arrives.

Perplexity Computer uses the skill for operating posture, not as the
local MCP runner. For direct CLI invocation, drive Claude Desktop /
Claude Code locally.

### Smoke test

```bash
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke","version":"0"}}}' \
  '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
  | ./target/release/trios-mcp
```

You should see well-formed JSON-RPC on stdout and any `RUST_LOG` lines
on stderr. Mixing logs into stdout violates rule 02.

## Tool catalog (15)

Mirrors
[trios-trainer-igla README §Commands](https://github.com/gHashTag/trios-trainer-igla#commands)
1:1.

### `tri deploy`
| Tool | Wraps |
|---|---|
| `tri_deploy_init`   | `tri deploy init` |
| `tri_deploy_seed`   | `tri deploy seed --seed <N> [--steps --hidden --lr --attn-layers]` |
| `tri_deploy_all`    | `tri deploy all` |
| `tri_deploy_status` | `tri deploy status` |
| `tri_deploy_logs`   | `tri deploy logs --seed <N>` |
| `tri_deploy_remove` | `tri deploy remove --seed <N>` |

### `tri train` / `tri race`
| Tool | Wraps |
|---|---|
| `tri_train`        | `tri train --seed <N> --steps <N> [--hidden --lr --attn-layers --optimizer --eval-every]` |
| `tri_race_start`   | `tri race start` |
| `tri_race_status`  | `tri race status` |
| `tri_race_best`    | `tri race best` |

### `trios-igla`
| Tool | Wraps | Exit codes |
|---|---|---|
| `igla_search`  | `trios-igla search [--seed --bpb-max --step-min --sha --gate-status]` | `0` hit, `2` no match |
| `igla_list`    | `trios-igla list [--last N]` | `0` |
| `igla_gate`    | `trios-igla gate [--target 1.85]` | `0` PASS, `2` NOT YET |
| `igla_check`   | `trios-igla check <sha>` (R9) | `0` clean, `1` embargoed |
| `igla_triplet` | `trios-igla triplet <index>` | `0` |

## Configuration

The server resolves binary paths from the environment, falling back to
`PATH` lookup:

| Env | Default |
|---|---|
| `TRIOS_TRI_BIN`  | `tri` |
| `TRIOS_IGLA_BIN` | `trios-igla` |
| `RUST_LOG`       | `info` (logs go to stderr; stdout is reserved for JSON-RPC) |

## Claude Desktop / Cursor / Computer

`~/Library/Application Support/Claude/claude_desktop_config.json`
(or your client's equivalent):

```json
{
  "mcpServers": {
    "trios": {
      "command": "/absolute/path/to/trios-mcp/target/release/trios-mcp",
      "env": {
        "TRIOS_TRI_BIN":  "/abs/path/trios-trainer-igla/target/release/tri",
        "TRIOS_IGLA_BIN": "/abs/path/trios-trainer-igla/target/release/trios-igla"
      }
    }
  }
}
```

A copy lives at [`examples/claude_desktop_config.json`](./examples/claude_desktop_config.json).

## Protocol

- JSON-RPC 2.0 over stdio
- Methods supported: `initialize`, `notifications/initialized`,
  `ping`, `tools/list`, `tools/call`
- Protocol version: `2024-11-05`
- One JSON document per stdin line; replies are newline-delimited

`tools/call` returns:

```jsonc
{
  "isError": false,           // true iff CLI exited non-zero
  "content": [{ "type": "text", "text": "$ tri deploy status\nexit_code: 0\n\n--- stdout ---\n..." }],
  "structuredContent": {
    "bin": "tri",
    "argv": ["deploy", "status"],
    "exit_code": 0,
    "stdout": "...",
    "stderr": ""
  }
}
```

## R-rule alignment

- **R1** Rust-only.
- **R5** No DONE without merged PR + green CI + ledger row. Tool results
  reflect the CLI verbatim; the MCP layer never invents success.
- **R7** R7 triplet is owned by the wrapped binaries, forwarded
  unchanged.
- **R9** `igla_check` exposes the embargo predicate. Always call it
  before any `ledger::emit_row`-equivalent action.
- **NO-COMMIT-WITHOUT-ISSUE** every commit references issue #1.
- **CANON_DE_ZIGFICATION** wraps existing binaries; touches no
  `.t27/.tri` specs or numeric/sacred-physics surfaces.

## Development

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

CI ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs the
same three checks on push and PR.

## License

Apache-2.0
