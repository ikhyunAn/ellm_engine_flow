use crate::{send_to_vllm, Data};
use super::{NewStateMachine, ProcessResult, StateMachine};


/*
NOTE: state machine may not be necessary for BasicPrompt */

#[derive(Clone)]
enum State {
    Start,
    Done,
}

#[derive(Clone)]
pub struct BasicPromptStateMachine {
    state: State,
    _data: Option<Data>,
    // intermediate_results
    // output for client
}

impl NewStateMachine for BasicPromptStateMachine {
    fn new() -> Self {
        BasicPromptStateMachine {
            state: State::Start,
            _data: None,
        }
    }
}

impl StateMachine for BasicPromptStateMachine {
    fn step(&mut self, data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        // DONE: send 1 request
        match self.state {
            State::Start => {
                // TODO: prompting
                send_to_vllm(data);
                self.state = State::Done;
                Ok(ProcessResult::Complete)
            }
            // FIXME: not necessary
            State::Done => {
                // respond_to_client(data);
                Ok(ProcessResult::Complete)
            }
        }
    }
}

