use super::ProcessResult;
use crate::{send_to_vllm, Data, PromptExchange};

use super::{NewStateMachine, StateMachine};

use super::prompt_template::{self, PromptTemplate, MapTemplate, MapParams, ReduceTemplate, ReduceParams};


#[derive(Clone)]
enum State {
    Map,
    Reduce,
    Done,
}

#[derive(Clone)]
pub struct MapReduceStateMachine {
    state: State,
    map_result: Option<Vec<String>>,
    chunk_num: usize,
    map_instruction: Option<String>,
    reduce_instruction: Option<String>,
}

impl NewStateMachine for MapReduceStateMachine {
    fn new() -> Self {
        MapReduceStateMachine {
            state: State::Map,
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
                // DONE: iterate through all chunks and send llm request
                // [x] parse instructions
                if let Some(fasoo) = &data.prompt_keys.fasoo {
                    // [x] fetch instructions
                    if let Some(pattern) = &fasoo.pattern {
                        if let Some(map_reduce) = &pattern.map_reduce {
                            self.map_instruction = Some(map_reduce.map_instruction.clone());
                            self.reduce_instruction = Some(map_reduce.reduce_instruction.clone());
                        }
                    }
                    // [x] iterate through each chunk and send request
                    if let Some(chunks) = &fasoo.chunks {
                        for chunk in chunks.iter() {
                            let mut data_to_send = data.clone();

                            /* [x]: parse params into Map Template */
                            let map_template: MapTemplate = prompt_template::MapTemplate;
                            let map_params: MapParams = prompt_template::MapParams {
                                map_instruction: self.map_instruction.clone().unwrap(),
                                chunk: chunk.chunk_text.clone(),
                            };

                            data_to_send.prompt_exchange = Some(PromptExchange {
                                index: self.chunk_num, // this increments for every chunk
                                // FIXME: use prompt template to construct a String, not vec<String>. fill in the blanks, etc.
                                /*
                                NOTE: prompt template will come in as paramter in step()

                                 */
                                // prompted_string: vec![
                                //     self.map_instruction.clone().unwrap(),
                                //     chunk.chunk_text.clone(),
                                // ],
                                prompted_string: map_template.fill(&map_params),
                                llm_response: None,
                            });
                            send_to_vllm(data_to_send);
                            self.chunk_num += 1; // increment
                        }
                    }
                }
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
                            if let Some(prompt_exchange) = &mut data.prompt_exchange {
                                if let Some(llm_response) = &prompt_exchange.llm_response {
                                    map_result.insert(prompt_exchange.index, llm_response.clone());
                                    // [x] decrement for each map result received
                                    self.chunk_num -= 1;
                                    // [x] Check if all items have been inserted, using self.chunk_num
                                    // if map_result.iter().all(|item| *item != String::default()) {
                                    if self.chunk_num == 0 {
                                        // DONE: reduce and send request
                                        let reduced_result = map_result.join("\n");
                                        /*
                                        NOTE: prompt for reduce operation
                                        - <reduce_instruction>
                                        - <intermediate_answers>
                                        - <task_instruction>
                                        - <input>
                                        */

                                        /* [x]: parse params into Reduce Template */
                                        let reduce_template: ReduceTemplate = prompt_template::ReduceTemplate;
                                        let reduce_params = ReduceParams {
                                            reduce_instruction: self.reduce_instruction.clone().unwrap(),
                                            map_result: reduced_result,
                                            task_instruction: data.prompt_keys.task_instruction.clone().unwrap(),
                                            user_query: data.prompt_keys.user_query.clone().unwrap(),
                                        };
                                        prompt_exchange.prompted_string = reduce_template.fill(&reduce_params);

                                        send_to_vllm(data);
                                        self.state = State::Done;
                                        return Ok(ProcessResult::Complete); // NOTE: returning Complete tells IO the next response is for client.
                                    } else {
                                        // return and wait for more map responses to come
                                        return Ok(ProcessResult::Incomplete);
                                    }
                                } else {
                                    // [ ] handle llm_response None case
                                    // return Err()
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
