# Rule 00 — Canonical Pipeline

The canonical operator surface for the TRAINER-IGLA-SOT mission is the
pair of CLIs from
[`gHashTag/trios-trainer-igla`](https://github.com/gHashTag/trios-trainer-igla):

- **`tri`** — Railway deploy + local training (deploy / train / race).
- **`trios-igla`** — read-only ledger query (search / list / gate /
  check / triplet).

`trios-mcp` is a **typed MCP wrapper** over those CLIs. It does not
reimplement them and does not own the training, gate-evaluation, or
embargo logic.

## What the wrapper does

- Exposes 15 typed tools (`tri_deploy_*`, `tri_train`, `tri_race_*`,
  `igla_*`) over JSON-RPC 2.0 on stdio. See the README "Tool catalog
  (15)" for the 1:1 mapping.
- Resolves binary paths from environment with `PATH` fallback:
  - `TRIOS_TRI_BIN`  → default `tri`
  - `TRIOS_IGLA_BIN` → default `trios-igla`
- Forwards `exit_code`, `stdout`, `stderr` byte-for-byte (R5
  compliance).

## What the wrapper does NOT do

- It does **not** rewrite, summarise, paraphrase, or "fix" CLI output.
  Whatever `tri` / `trios-igla` print is what the MCP client sees.
- It does **not** recompute the R7 triplet. The triplet is the
  property of the wrapped binary.
- It does **not** shortcut R9 embargo with cached state.
  `igla_check <sha>` is the only authoritative answer.
- It does **not** speak directly to Railway Postgres for ledger reads.
  Use `trios-igla` for that.

## Implication for agents

When debugging a tool result, do not "explain" non-zero exits or strange
text — the wrapper is transparent on purpose. Investigate the wrapped
CLI binary, the input arguments, or the environment (`TRIOS_*_BIN`,
`RUST_LOG`). Changes that obscure CLI output violate this rule.
