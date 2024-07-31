use crate::{Data, ProcessResult};

pub mod basicprompt;
pub mod mapreduce;
// TODO: add more statemachines

pub trait NewStateMachine {
    fn new() -> Self;
}

pub trait StateMachine {
    fn step(&mut self, data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>>;
}

pub enum ProcessPatternType {
    BasicPrompt,
    MapReduce,
    // TODO: add more Process Patterns
}

pub struct ProcessPattern {
    pattern_type: ProcessPatternType,   // NOTE: may not be necessary
    pub state_machine: Box<dyn StateMachine>,
}

impl ProcessPattern {
    pub fn new(pattern_type: ProcessPatternType) -> Self {
        let state_machine: Box<dyn StateMachine> = match pattern_type {
            ProcessPatternType::BasicPrompt => Box::new(basicprompt::BasicPromptStateMachine::new()),
            ProcessPatternType::MapReduce => Box::new(mapreduce::MapReduceStateMachine::new()),
        };
        ProcessPattern { pattern_type, state_machine } // returns Process struct
    }
}