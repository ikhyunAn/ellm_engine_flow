use crate::{Data, ProcessResult};

pub mod basicprompt;
pub mod mapreduce;

pub trait NewStateMachine {
    fn new() -> Self;
}

pub trait StateMachine {
    fn drive(&mut self, data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>>;
}