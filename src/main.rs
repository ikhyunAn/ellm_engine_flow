use serde::{self, Serialize, Deserialize};
use thiserror::Error;

use crate::debug_print::DEBUG;

// import statements required to uset the flow module
use crate::flow::{ProcessPattern, ProcessPatternType, ProcessResult};
pub mod flow;

pub mod debug_print;

fn main() {
    let mapreduce_template = vec!("
<map_instruction>
<chunk>
".to_string(), "
<reduce_instruction>
<map_result>
<task_instruction>
<user_query>
".to_string());

    let process_pattern = ProcessPattern::new(ProcessPatternType::MapReduce, mapreduce_template);
    // let process_pattern2 = ProcessPattern::new(ProcessPatternType::MapReduce);

    let data = Data {
        app_agent_token: "your_app_agent_token_here".to_string(),
        prompt_keys: PromptKeys {
            user_query: Some("What are the main features of Rust?".to_string()),
            task_instruction: Some("Summarize the key features of the Rust programming language".to_string()),
            system_query: None,
            fasoo: Some(Fasoo {
                session_id: "session_123456".to_string(),
                task: "summarize".to_string(),
                pattern: Some(Pattern {
                    name: "map_reduce".to_string(),
                    map_reduce: Some(MapReduce {
                        map_instruction: "Extract key features of Rust from each chunk".to_string(),
                        reduce_instruction: "Combine and summarize the extracted features".to_string(),
                    }),
                }),
                task_instruction: Some("Provide a concise summary of Rust's main features".to_string()),
                chunks: Some(vec![
                    Chunk {
                        doc_id: "rust_doc_001".to_string(),
                        chunk_idx: 1,
                        chunk_text: "Rust is known for its memory safety without garbage collection.".to_string(),
                        similarity: 0.85,
                        filename: "rust_overview.txt".to_string(),
                        title: "Rust Programming Language Overview".to_string(),
                        author: "Rust Documentation Team".to_string(),
                        page: 1,
                        time_stamp: 1628097600, // Unix timestamp
                        summary: "Introduces Rust's memory safety feature".to_string(),
                        score: 0.9,
                    },
                    Chunk {
                        doc_id: "rust_doc_002".to_string(),
                        chunk_idx: 2,
                        chunk_text: "Rust offers zero-cost abstractions, making it efficient for systems programming.".to_string(),
                        similarity: 0.78,
                        filename: "rust_performance.txt".to_string(),
                        title: "Rust Performance Characteristics".to_string(),
                        author: "Rust Performance Team".to_string(),
                        page: 3,
                        time_stamp: 1628184000, // Unix timestamp
                        summary: "Highlights Rust's efficiency in systems programming".to_string(),
                        score: 0.85,
                    },
                    Chunk {
                        doc_id: "rust_doc_003".to_string(),
                        chunk_idx: 3,
                        chunk_text: "Rust's ownership system ensures thread safety and prevents data races at compile time.".to_string(),
                        similarity: 0.92,
                        filename: "rust_concurrency.txt".to_string(),
                        title: "Concurrency in Rust".to_string(),
                        author: "Rust Concurrency Expert".to_string(),
                        page: 7,
                        time_stamp: 1628270400, // Unix timestamp
                        summary: "Explains Rust's approach to safe concurrency".to_string(),
                        score: 0.95,
                    },
                ]),
            }),
        },
        prompt_exchange: None,
    };
    
    // Data{
    //     prompted_input: "this is a prompt input".to_string(),
    //     // llm_task: "code".to_string(),
    //     instructions: vec![String::from("map instruction"), String::from("reduce instruction")],
    //     chunks: vec![DocChunkData {
    //         doc_id: 0,
    //         chunk_idx: 0,
    //         chunk_text: String::from("this is a text"),
    //     }
    // ],
    //     prompt_exchange: PromptExchange {
    //         index: 0,
    //         prompted_string: None,
    //         llm_response: None,
    //     },
    //     task_instruction: String::from("task_instruction"),
    //     input: String::from("input"),
    // };

    let mut sm = process_pattern.state_machine;
    
    match sm.step(data.clone()) {
        Ok(process_result) => {
            match process_result {
                ProcessResult::Incomplete => {},//continue, // keep FlowManager
                ProcessResult::Complete => {}, //break, // handle complete Flow: make null
            }
        }
        Err(e) => {
            // handle error
            println!("error!{}", e.to_string());
            // break;
        }
    }
    // loop {
    // }

    debug_print::debug_print(DEBUG, "state machine exited without error");
}



#[derive(Debug, Clone)]
pub struct Data {
    app_agent_token: String,
    prompt_keys: PromptKeys,
    pub prompt_exchange: Option<PromptExchange>,
}

#[derive(Debug, Clone)]
struct DocChunkData {
    doc_id: u64,
    chunk_idx: usize,
    chunk_text: String,
    // similarlity, score, filename, title, author, page, time_stamp, summary
}
#[derive(Debug, Clone)]
pub struct PromptKeys {
    pub user_query: Option<String>,
    pub task_instruction: Option<String>,
    pub system_query: Option<String>,
    pub fasoo: Option<Fasoo>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fasoo {
    session_id: String,
    pub task: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Pattern>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_instruction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunks: Option<Vec<Chunk>>,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct Chunk {
    doc_id: String,
    chunk_idx: u32,
    pub chunk_text: String,
    similarity: f64,
    filename: String,
    title: String,
    author: String,
    page: u32,
    time_stamp: u64,
    summary: String,
    pub score: f64,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pattern {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_reduce: Option<MapReduce>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // refine: Option<Refine>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // multi_reduce: Option<MultiReduce>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // flare: Option<Config>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct MapReduce {
    map_instruction: String,
    reduce_instruction: String,
}

pub fn send_to_vllm(_data: Data) {

}

// #[derive(Clone)]
// pub enum ProcessResult {
//     Incomplete,
//     Complete,
// }

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptExchange {
    pub index: usize,
    pub prompted_string: String,
    pub llm_response: Option<String>,
}
