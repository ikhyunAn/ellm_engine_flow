use crate::{send_to_vllm, Data};
use super::{NewStateMachine, ProcessResult, StateMachine};

#[derive(Clone)]
enum State {
    Map,
    Reduce,
}

#[derive(Clone)]
pub struct MapReduceStateMachine {
    state: State,
    _data: Option<Data>,
    // intermediate_results
    // output for client
}

impl NewStateMachine for MapReduceStateMachine {
    fn new() -> Self {
        MapReduceStateMachine {
            state: State::Map,
            _data: None,
        }
    }
}

impl StateMachine for MapReduceStateMachine {
    fn drive(&mut self, data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        // TODO: drive using states
        loop {
            match self.state {
                State::Map => {
                    send_to_vllm(data);
                    self.state = State::Reduce;
                    return Ok(ProcessResult::Incomplete);
                }
                State::Reduce => {
                    /* TODO: store llm responses for 'N' chunks in Vec,
                        [ ] proceed only if all responses are collected - use DocChunkData.chunk_len & chunk_index.
                        [ ] collect all into one llm_response
                    */
                    


                    send_to_vllm(data);
                    return Ok(ProcessResult::Complete);
                }
            }
        }
    }
}

