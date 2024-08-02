use crate::{send_to_vllm, Data};
use super::ProcessResult;

use super::{NewStateMachine, StateMachine};

#[derive(Clone)]
enum State {
    Map,
    Reduce,
    Done,
}

#[derive(Clone)]
pub struct MapReduceStateMachine {
    state: State,
    _data: Option<Data>,
    map_result: Option<Vec<String>>,
    chunk_num: usize,
    map_instruction: Option<String>,
    reduce_instruction: Option<String>,
}

impl NewStateMachine for MapReduceStateMachine {
    fn new() -> Self {
        MapReduceStateMachine {
            state: State::Map,
            _data: None,
            map_result: None,
            chunk_num: 0,
            map_instruction: None,
            reduce_instruction: None,
        }
    }
}

impl StateMachine for MapReduceStateMachine {
    fn step(&mut self, mut data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        // DONE: drive using states
        // NOTE: Removed infinite loop because it's unnecessary
        match self.state {
            State::Map => {
                // TODO: parse map and reduce instructions

                // DONE: iterate through all chunks and send llm request
                for chunk in data.chunks.iter() {
                    /* FIXME: 
                        1. use parsed instructions- map_inst and reduce_inst
                        2.  */
                    data.prompt_exchange.prompted_string = Some(vec![data.instructions[0].clone(), chunk.chunk_text.clone()]);
                    send_to_vllm(data.clone());
                    self.chunk_num += 1;
                }
                // send_to_vllm(data);
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
                loop {
                    // loop,
                    match &mut self.map_result {
                        None => {
                            self.map_result = Some(vec![String::default(); self.chunk_num]);
                            // create map_result and continue so that chunk can be inserted into the new map_result Vec

                            // if let Some(llm_response) = data.prompt_exchange.llm_response.clone() {
                            //     self.map_result.insert(data.prompt_exchange.index, llm_response);
                            // }
                        }
                        Some(map_result) => {
                            // 2. insert into Vec using index
                            if let Some(llm_response) = data.prompt_exchange.llm_response.clone() {
                                map_result.insert(data.prompt_exchange.index, llm_response);
                                // break loop
                                // Check if all items have been inserted
                                if map_result.iter().all(|item| *item != String::default()) {
                                    // DONE: reduce and send request
                                    let reduced_result = map_result.join("\n");
                                    /*
                                    NOTE: prompt for reduce operation
                                    - <reduce_instruction>
                                    - <intermediate_answers>
                                    - <task_instruction>
                                    - <input>
                                    */
                                    data.prompt_exchange.prompted_string = Some(vec![
                                        data.instructions[1].clone(), 
                                        reduced_result,
                                        data.task_instruction.clone(),
                                        data.input.clone()]);
                                    send_to_vllm(data);
                                    self.state = State::Done;
                                    return Ok(ProcessResult::Complete); // NOTE: returning Complete tells IO the next response is for client.
                                } else {
                                    // return and wait for more map responses to come
                                    return Ok(ProcessResult::Incomplete);
                                }
                            }
                        }
                    }
                }
            } 
            State::Done => {
                // respond_to_client(data);
                return Ok(ProcessResult::Complete);
            }
        }
    }
}

