# Rule 01 — SSOT and Derived Artifacts

The **single source of truth** for the TRAINER-IGLA-SOT mission is the
Railway Postgres ledger (TRAINER-IGLA-SOT database) reached via:

- `TRIOS_DATABASE_URL` — primary ledger (gate results, R7 triplets,
  embargo state, Wave records).
- `MATRIX_DATABASE_URL` — matrix / training-run companion database (if
  configured).
- `DATABASE_URL` — generic fallback alias accepted by the underlying
  CLIs.

These DSNs are **referenced by name** in code, docs, and PRs. Values
live only in `.env` (gitignored) or your host's secret store.

## Authoritative vs derived

| Artefact | Status |
|----------|--------|
| Postgres ledger via `trios-igla` | **Authoritative** |
| `igla_gate` exit code (PASS / NOT YET) | **Authoritative** |
| `igla_check <sha>` exit code | **Authoritative** for R9 embargo |
| `igla_search` / `igla_list` / `igla_triplet` stdout | **Authoritative**, byte-for-byte |
| MCP JSON-RPC envelope from `trios-mcp` | Pass-through; metadata only |
| Chat summaries, README sentences, social posts | **Derived** — never authoritative |

## Implications for agents

- Never store gate status in chat memory and re-quote it without
  re-running `igla_gate`.
- Never claim "PASS" without an `igla_gate` exit code of 0 in the same
  session.
- Never claim a sha is "clean" without an `igla_check` exit code of 0
  in the same session.
- The MCP wrapper does not — and must not — cache ledger state across
  calls.
