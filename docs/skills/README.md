# Agent Skills — `docs/skills/`

Pre-packaged [Agent Skills](https://agentskills.io) that mirror this
repository's operating rules. Designed to wake up any
agentskills-compatible host (Perplexity Computer, Claude Code with the
`skills` plugin, custom orchestrators) into the same operating posture
as `AGENTS.md` and `CLAUDE.md` require.

| File | Skill | When to load |
|------|-------|--------------|
| [`trios-mcp.zip`](trios-mcp.zip) | `trios-mcp` (operating rules) | Always — for any task touching the `trios-mcp` wrapper: changing tool wrappers, JSON-RPC protocol, the Railway Postgres ledger, claim-status framing, or running `tri` / `trios-igla` via MCP. |

## How to use

### Perplexity Computer

1. Open [Manage skills](https://www.perplexity.ai/computer/skills).
2. Upload `trios-mcp.zip` under "User skills".
3. The skill description includes trigger phrases; the platform
   auto-loads it when relevant tasks arrive.

### Other agentskills-compatible hosts

Unzip the archive and point your host at the resulting directory.
The directory has a `SKILL.md` (YAML frontmatter + body) plus a
`references/` folder with the 7 normative rule files.

## Companion skill

For the broader rules covering the GOLDEN CHAIN PDF pipeline and the
literature canon, see
[`gHashTag/trios-mcp-rag`](https://github.com/gHashTag/trios-mcp-rag) —
its `docs/skills/` folder ships two larger skills (`trios-mcp-rag`
and `trios-research-canon`).

## Provenance

This skill is derived from this repository. The zip here is the
authoritative copy; the matching source folders are `AGENTS.md` /
`CLAUDE.md` / `docs/agent-rules/`.
