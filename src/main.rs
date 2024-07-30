use thiserror::Error;

use crate::debug_print::DEBUG;
use crate::statemachine::{ProcessPattern, ProcessPatternType};

pub mod statemachine;
pub mod debug_print;

fn main() {
    let _process_pattern1 = ProcessPattern::new(ProcessPatternType::BasicPrompt);
    let process_pattern2 = ProcessPattern::new(ProcessPatternType::MapReduce);

    let data = Data{
        prompted_input: "this is a prompt input".to_string(),
        llm_task: "code".to_string(),
        doc_chunk_data: Some(DocChunkData{
            doc_chunk_id: 0,
            chunk_len : 3,
            chunk_index: 0,
            chunk_text: Some("first chunk".to_string()),
        })
    };

    let mut sm_2 = process_pattern2.state_machine;
    
    loop {
        match sm_2.step(data.clone()) {
            Ok(process_result) => {
                match process_result {
                    ProcessResult::Incomplete => continue, // keep FlowManager
                    ProcessResult::Complete => break, // handle complete Flow: make null
                }
            }
            Err(e) => {
                // handle error
                println!("error!{}", e.to_string());
                break;
            }
        }
    }

    debug_print::debug_print(DEBUG, "state machine exited without error");
}



#[derive(Debug, Clone)]
pub struct Data {
    prompted_input: String,
    // app_agent_token: Token,
    // task: String,
    llm_task: String,
    // is_done: bool,
    // actor: String,
    // request_json_data: RequestJsonData,
    // prompt_keys: PromptKeys,
    doc_chunk_data: Option<DocChunkData>, // doc_summary 할때 문서 청크
    // chunk_btreemap: Option<BTreeMap<char, Chunk>>,
    // full_text: String,
}

#[derive(Debug, Clone)]
struct DocChunkData {
    doc_chunk_id: u64,
    chunk_len: usize,
    chunk_index: usize,
    chunk_text: Option<String>,
}


pub fn vllm_connect() {

}

pub fn send_to_vllm(_data: Data) {

}

pub fn respond_to_client(_data: Data) {

}

#[derive(Clone)]
pub enum ProcessResult {
    Incomplete,
    Complete,
}

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("Invalid task type: {0}")]
    InvalidTaskType(String),
    #[error("Task type not set")]
    TaskTypeNotSet,
    #[error("State machine error: {0}")]
    StateMachineError(#[from] Box<dyn std::error::Error>),
}

// pub static DEBUG: i8 = 1;