# Rule 03 — Safety: Railway + Postgres

`trios-mcp` orchestrates tools that can deploy services, schedule
training runs, and (via the wrapped CLIs) read the SSOT ledger. Some
operations are destructive or expensive.

## Read-only by default

The following are **read-only** and may be called freely:

- `igla_search`, `igla_list`, `igla_gate`, `igla_check`, `igla_triplet`
- `tri_deploy_status`, `tri_deploy_logs`
- `tri_race_status`, `tri_race_best`

The following are **stateful and constrained**:

- `tri_deploy_init`, `tri_deploy_seed`, `tri_deploy_all`,
  `tri_deploy_remove` — touch Railway services.
- `tri_train` — burns compute and writes training artefacts.
- `tri_race_start` — starts a race against Railway.

## Write gate

Stateful tools require, in the same session:

1. **Backup-first plan.** A written sentence stating what gets
   captured (e.g. current Railway service list, ledger snapshot via
   `pg_dump --schema-only` plus relevant tables) and where.
2. **Dry-run.** Where supported, run the underlying CLI with a
   dry-run / `--help` invocation first and show the resolved arguments.
3. **Explicit "go ahead"** from the maintainer in the same session.
   Implicit acknowledgement ("looks fine") is not enough.

No exceptions for "small fixes". The MCP wrapper must not invent a
"force" flag that bypasses these conditions.

## Secret hygiene

- **Never log or commit:** `TRIOS_DATABASE_URL`, `MATRIX_DATABASE_URL`,
  `DATABASE_URL`, `RAILWAY_TOKEN`, any Postgres password, any value
  resembling a DSN, any UUIDv4 that could be a Railway API token.
- Reference these by env-var name only in code, docs, commits, PRs,
  issues, and chat.
- If a secret is accidentally pasted into a session, **stop**, rotate
  it before resuming, and confirm rotation in writing.
- `.gitleaks.toml` plus the pre-commit hook enforces this at commit
  time — see `.pre-commit-config.yaml`.
