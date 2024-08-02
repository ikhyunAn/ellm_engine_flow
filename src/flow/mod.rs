use crate::Data;

pub mod basicprompt;
pub mod mapreduce;
pub mod multireduce;
pub mod refine;
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
    pattern_type: ProcessPatternType,
    pub state_machine: Box<dyn StateMachine>,
    prompt_template: Vec<String>,
}

impl ProcessPattern {
    pub fn new(pattern_type: ProcessPatternType, prompt_template: Vec<String>) -> Self {
        let state_machine: Box<dyn StateMachine> = match pattern_type {
            ProcessPatternType::BasicPrompt => Box::new(basicprompt::BasicPromptStateMachine::new()),
            ProcessPatternType::MapReduce => Box::new(mapreduce::MapReduceStateMachine::new()),
        };
        ProcessPattern { pattern_type, state_machine, prompt_template } // returns Process struct
    }
}

#[derive(Clone)]
pub enum ProcessResult {
    Incomplete,
    Complete,
}