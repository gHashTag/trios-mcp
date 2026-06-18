# Rule 02 — Output Style (JSON-RPC discipline)

The MCP protocol layer follows strict input/output discipline:

- **stdout is JSON-RPC 2.0 only.** One JSON document per line, replies
  newline-delimited. No banners, no `println!`, no progress bars.
- **Logging goes to stderr.** Controlled by `RUST_LOG` (default `info`).
  Anything that needs to be seen by humans during a session goes to
  stderr.
- **Protocol version:** `2024-11-05`.
- **Supported methods:** `initialize`, `notifications/initialized`,
  `ping`, `tools/list`, `tools/call`.
- **`tools/call` envelope:**
  ```jsonc
  {
    "isError": false,           // true iff CLI exited non-zero
    "exit_code": 0,
    "stdout": "<verbatim CLI stdout>",
    "stderr": "<verbatim CLI stderr>"
  }
  ```
  No additional reinterpretation. If a future tool needs structured
  data (parsed JSON, derived fields), wrap that data in a clearly
  named property *alongside* the verbatim `stdout` / `stderr` — never
  in place of them.

## Forbidden

- Emitting log lines on stdout under any condition.
- Replacing `\r` / `\n` / ANSI escapes from CLI output.
- Localising or translating CLI output.
- Silently truncating large outputs without an explicit
  `truncated: true` marker.

## Rationale

The R5 contract from `trios-trainer-igla` requires byte-for-byte
forwarding. If the MCP layer paraphrases, downstream R7 / R9 checks may
diverge from CLI-direct verification.
