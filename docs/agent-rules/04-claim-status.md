# Rule 04 — Claim Status

Every empirical statement made by an agent about gate status, BPB,
training outcomes, or R7 triplet validity must carry a status label.
This mirrors the framing in
[`gHashTag/trios-mcp-rag`](https://github.com/gHashTag/trios-mcp-rag/blob/main/docs/agent-rules/04-claim-status.md).

## Labels

| Label | Meaning |
|-------|---------|
| **Verified** | Backed by a same-session `igla_gate` / `igla_check` exit code, a passing `cargo test`, or a Railway-side audited record. |
| **Empirical fit** | Pattern matches recent runs but is not gated by an authoritative tool call. |
| **Open conjecture** | A hypothesis. Must carry a written `falsification_path` — what observation would disconfirm it. |
| **High-risk** | Plausible but with known counter-evidence or large unknowns. Cite both sides. |
| **Retracted** | Previously claimed, now withdrawn. Keep the retraction visible in the same place the original claim appeared. |

## Forbidden framings

- **Prize / Nobel claims as deliverables.** Beating the BPB ≤ 1.85
  gate is a measurement target, not a "won the prize" statement. Any
  external recognition is a long-term external-validation standard,
  not a deliverable.
- **"PASS"** without an `igla_gate` exit code of `0` from the same
  session.
- **"Embargo clear"** without an `igla_check <sha>` exit code of `0`
  from the same session.
- **"R7 triplet validated"** without the wrapped CLI producing it.
  `trios-mcp` does not compute triplets.

## Operational requirement

Every Open-conjecture claim in code comments, docs, or derived
artefacts must carry an inline `falsification_path:` note. Builds /
releases that contain Open-conjecture claims without falsification
paths fail the QA checklist (rule 05).
