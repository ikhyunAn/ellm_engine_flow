use crate::{send_to_vllm, Data};
use super::ProcessResult;

use super::{NewStateMachine, StateMachine};

#[derive(Clone)]
enum State {
    Map,
    Reduce,
}

#[derive(Clone)]
pub struct MapReduceStateMachine {
    state: State,
    _data: Option<Data>,
    map_result: Option<Vec<String>>
}

impl NewStateMachine for MapReduceStateMachine {
    fn new() -> Self {
        MapReduceStateMachine {
            state: State::Map,
            _data: None,
            map_result: None,
        }
    }
}

impl StateMachine for MapReduceStateMachine {
    fn step(&mut self, mut data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        // DONE: drive using states
        // NOTE: Removed infinite loop because it's unnecessary
        match self.state {
            State::Map => {
                // TODO: iterate through all chunks and send llm request
                send_to_vllm(data);
                self.state = State::Reduce;
                Ok(ProcessResult::Incomplete)
            }
            State::Reduce => {
                /* DONE: store llm responses for 'N' chunks in Vec,
                    [x] proceed only if all responses are collected - use DocChunkData.chunk_len & chunk_index.
                    [x] collect all into one llm_response
                        1. use size to create Vec: if the Vec is non-existent
                        2. use index to insert into Vec
                        3. Collect all when finished
                */
                // 1. create Vec if non-existent
                loop {
                    // loop,
                    match &mut self.map_result {
                        None => {
                            if let Some(doc_chunk) = data.doc_chunk_data.clone() {
                                self.map_result = Some(vec![String::default(); doc_chunk.chunk_len]);
                                // continue so that chunk can be inserted into the new map_result Vec
                            }
                        }
                        Some(map_result) => {
                            // 2. insert into Vec using index
                            if let Some(doc_chunk) = data.doc_chunk_data.clone() {
                                map_result.insert(doc_chunk.chunk_index, doc_chunk.chunk_text.unwrap().clone());
                                // break loop
                                // Check if all items have been inserted
                                if map_result.iter().all(|item| *item != String::default()) {
                                    // all inserted
                                    // DONE: reduce and send request
                                    let reduced_result = map_result.join("\n");
                                    // [x] reuse Data which will be sent to the llm for the final, reduced query
                                    data.prompted_input = reduced_result;
                                    send_to_vllm(data);
                                    return Ok(ProcessResult::Complete);
                                } else {
                                    // return and wait for more items to come
                                    return Ok(ProcessResult::Incomplete);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

