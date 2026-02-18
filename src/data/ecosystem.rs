use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct DeployMode {
    pub languages: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub struct Ecosystem {
    pub id: String,
    pub name: String,
    pub short: String,
    pub languages: Vec<String>,
    pub vm: String,
    pub transaction_model: String,
    pub evm_compatibility: String,
    pub deployment_options: Vec<String>,
    pub chain_layer: String,
    pub l2_maturity: u8,
    pub consensus: String,
    pub tooling_maturity: u8,
    pub tooling: Vec<String>,
    pub doc_quality: u8,
    pub ecosystem_funding: u8,
    pub position: [f64; 2],
    pub deploy_modes: Option<HashMap<String, DeployMode>>,
}

include!(concat!(env!("OUT_DIR"), "/ecosystems_generated.rs"));
