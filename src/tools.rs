//! Tool definitions for the trios-mcp server.
//!
//! Each tool wraps exactly one subcommand of the `tri` or `trios-igla`
//! binary. The mapping is documented in the README's "Tool catalog"
//! section and frozen by `tests/tools_catalog.rs`.
//!
//! R5: argument builders never inject hidden defaults; they reflect
//! exactly what the user requested, the same way a human would type the
//! flags. Every CLI flag is optional unless required by the underlying
//! command (e.g. `--seed` for `tri deploy seed`).

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Which binary the tool dispatches to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bin {
    Tri,
    Igla,
}

impl Bin {
    pub fn env_var(self) -> &'static str {
        match self {
            Bin::Tri => "TRIOS_TRI_BIN",
            Bin::Igla => "TRIOS_IGLA_BIN",
        }
    }

    pub fn default_path(self) -> &'static str {
        match self {
            Bin::Tri => "tri",
            Bin::Igla => "trios-igla",
        }
    }
}

/// MCP tool descriptor exposed via `tools/list`.
#[derive(Debug, Clone, Serialize)]
pub struct ToolDescriptor {
    pub name: &'static str,
    pub description: &'static str,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
    #[serde(skip_serializing)]
    pub bin: Bin,
}

/// Build the canonical 15-tool catalog.
///
/// Order is deterministic: `tri deploy*` (6), `tri train` (1),
/// `tri race*` (3), `trios-igla *` (5) = 15.
pub fn catalog() -> Vec<ToolDescriptor> {
    vec![
        // --- tri deploy ---
        ToolDescriptor {
            name: "tri_deploy_init",
            description: "Create the Railway project `trios-trainer` (first-time setup). \
                Wraps `tri deploy init`.",
            input_schema: empty_object(),
            bin: Bin::Tri,
        },
        ToolDescriptor {
            name: "tri_deploy_seed",
            description: "Deploy a single seed container to Railway. \
                Wraps `tri deploy seed --seed <N> [--steps --hidden --lr --attn-layers]`.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "seed":        { "type": "integer", "description": "Seed value (e.g. 43)" },
                    "steps":       { "type": "integer", "description": "Training steps (default 27000)" },
                    "hidden":      { "type": "integer", "description": "Hidden dim (default 384)" },
                    "lr":          { "type": "number",  "description": "Learning rate (INV-8: in [0.001, 0.01])" },
                    "attn_layers": { "type": "integer", "description": "Attention layers (default 2)" }
                },
                "required": ["seed"],
                "additionalProperties": false
            }),
            bin: Bin::Tri,
        },
        ToolDescriptor {
            name: "tri_deploy_all",
            description: "Deploy all Gate-2 seeds (42, 43, 44). Wraps `tri deploy all`.",
            input_schema: empty_object(),
            bin: Bin::Tri,
        },
        ToolDescriptor {
            name: "tri_deploy_status",
            description: "List deployed Railway services. Wraps `tri deploy status`.",
            input_schema: empty_object(),
            bin: Bin::Tri,
        },
        ToolDescriptor {
            name: "tri_deploy_logs",
            description: "Stream logs for a seed's container. \
                Wraps `tri deploy logs --seed <N>`.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "seed": { "type": "integer", "description": "Seed value" }
                },
                "required": ["seed"],
                "additionalProperties": false
            }),
            bin: Bin::Tri,
        },
        ToolDescriptor {
            name: "tri_deploy_remove",
            description: "Remove a seed's container. Wraps `tri deploy remove --seed <N>`.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "seed": { "type": "integer", "description": "Seed value" }
                },
                "required": ["seed"],
                "additionalProperties": false
            }),
            bin: Bin::Tri,
        },
        // --- tri train ---
        ToolDescriptor {
            name: "tri_train",
            description: "Local training (no Railway). \
                Wraps `tri train --seed <N> --steps <N> [--hidden --lr --attn-layers --optimizer --eval-every]`.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "seed":        { "type": "integer" },
                    "steps":       { "type": "integer" },
                    "hidden":      { "type": "integer" },
                    "lr":          { "type": "number"  },
                    "attn_layers": { "type": "integer" },
                    "optimizer":   { "type": "string", "enum": ["adamw", "muon", "muon-cwd"] },
                    "eval_every":  { "type": "integer" }
                },
                "required": ["seed", "steps"],
                "additionalProperties": false
            }),
            bin: Bin::Tri,
        },
        // --- tri race ---
        ToolDescriptor {
            name: "tri_race_start",
            description: "Start the ASHA worker loop. Wraps `tri race start`.",
            input_schema: empty_object(),
            bin: Bin::Tri,
        },
        ToolDescriptor {
            name: "tri_race_status",
            description: "Show leaderboard (needs DATABASE_URL). Wraps `tri race status`.",
            input_schema: empty_object(),
            bin: Bin::Tri,
        },
        ToolDescriptor {
            name: "tri_race_best",
            description: "Show best trial (needs DATABASE_URL). Wraps `tri race best`.",
            input_schema: empty_object(),
            bin: Bin::Tri,
        },
        // --- trios-igla ---
        ToolDescriptor {
            name: "igla_search",
            description: "Filter the IGLA RACE ledger; one R7 triplet per match. \
                Wraps `trios-igla search [--seed --bpb-max --step-min --sha --gate-status]`. \
                Exit 0 = at least one hit, exit 2 = no match.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "seed":        { "type": "integer" },
                    "bpb_max":     { "type": "number"  },
                    "step_min":    { "type": "integer" },
                    "sha":         { "type": "string"  },
                    "gate_status": { "type": "string"  },
                    "ledger":      { "type": "string", "description": "Override ledger path" },
                    "embargo":     { "type": "string", "description": "Override embargo path" }
                },
                "additionalProperties": false
            }),
            bin: Bin::Igla,
        },
        ToolDescriptor {
            name: "igla_list",
            description: "Last N rows in canonical R7 triplet form (default 10). \
                Wraps `trios-igla list [--last N]`.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "last":    { "type": "integer", "description": "Row count (default 10)" },
                    "ledger":  { "type": "string"  },
                    "embargo": { "type": "string"  }
                },
                "additionalProperties": false
            }),
            bin: Bin::Igla,
        },
        ToolDescriptor {
            name: "igla_gate",
            description: "Gate-2 quorum verdict. PASS iff >=3 distinct seeds satisfy `bpb < target` AND `step >= 4000`. \
                Wraps `trios-igla gate [--target 1.85]`. Exit 0 = PASS, 2 = NOT YET.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target":  { "type": "number", "description": "BPB target (default 1.85)" },
                    "ledger":  { "type": "string" },
                    "embargo": { "type": "string" }
                },
                "additionalProperties": false
            }),
            bin: Bin::Igla,
        },
        ToolDescriptor {
            name: "igla_check",
            description: "R9 embargo refusal against `assertions/embargo.txt`. \
                Wraps `trios-igla check <sha>`. Exit 0 = clean, 1 = embargoed.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "sha":     { "type": "string", "description": "Commit SHA (full or 7-char)" },
                    "embargo": { "type": "string" }
                },
                "required": ["sha"],
                "additionalProperties": false
            }),
            bin: Bin::Igla,
        },
        ToolDescriptor {
            name: "igla_triplet",
            description: "Canonical R7 triplet for a row index (0-based). \
                Wraps `trios-igla triplet <index>`.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "index":   { "type": "integer", "minimum": 0 },
                    "ledger":  { "type": "string" },
                    "embargo": { "type": "string" }
                },
                "required": ["index"],
                "additionalProperties": false
            }),
            bin: Bin::Igla,
        },
    ]
}

fn empty_object() -> Value {
    json!({ "type": "object", "properties": {}, "additionalProperties": false })
}

/// Convert a tool name + JSON arguments to the CLI argv (excluding the
/// binary itself).
///
/// R5: any unrecognised argument key returns an error; we never silently
/// drop user intent.
pub fn build_args(tool: &str, args: &Value) -> anyhow::Result<Vec<String>> {
    let obj = args.as_object();

    macro_rules! get_int {
        ($key:literal) => {
            obj.and_then(|o| o.get($key)).and_then(|v| v.as_i64())
        };
    }
    macro_rules! get_num {
        ($key:literal) => {
            obj.and_then(|o| o.get($key)).and_then(|v| v.as_f64())
        };
    }
    macro_rules! get_str {
        ($key:literal) => {
            obj.and_then(|o| o.get($key)).and_then(|v| v.as_str())
        };
    }

    let mut a: Vec<String> = Vec::new();

    match tool {
        "tri_deploy_init" => {
            a.extend(["deploy", "init"].map(str::to_string));
        }
        "tri_deploy_all" => {
            a.extend(["deploy", "all"].map(str::to_string));
        }
        "tri_deploy_status" => {
            a.extend(["deploy", "status"].map(str::to_string));
        }
        "tri_race_start" => {
            a.extend(["race", "start"].map(str::to_string));
        }
        "tri_race_status" => {
            a.extend(["race", "status"].map(str::to_string));
        }
        "tri_race_best" => {
            a.extend(["race", "best"].map(str::to_string));
        }

        "tri_deploy_seed" => {
            a.extend(["deploy", "seed"].map(str::to_string));
            let seed = get_int!("seed").ok_or_else(|| anyhow::anyhow!("`seed` is required"))?;
            a.push("--seed".into());
            a.push(seed.to_string());
            if let Some(v) = get_int!("steps") {
                a.push("--steps".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_int!("hidden") {
                a.push("--hidden".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_num!("lr") {
                a.push("--lr".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_int!("attn_layers") {
                a.push("--attn-layers".into());
                a.push(v.to_string());
            }
        }
        "tri_deploy_logs" => {
            a.extend(["deploy", "logs"].map(str::to_string));
            let seed = get_int!("seed").ok_or_else(|| anyhow::anyhow!("`seed` is required"))?;
            a.push("--seed".into());
            a.push(seed.to_string());
        }
        "tri_deploy_remove" => {
            a.extend(["deploy", "remove"].map(str::to_string));
            let seed = get_int!("seed").ok_or_else(|| anyhow::anyhow!("`seed` is required"))?;
            a.push("--seed".into());
            a.push(seed.to_string());
        }

        "tri_train" => {
            a.push("train".into());
            let seed = get_int!("seed").ok_or_else(|| anyhow::anyhow!("`seed` is required"))?;
            let steps = get_int!("steps").ok_or_else(|| anyhow::anyhow!("`steps` is required"))?;
            a.push("--seed".into());
            a.push(seed.to_string());
            a.push("--steps".into());
            a.push(steps.to_string());
            if let Some(v) = get_int!("hidden") {
                a.push("--hidden".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_num!("lr") {
                a.push("--lr".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_int!("attn_layers") {
                a.push("--attn-layers".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_str!("optimizer") {
                a.push("--optimizer".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_int!("eval_every") {
                a.push("--eval-every".into());
                a.push(v.to_string());
            }
        }

        // --- trios-igla ---
        "igla_search" => {
            a.push("search".into());
            if let Some(v) = get_int!("seed") {
                a.push("--seed".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_num!("bpb_max") {
                a.push("--bpb-max".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_int!("step_min") {
                a.push("--step-min".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_str!("sha") {
                a.push("--sha".into());
                a.push(v.to_string());
            }
            if let Some(v) = get_str!("gate_status") {
                a.push("--gate-status".into());
                a.push(v.to_string());
            }
            push_common_igla(&mut a, obj);
        }
        "igla_list" => {
            a.push("list".into());
            if let Some(v) = get_int!("last") {
                a.push("--last".into());
                a.push(v.to_string());
            }
            push_common_igla(&mut a, obj);
        }
        "igla_gate" => {
            a.push("gate".into());
            if let Some(v) = get_num!("target") {
                a.push("--target".into());
                a.push(v.to_string());
            }
            push_common_igla(&mut a, obj);
        }
        "igla_check" => {
            a.push("check".into());
            push_common_igla(&mut a, obj);
            let sha = get_str!("sha").ok_or_else(|| anyhow::anyhow!("`sha` is required"))?;
            a.push(sha.to_string());
        }
        "igla_triplet" => {
            a.push("triplet".into());
            push_common_igla(&mut a, obj);
            let idx = get_int!("index").ok_or_else(|| anyhow::anyhow!("`index` is required"))?;
            if idx < 0 {
                anyhow::bail!("`index` must be >= 0");
            }
            a.push(idx.to_string());
        }

        other => anyhow::bail!("unknown tool: {}", other),
    }

    Ok(a)
}

fn push_common_igla(a: &mut Vec<String>, obj: Option<&serde_json::Map<String, Value>>) {
    if let Some(o) = obj {
        if let Some(v) = o.get("ledger").and_then(|v| v.as_str()) {
            a.push("--ledger".into());
            a.push(v.to_string());
        }
        if let Some(v) = o.get("embargo").and_then(|v| v.as_str()) {
            a.push("--embargo".into());
            a.push(v.to_string());
        }
    }
}

/// Result of running a wrapped CLI subcommand, ready to embed in the
/// MCP `tools/call` response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliOutcome {
    pub bin: String,
    pub argv: Vec<String>,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_15_tools() {
        let c = catalog();
        assert_eq!(c.len(), 15, "catalog must register exactly 15 tools");
    }

    #[test]
    fn catalog_names_unique() {
        let c = catalog();
        let mut names: Vec<&str> = c.iter().map(|t| t.name).collect();
        names.sort();
        let before = names.len();
        names.dedup();
        assert_eq!(before, names.len(), "tool names must be unique");
    }

    #[test]
    fn build_args_tri_deploy_seed_minimal() {
        let v = json!({ "seed": 43 });
        let a = build_args("tri_deploy_seed", &v).unwrap();
        assert_eq!(a, vec!["deploy", "seed", "--seed", "43"]);
    }

    #[test]
    fn build_args_tri_deploy_seed_full() {
        let v = json!({
            "seed": 43, "steps": 27000, "hidden": 384, "lr": 0.004, "attn_layers": 2
        });
        let a = build_args("tri_deploy_seed", &v).unwrap();
        assert_eq!(
            a,
            vec![
                "deploy",
                "seed",
                "--seed",
                "43",
                "--steps",
                "27000",
                "--hidden",
                "384",
                "--lr",
                "0.004",
                "--attn-layers",
                "2"
            ]
        );
    }

    #[test]
    fn build_args_tri_train_requires_seed_and_steps() {
        assert!(build_args("tri_train", &json!({ "seed": 43 })).is_err());
        assert!(build_args("tri_train", &json!({ "steps": 27000 })).is_err());
        let a = build_args("tri_train", &json!({ "seed": 43, "steps": 100 })).unwrap();
        assert_eq!(a, vec!["train", "--seed", "43", "--steps", "100"]);
    }

    #[test]
    fn build_args_igla_search_no_filters() {
        let a = build_args("igla_search", &json!({})).unwrap();
        assert_eq!(a, vec!["search"]);
    }

    #[test]
    fn build_args_igla_search_full() {
        let v = json!({
            "seed": 43, "bpb_max": 1.85, "step_min": 4000,
            "sha": "2446855", "gate_status": "pass"
        });
        let a = build_args("igla_search", &v).unwrap();
        assert_eq!(
            a,
            vec![
                "search",
                "--seed",
                "43",
                "--bpb-max",
                "1.85",
                "--step-min",
                "4000",
                "--sha",
                "2446855",
                "--gate-status",
                "pass"
            ]
        );
    }

    #[test]
    fn build_args_igla_check_requires_sha() {
        assert!(build_args("igla_check", &json!({})).is_err());
        let a = build_args("igla_check", &json!({ "sha": "477e3377" })).unwrap();
        assert_eq!(a, vec!["check", "477e3377"]);
    }

    #[test]
    fn build_args_igla_triplet_rejects_negative_index() {
        assert!(build_args("igla_triplet", &json!({ "index": -1 })).is_err());
        let a = build_args("igla_triplet", &json!({ "index": 0 })).unwrap();
        assert_eq!(a, vec!["triplet", "0"]);
    }

    #[test]
    fn build_args_unknown_tool() {
        assert!(build_args("nope", &json!({})).is_err());
    }
}
