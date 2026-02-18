use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct DeployMode {
    languages: Vec<String>,
}

#[derive(Deserialize)]
struct Ecosystem {
    id: String,
    name: String,
    short: String,
    languages: Vec<String>,
    vm: String,
    transaction_model: String,
    evm_compatibility: String,
    deployment_options: Vec<String>,
    chain_layer: String,
    l2_maturity: u8,
    consensus: String,
    tooling_maturity: u8,
    tooling: Vec<String>,
    doc_quality: u8,
    ecosystem_funding: u8,
    position: [f64; 2],
    deploy_modes: Option<HashMap<String, DeployMode>>,
}

#[derive(Deserialize)]
struct EcosystemFile {
    ecosystem: Vec<Ecosystem>,
}

fn quote(s: &str) -> String {
    format!("{:?}", s)
}

fn str_vec(v: &[String]) -> String {
    let items: Vec<String> = v.iter().map(|s| format!("{}.into()", quote(s))).collect();
    format!("vec![{}]", items.join(", "))
}

fn main() {
    println!("cargo:rerun-if-changed=data/ecosystems.toml");

    let raw = fs::read_to_string("data/ecosystems.toml").expect("read ecosystems.toml");
    let file: EcosystemFile = toml::from_str(&raw).expect("parse ecosystems.toml");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("ecosystems_generated.rs");

    let mut code = String::from("pub fn load_ecosystems() -> Vec<Ecosystem> {\n    vec![\n");

    for eco in &file.ecosystem {
        code.push_str("        Ecosystem {\n");
        code.push_str(&format!("            id: {}.into(),\n", quote(&eco.id)));
        code.push_str(&format!("            name: {}.into(),\n", quote(&eco.name)));
        code.push_str(&format!("            short: {}.into(),\n", quote(&eco.short)));
        code.push_str(&format!("            languages: {},\n", str_vec(&eco.languages)));
        code.push_str(&format!("            vm: {}.into(),\n", quote(&eco.vm)));
        code.push_str(&format!(
            "            transaction_model: {}.into(),\n",
            quote(&eco.transaction_model)
        ));
        code.push_str(&format!(
            "            evm_compatibility: {}.into(),\n",
            quote(&eco.evm_compatibility)
        ));
        code.push_str(&format!(
            "            deployment_options: {},\n",
            str_vec(&eco.deployment_options)
        ));
        code.push_str(&format!(
            "            chain_layer: {}.into(),\n",
            quote(&eco.chain_layer)
        ));
        code.push_str(&format!("            l2_maturity: {},\n", eco.l2_maturity));
        code.push_str(&format!(
            "            consensus: {}.into(),\n",
            quote(&eco.consensus)
        ));
        code.push_str(&format!(
            "            tooling_maturity: {},\n",
            eco.tooling_maturity
        ));
        code.push_str(&format!("            tooling: {},\n", str_vec(&eco.tooling)));
        code.push_str(&format!("            doc_quality: {},\n", eco.doc_quality));
        code.push_str(&format!(
            "            ecosystem_funding: {},\n",
            eco.ecosystem_funding
        ));
        code.push_str(&format!(
            "            position: [{:.1}, {:.1}],\n",
            eco.position[0], eco.position[1]
        ));

        match &eco.deploy_modes {
            None => code.push_str("            deploy_modes: None,\n"),
            Some(modes) => {
                code.push_str("            deploy_modes: Some(HashMap::from([\n");
                for (key, dm) in modes {
                    code.push_str(&format!(
                        "                ({}.into(), DeployMode {{ languages: {} }}),\n",
                        quote(key),
                        str_vec(&dm.languages)
                    ));
                }
                code.push_str("            ])),\n");
            }
        }

        code.push_str("        },\n");
    }

    code.push_str("    ]\n}\n");
    fs::write(&dest, code).expect("write generated file");
}
