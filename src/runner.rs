//! Subprocess runner for wrapped CLIs.
//!
//! R5: stdout, stderr, and exit code are forwarded verbatim. The MCP
//! layer never reinterprets them.

use std::process::Stdio;

use anyhow::{Context, Result};
use tokio::process::Command;

use crate::tools::{Bin, CliOutcome};

/// Resolve the binary path for `bin`, honouring the override env var.
pub fn resolve_bin(bin: Bin) -> String {
    std::env::var(bin.env_var()).unwrap_or_else(|_| bin.default_path().to_string())
}

/// Spawn the binary, await completion, capture stdout/stderr.
pub async fn run(bin: Bin, argv: Vec<String>) -> Result<CliOutcome> {
    let path = resolve_bin(bin);
    let mut cmd = Command::new(&path);
    cmd.args(&argv);
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let output = cmd
        .output()
        .await
        .with_context(|| format!("failed to spawn `{}` {:?}", path, argv))?;

    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    Ok(CliOutcome {
        bin: path,
        argv,
        exit_code,
        stdout,
        stderr,
    })
}
