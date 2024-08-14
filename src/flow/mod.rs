use crate::server::{Data, PromptExchange, VLLMConnWithState, VLLMTokenWithData};

pub mod basicprompt;
pub mod mapreduce;
// pub mod multireduce;
// pub mod refine;
pub mod prompt_template;
// TODO: add more statemachines

pub trait NewStateMachine {
    fn new() -> Self;
}

pub trait StateMachine {
    fn step(
        &mut self,
        data: Data,
        vllm_connection_pool: &mut Vec<Option<VLLMConnWithState>>,
        vllm_token_with_data_vec: &mut Vec<Option<VLLMTokenWithData>>,
    ) -> Result<ProcessResult, Box<dyn std::error::Error>>;
}

pub enum ProcessPatternType {
    BasicPrompt,
    MapReduce,
    // TODO: add more Process Patterns
}

pub struct ProcessPattern {
    pattern_type: ProcessPatternType,
    pub state_machine: Box<dyn StateMachine>,
}

impl ProcessPattern {
    pub fn new(pattern_type: ProcessPatternType) -> Self {
        let state_machine: Box<dyn StateMachine> = match pattern_type {
            ProcessPatternType::BasicPrompt => {
                Box::new(basicprompt::BasicPromptStateMachine::new())
            }
            ProcessPatternType::MapReduce => Box::new(mapreduce::MapReduceStateMachine::new()),
        };
        ProcessPattern {
            pattern_type,
            state_machine,
        } // returns Process struct
    }
}

#[derive(Clone)]
pub enum ProcessResult {
    Incomplete,
    Complete,
}
