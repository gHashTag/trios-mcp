//! End-to-end smoke test: spawn `trios-mcp`, perform initialize +
//! tools/list over stdio, assert protocol shape.
//!
//! This test does NOT exercise `tools/call` because that requires real
//! `tri` / `trios-igla` binaries on PATH; that path is exercised via
//! the unit tests in `src/tools.rs` (argument-builder coverage) and is
//! verified in production against the actual binaries.

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

fn cargo_bin() -> std::path::PathBuf {
    let exe = env!("CARGO_BIN_EXE_trios-mcp");
    std::path::PathBuf::from(exe)
}

#[test]
fn initialize_and_list_tools() {
    let mut child = Command::new(cargo_bin())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn trios-mcp");

    let mut stdin = child.stdin.take().expect("stdin");
    let init = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "smoke", "version": "0.0.0" }
        }
    });
    let list = serde_json::json!({
        "jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}
    });

    writeln!(stdin, "{}", init).unwrap();
    writeln!(stdin, "{}", list).unwrap();
    drop(stdin);

    // Give the child a moment to write both responses.
    std::thread::sleep(Duration::from_millis(500));

    let output = child.wait_with_output().expect("wait");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();

    let r1: serde_json::Value = serde_json::from_str(lines.next().expect("init reply")).unwrap();
    assert_eq!(r1["id"], 1);
    assert_eq!(r1["result"]["protocolVersion"], "2024-11-05");
    assert_eq!(r1["result"]["serverInfo"]["name"], "trios-mcp");

    let r2: serde_json::Value = serde_json::from_str(lines.next().expect("list reply")).unwrap();
    assert_eq!(r2["id"], 2);
    let tools = r2["result"]["tools"].as_array().expect("tools array");
    assert_eq!(tools.len(), 15, "exactly 15 tools must be registered");

    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    for required in [
        "tri_deploy_init",
        "tri_deploy_seed",
        "tri_deploy_all",
        "tri_deploy_status",
        "tri_deploy_logs",
        "tri_deploy_remove",
        "tri_train",
        "tri_race_start",
        "tri_race_status",
        "tri_race_best",
        "igla_search",
        "igla_list",
        "igla_gate",
        "igla_check",
        "igla_triplet",
    ] {
        assert!(names.contains(&required), "missing tool {}", required);
    }
}
