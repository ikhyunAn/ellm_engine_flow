use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use server::Server;
use std::collections::BTreeMap;
use std::fs;
use std::net::SocketAddr;
use tokenizers::{self, Tokenizer};

extern crate log4rs;
use log::{debug, error, info};

// use crate::data_channel::*;

mod data_channel;
mod prompting;
mod server;
mod summarize;
mod flow;

// 서버 포트
const SERVER_PORT: u16 = 8084;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PromptTemplate {
    no_background_sys: String,
    background_sys: String,
    choice: String,
    doc_summary1: String,
    doc_summary2: String,
    this_chat_summary: String,
    this_document_summary: String,
    translate: String,
    api: String,
    explain: String,
    etc: String,
    rationale: String,
    custom_prompt: String,
    image: String,
    code: String,
    insight1: String,
    insight2: String,
}
impl PromptTemplate {
    fn get_field_value(&self, field_name: &str) -> Option<&str> {
        match field_name {
            "no_background_sys" => Some(&self.no_background_sys),
            "background_sys" => Some(&self.background_sys),
            "choice" => Some(&self.choice),
            "doc_summary1" => Some(&self.doc_summary1),
            "doc_summary2" => Some(&self.doc_summary2),
            "this_chat_summary" => Some(&self.this_chat_summary),
            "this_document_summary" => Some(&self.this_document_summary),
            "translate" => Some(&self.translate),
            "api" => Some(&self.api),
            "explain" => Some(&self.explain),
            "etc" => Some(&self.etc),
            "rationale" => Some(&self.rationale),
            "custom_prompt" => Some(&self.custom_prompt),
            "image" => Some(&self.image),
            "code" => Some(&self.code),
            "insight1" => Some(&self.insight1),
            "insight2" => Some(&self.insight2),
            _ => None,
        }
    }
}

// #[derive(Debug, Deserialize, Serialize)]
// struct DocSummaryRequestJsonData {
//     session_id: String,
//     dialog_type: String,
//     task: String,
//     document: String,
// }

fn main() -> Result<(), Error> {
    log4rs::init_file("log4rs.yml", Default::default()).expect("failed to init log4rs.yml");

    // let prompt_template_file_path = "prompt_template.txt";
    // let prompt_template_data = fs::read_to_string(prompt_template_file_path)?;
    // let prompt_template: PromptTemplate = toml::from_str(&prompt_template_data)?;
    let addr: SocketAddr = format!("0.0.0.0:{}", SERVER_PORT)
        .parse()
        .expect("failed to parsing the server address");

    let tokenizer = Tokenizer::from_file("tokenizer.json")
        .expect("failed to get tokenizer from tokenizer.json");

    let mut server = Server::new(addr, tokenizer);
    info!("new Ellm agent server at {}", SERVER_PORT);

    let _ = server.run();

    Ok(())
}
