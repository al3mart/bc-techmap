use crate::data::ecosystem::Ecosystem;

#[derive(Clone, PartialEq)]
pub struct DimensionScore {
    pub name: String,
    pub score: f64,
    pub label: String,
}

#[derive(Clone, PartialEq)]
pub struct MigrationReport {
    pub overall: f64,
    pub difficulty_label: String,
    pub dimensions: Vec<DimensionScore>,
    pub challenges: Vec<String>,
    pub positives: Vec<String>,
}

const W_LANGUAGE: f64 = 0.35;
const W_VM: f64 = 0.12;
const W_TX_MODEL: f64 = 0.08;
const W_EVM_COMPAT: f64 = 0.08;
const W_DEPLOY: f64 = 0.07;
const W_TOOLING: f64 = 0.10;
const W_DOCS: f64 = 0.07;
const W_L2: f64 = 0.07;
const W_FUNDING: f64 = 0.06;

/// Get the effective languages for an ecosystem given an optional deploy mode.
fn effective_languages<'a>(eco: &'a Ecosystem, mode: Option<&str>) -> &'a [String] {
    if let (Some(mode_name), Some(modes)) = (mode, &eco.deploy_modes) {
        if let Some(dm) = modes.get(mode_name) {
            return &dm.languages;
        }
    }
    &eco.languages
}

/// Map a language string to its family for fuzzy matching.
fn lang_family(lang: &str) -> &str {
    match lang {
        "Solidity" | "Vyper" | "Huff" | "Cairo" => "evm-adjacent",
        "Rust" => "rust",
        "Go" | "Golang" => "go",
        "C++" | "C" => "c-family",
        "TypeScript" | "JavaScript" | "AssemblyScript" => "js-family",
        "Move" | "Sui Move" | "Aptos Move" => "move",
        "Tolk" | "Tact" | "FunC" | "FunC (legacy)" => "ton-native",
        "Aiken" | "Plinth" | "OpShin" => "cardano-native",
        "Compact" => "zk-native",
        _ => "other",
    }
}

fn language_distance(
    src: &Ecosystem,
    dst: &Ecosystem,
    src_mode: Option<&str>,
    dst_mode: Option<&str>,
) -> f64 {
    let src_langs = effective_languages(src, src_mode);
    let dst_langs = effective_languages(dst, dst_mode);

    let shared = src_langs
        .iter()
        .any(|l| dst_langs.iter().any(|d| d == l));
    if shared {
        return 0.0;
    }

    let src_families: Vec<&str> = src_langs.iter().map(|l| lang_family(l)).collect();
    let dst_families: Vec<&str> = dst_langs.iter().map(|l| lang_family(l)).collect();

    let shared_family = src_families.iter().any(|f| dst_families.contains(f));
    if shared_family {
        0.3
    } else {
        1.0
    }
}

/// Categorize a VM string into a compatibility group.
fn vm_group(vm: &str) -> &str {
    match vm {
        "EVM" | "EVM / Subnet-EVM" | "zkEVM" => "evm",
        "PolkaVM/EVM" => "evm-plus-pvm",
        "CosmWasm" | "NearVM" | "Soroban (Wasmi)" => "wasm",
        "SVM (sBPF)" => "svm",
        "Sui MoveVM" | "MoveVM + Block-STM" => "move",
        "TVM" => "tvm",
        "CairoVM (STARK)" => "cairo",
        "Plutus VM (UPLC)" => "plutus",
        "ZK Circuit VM" => "zk-circuit",
        "N/A (DA layer)" => "da-layer",
        "XRPL Native" => "xrpl-native",
        _ => "other",
    }
}

fn vm_distance(src: &Ecosystem, dst: &Ecosystem) -> f64 {
    if src.vm == dst.vm {
        return 0.0;
    }

    let sg = vm_group(&src.vm);
    let dg = vm_group(&dst.vm);

    if sg == dg {
        return 0.2;
    }

    // PolkaVM/EVM overlaps with both EVM and Wasm groups
    if (sg == "evm-plus-pvm" && (dg == "evm" || dg == "wasm"))
        || (dg == "evm-plus-pvm" && (sg == "evm" || sg == "wasm"))
    {
        return 0.3;
    }

    // DA layer has no VM — always a big gap
    if sg == "da-layer" || dg == "da-layer" {
        return 1.0;
    }

    // XRPL has no general-purpose VM — always a big gap
    if sg == "xrpl-native" || dg == "xrpl-native" {
        return 1.0;
    }

    1.0
}

fn transaction_model_distance(src: &Ecosystem, dst: &Ecosystem) -> f64 {
    if src.transaction_model == dst.transaction_model {
        return 0.0;
    }

    let is_account_family =
        |m: &str| matches!(m, "account" | "account-resource");

    // Account variants are close
    if is_account_family(&src.transaction_model) && is_account_family(&dst.transaction_model) {
        return 0.15;
    }

    // eUTXO ↔ account is a moderate paradigm shift
    if (is_account_family(&src.transaction_model) && dst.transaction_model == "eUTXO")
        || (src.transaction_model == "eUTXO" && is_account_family(&dst.transaction_model))
    {
        return 0.6;
    }

    // eUTXO variants are close
    if src.transaction_model == "eUTXO" && dst.transaction_model == "eUTXO" {
        return 0.0;
    }

    // Object-centric ↔ account is moderate
    if (is_account_family(&src.transaction_model) && dst.transaction_model == "object-centric")
        || (src.transaction_model == "object-centric" && is_account_family(&dst.transaction_model))
    {
        return 0.5;
    }

    // Actor ↔ anything else is significant
    if src.transaction_model == "actor" || dst.transaction_model == "actor" {
        return 0.8;
    }

    0.7
}

/// EVM compatibility distance: moving between EVM-native chains is trivial,
/// moving from EVM to non-EVM (or vice versa) is a penalty.
fn evm_compat_distance(src: &Ecosystem, dst: &Ecosystem) -> f64 {
    let rank = |c: &str| -> u8 {
        match c {
            "native" => 0,
            "supported" => 1,
            _ => 2, // "none"
        }
    };

    let sr = rank(&src.evm_compatibility);
    let dr = rank(&dst.evm_compatibility);

    if sr == dr {
        return 0.0;
    }

    // native ↔ supported is a small gap
    if sr <= 1 && dr <= 1 {
        return 0.2;
    }

    // EVM world ↔ non-EVM world
    if (sr <= 1 && dr == 2) || (sr == 2 && dr <= 1) {
        return 1.0;
    }

    0.5
}

/// Deployment model distance: measures how different the "what you ship" is.
fn deploy_model_distance(src: &Ecosystem, dst: &Ecosystem) -> f64 {
    let src_set: Vec<&str> = src.deployment_options.iter().map(|s| s.as_str()).collect();
    let dst_set: Vec<&str> = dst.deployment_options.iter().map(|s| s.as_str()).collect();

    // Any shared deployment option = familiar territory
    let shared = src_set.iter().any(|s| dst_set.contains(s));
    if shared {
        return 0.0;
    }

    let has = |set: &[&str], opt: &str| set.contains(&opt);

    // Sidechains are penalized heavily: separate validator set, bridge
    // trust assumptions, not a native solution. Worse than appchain/rollup
    // transitions (0.7) since L2s and appchains inherit or extend L1 security.
    if (has(&src_set, "sidechain") && has(&dst_set, "contract"))
        || (has(&src_set, "contract") && has(&dst_set, "sidechain"))
    {
        return 0.8;
    }

    if has(&src_set, "sidechain") || has(&dst_set, "sidechain") {
        return 0.9;
    }

    // contract-only ↔ appchain-only is a big shift
    // contract ↔ rollup is moderate
    if (has(&src_set, "contract") && has(&dst_set, "appchain") && !has(&dst_set, "contract"))
        || (has(&src_set, "appchain") && has(&dst_set, "contract") && !has(&dst_set, "appchain"))
    {
        return 0.7;
    }

    0.5
}

fn dest_tooling_difficulty(dst: &Ecosystem) -> f64 {
    1.0 - (dst.tooling_maturity as f64 - 1.0) / 4.0
}

fn dest_docs_difficulty(dst: &Ecosystem) -> f64 {
    1.0 - (dst.doc_quality as f64 - 1.0) / 4.0
}

fn dest_funding_difficulty(dst: &Ecosystem) -> f64 {
    1.0 - (dst.ecosystem_funding as f64 - 1.0) / 4.0
}

fn l2_gap(src: &Ecosystem, dst: &Ecosystem) -> f64 {
    let diff = (src.l2_maturity as f64 - dst.l2_maturity as f64).abs();
    diff / 4.0
}

fn score_label(s: f64) -> String {
    match s {
        x if x < 0.2 => "Trivial".to_string(),
        x if x < 0.4 => "Easy".to_string(),
        x if x < 0.6 => "Moderate".to_string(),
        x if x < 0.8 => "Hard".to_string(),
        _ => "Extreme".to_string(),
    }
}

fn dim_label(s: f64) -> String {
    match s {
        x if x < 0.15 => "Same".to_string(),
        x if x < 0.35 => "Similar".to_string(),
        x if x < 0.65 => "Different".to_string(),
        _ => "Very different".to_string(),
    }
}

fn funding_label(s: f64) -> String {
    match s {
        x if x < 0.15 => "Strong opportunities".to_string(),
        x if x < 0.35 => "Good opportunities".to_string(),
        x if x < 0.65 => "Moderate opportunities".to_string(),
        _ => "Limited opportunities".to_string(),
    }
}

pub fn compute_migration(
    src: &Ecosystem,
    dst: &Ecosystem,
    src_mode: Option<&str>,
    dst_mode: Option<&str>,
) -> MigrationReport {
    let lang = language_distance(src, dst, src_mode, dst_mode);
    let vm = vm_distance(src, dst);
    let state = transaction_model_distance(src, dst);
    let evm = evm_compat_distance(src, dst);
    let deploy = deploy_model_distance(src, dst);
    let tooling = dest_tooling_difficulty(dst);
    let docs = dest_docs_difficulty(dst);
    let l2 = l2_gap(src, dst);
    let funding = dest_funding_difficulty(dst);

    let overall = lang * W_LANGUAGE
        + vm * W_VM
        + state * W_TX_MODEL
        + evm * W_EVM_COMPAT
        + deploy * W_DEPLOY
        + tooling * W_TOOLING
        + docs * W_DOCS
        + l2 * W_L2
        + funding * W_FUNDING;

    let dimensions = vec![
        DimensionScore {
            name: "Language".to_string(),
            score: lang,
            label: dim_label(lang),
        },
        DimensionScore {
            name: "VM / Runtime".to_string(),
            score: vm,
            label: dim_label(vm),
        },
        DimensionScore {
            name: "Tx Model".to_string(),
            score: state,
            label: dim_label(state),
        },
        DimensionScore {
            name: "EVM Compat".to_string(),
            score: evm,
            label: dim_label(evm),
        },
        DimensionScore {
            name: "Deploy Model".to_string(),
            score: deploy,
            label: dim_label(deploy),
        },
        DimensionScore {
            name: "Dest. Tooling".to_string(),
            score: tooling,
            label: dim_label(tooling),
        },
        DimensionScore {
            name: "Dest. Docs".to_string(),
            score: docs,
            label: dim_label(docs),
        },
        DimensionScore {
            name: "L2 Gap".to_string(),
            score: l2,
            label: dim_label(l2),
        },
        DimensionScore {
            name: "Ecosystem Funding".to_string(),
            score: funding,
            label: funding_label(funding),
        },
    ];

    let src_langs = effective_languages(src, src_mode);
    let dst_langs = effective_languages(dst, dst_mode);

    // ── Positives ──
    let mut positives = Vec::new();

    if lang == 0.0 {
        positives.push(format!(
            "Same language ({}) — existing code may port directly",
            src_langs.join(", ")
        ));
    } else if lang <= 0.3 {
        positives.push("Related language family — developer skills transfer well".to_string());
    }

    if vm == 0.0 {
        positives.push(format!(
            "Same VM ({}) — runtime behavior is identical",
            src.vm
        ));
    }

    if evm == 0.0 && src.evm_compatibility == "native" {
        positives.push(
            "Both EVM-native — tooling, libraries, and patterns transfer directly".to_string(),
        );
    }

    if state == 0.0 {
        positives.push("Same transaction model — no paradigm shift required".to_string());
    }

    if deploy == 0.0 {
        positives.push("Same deployment model — no infrastructure changes needed".to_string());
    }

    if tooling <= 0.25 {
        positives.push(format!(
            "Excellent destination tooling ({}/5)",
            dst.tooling_maturity
        ));
    }

    if docs <= 0.25 {
        positives.push(format!(
            "Strong destination documentation ({}/5)",
            dst.doc_quality
        ));
    }

    if dst.ecosystem_funding >= 4 {
        positives.push(format!(
            "Well-funded destination ecosystem ({}/5) — grants and support available",
            dst.ecosystem_funding
        ));
    }

    // ── Challenges ──
    let mut challenges = Vec::new();

    if lang >= 0.8 {
        challenges.push(format!(
            "Completely different languages: {} → {}",
            src_langs.join(", "),
            dst_langs.join(", ")
        ));
    } else if lang >= 0.3 {
        challenges.push(format!(
            "Related but distinct languages: {} → {}",
            src_langs.join(", "),
            dst_langs.join(", ")
        ));
    }

    if vm >= 0.8 {
        challenges.push(format!(
            "Different VM architecture: {} → {}",
            src.vm, dst.vm
        ));
    }

    if state >= 0.5 {
        challenges.push(format!(
            "Different transaction model: {} → {}",
            src.transaction_model, dst.transaction_model
        ));
    }

    if evm >= 0.8 {
        if src.evm_compatibility == "native" || src.evm_compatibility == "supported" {
            challenges
                .push("Leaving the EVM ecosystem — existing tooling won't transfer".to_string());
        } else {
            challenges.push(
                "Entering the EVM ecosystem — different paradigm from source".to_string(),
            );
        }
    }

    if deploy >= 0.5 {
        challenges.push(format!(
            "Different deployment model: {} → {}",
            src.deployment_options.join("/"),
            dst.deployment_options.join("/")
        ));
    }

    if tooling >= 0.6 {
        challenges.push(format!(
            "Destination tooling is immature ({}/5)",
            dst.tooling_maturity
        ));
    }

    if docs >= 0.6 {
        challenges.push(format!(
            "Destination documentation is limited ({}/5)",
            dst.doc_quality
        ));
    }

    if l2 >= 0.5 {
        challenges.push("Significant L2/rollup ecosystem gap".to_string());
    }

    if dst.ecosystem_funding <= 2 {
        challenges.push(format!(
            "Limited ecosystem funding ({}/5) — fewer grants and support programs",
            dst.ecosystem_funding
        ));
    }

    MigrationReport {
        overall,
        difficulty_label: score_label(overall),
        dimensions,
        challenges,
        positives,
    }
}
