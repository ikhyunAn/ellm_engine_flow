use crate::{send_to_vllm, Data};
use super::{NewStateMachine, ProcessResult, StateMachine};


/*
NOTE: state machine may not be necessary for BasicPrompt */

// #[derive(Clone)]
// enum State {
//     Start,
//     Done,
// }

#[derive(Clone)]
pub struct BasicPromptStateMachine {
    // state: State,
    _data: Option<Data>,
    // intermediate_results
    // output for client
}

impl NewStateMachine for BasicPromptStateMachine {
    fn new() -> Self {
        BasicPromptStateMachine {
            // state: State::Start,
            _data: None,
        }
    }
}

impl StateMachine for BasicPromptStateMachine {
    fn drive(&mut self, data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        // DONE: send 1 request
        send_to_vllm(data);
        Ok(ProcessResult::Complete)
    }
}

