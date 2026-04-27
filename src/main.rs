//! `trios-mcp` — MCP server over stdio for the `tri` and `trios-igla`
//! CLIs shipped by [`gHashTag/trios-trainer-igla`].
//!
//! Anchor: phi^2 + phi^-2 = 3.
//!
//! Standing rules (binding):
//!   R1 — Rust-only.
//!   R5 — exit code, stdout, stderr forwarded verbatim.
//!   R7 — R7 triplet (`BPB=<v> @ step=<N> seed=<S> sha=<7c> jsonl_row=<L> gate_status=<g>`)
//!        is a property of the wrapped binaries; never invented here.
//!   R9 — `igla_check` exposes the embargo predicate.
//!
//! See `README.md` for the tool catalog and Claude Desktop / Cursor
//! configuration examples.

mod protocol;
mod runner;
mod tools;

use anyhow::Result;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use protocol::{codes, Request, Response, PROTOCOL_VERSION};

const SERVER_NAME: &str = "trios-mcp";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    // Logs go to stderr only — stdout is reserved for JSON-RPC frames.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .compact()
        .init();

    tracing::info!(
        version = SERVER_VERSION,
        tri_bin = %runner::resolve_bin(tools::Bin::Tri),
        igla_bin = %runner::resolve_bin(tools::Bin::Igla),
        "trios-mcp starting on stdio"
    );

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut stdout = tokio::io::stdout();

    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let response = handle_line(&line).await;

        // Notifications (no id) generate no response. JSON-RPC says we
        // must drop them silently.
        if let Some(resp) = response {
            let mut bytes = serde_json::to_vec(&resp)?;
            bytes.push(b'\n');
            stdout.write_all(&bytes).await?;
            stdout.flush().await?;
        }
    }

    Ok(())
}

async fn handle_line(line: &str) -> Option<Response> {
    let req: Request = match serde_json::from_str(line) {
        Ok(r) => r,
        Err(e) => {
            return Some(Response::err(
                Value::Null,
                codes::PARSE_ERROR,
                format!("parse error: {}", e),
            ));
        }
    };

    let id_for_response = req.id.clone();

    // No id => notification. Process, but never respond.
    let is_notification = req.id.is_none();
    let id = id_for_response.unwrap_or(Value::Null);

    let result = dispatch(&req).await;

    if is_notification {
        return None;
    }

    Some(match result {
        Ok(value) => Response::ok(id, value),
        Err(DispatchError::MethodNotFound(m)) => Response::err(
            id,
            codes::METHOD_NOT_FOUND,
            format!("unknown method `{}`", m),
        ),
        Err(DispatchError::InvalidParams(msg)) => Response::err(id, codes::INVALID_PARAMS, msg),
        Err(DispatchError::Internal(msg)) => Response::err(id, codes::INTERNAL_ERROR, msg),
    })
}

#[derive(Debug)]
enum DispatchError {
    MethodNotFound(String),
    InvalidParams(String),
    Internal(String),
}

async fn dispatch(req: &Request) -> Result<Value, DispatchError> {
    match req.method.as_str() {
        "initialize" => Ok(json!({
            "protocolVersion": PROTOCOL_VERSION,
            "capabilities": {
                "tools": { "listChanged": false }
            },
            "serverInfo": {
                "name": SERVER_NAME,
                "version": SERVER_VERSION
            }
        })),

        "notifications/initialized" => Ok(Value::Null),
        "ping" => Ok(json!({})),

        "tools/list" => {
            let tools: Vec<Value> = tools::catalog()
                .into_iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "inputSchema": t.input_schema,
                    })
                })
                .collect();
            Ok(json!({ "tools": tools }))
        }

        "tools/call" => handle_tools_call(&req.params).await,

        other => Err(DispatchError::MethodNotFound(other.to_string())),
    }
}

async fn handle_tools_call(params: &Value) -> Result<Value, DispatchError> {
    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| DispatchError::InvalidParams("missing `name`".into()))?
        .to_string();

    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));

    let descriptor = tools::catalog()
        .into_iter()
        .find(|t| t.name == name)
        .ok_or_else(|| DispatchError::InvalidParams(format!("unknown tool `{}`", name)))?;

    let argv =
        tools::build_args(&name, &args).map_err(|e| DispatchError::InvalidParams(e.to_string()))?;

    let outcome = runner::run(descriptor.bin, argv)
        .await
        .map_err(|e| DispatchError::Internal(e.to_string()))?;

    // R5: present the verbatim outcome. The `isError` flag mirrors the
    // CLI exit code so an MCP client sees the failure honestly.
    let is_error = outcome.exit_code != 0;
    let header = format!(
        "$ {} {}\nexit_code: {}\n",
        outcome.bin,
        outcome.argv.join(" "),
        outcome.exit_code
    );
    let mut body = header;
    if !outcome.stdout.is_empty() {
        body.push_str("\n--- stdout ---\n");
        body.push_str(&outcome.stdout);
    }
    if !outcome.stderr.is_empty() {
        if !body.ends_with('\n') {
            body.push('\n');
        }
        body.push_str("\n--- stderr ---\n");
        body.push_str(&outcome.stderr);
    }

    Ok(json!({
        "isError": is_error,
        "content": [
            { "type": "text", "text": body }
        ],
        "structuredContent": {
            "bin": outcome.bin,
            "argv": outcome.argv,
            "exit_code": outcome.exit_code,
            "stdout": outcome.stdout,
            "stderr": outcome.stderr,
        }
    }))
}
