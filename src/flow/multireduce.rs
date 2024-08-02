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
pub struct MultiReduceStateMachine {
    state: State,
    _data: Option<Data>,
    map_result: Option<Vec<String>>,
    instruction_size: usize,
}

impl NewStateMachine for MultiReduceStateMachine {
    fn new() -> Self {
        MultiReduceStateMachine {
            state: State::Map,
            _data: None,
            map_result: None,
            instruction_size: 0,
        }
    }
}

/*
Multireduce assumes..
1. all instructions come in one vector
2. the last instruction is the reduce instruction, all others are multi instructions.
*/

impl StateMachine for MultiReduceStateMachine {
    fn step(&mut self, mut data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        match self.state {
            State::Map => {
                // DONE: request per instruction, not per background(chunk). - difference between mapreduce and multireduce
                self.instruction_size = data.instructions.len();
                for i in 0..(self.instruction_size - 1) {
                    // FIXME: prompting
                    /*
                    instruction
                    chunk
                    input
                     */
                    data.prompt_exchange.prompted_string = Some(vec![
                        data.instructions[i].clone(),
                        data.chunks[0].chunk_text.clone(),  // only one chunk in multireduce
                        data.input.clone()
                    ]);
                    send_to_vllm(data.clone());
                }

                self.state = State::Reduce;
                Ok(ProcessResult::Incomplete)
            }
            State::Reduce => {
                loop {
                    // loop,
                    match &mut self.map_result {
                        None => {
                            self.map_result = Some(vec![]);
                        }
                        Some(map_result) => {
                            // 2. insert into Vec using index
                            if let Some(llm_response) = data.prompt_exchange.llm_response.clone() {
                                /* NOTE: MultiReduce may not need ordered collections of responses from multi-instruction phase.
                                Instead of inserting into specific indicies, consider pushing into to a vector. In such case I don't need to create a sized vector.
                                */
                                map_result.push(llm_response);
                                // Check size
                                if map_result.len() == self.instruction_size - 1 {
                                    let reduced_result = map_result.join("\n");
                                    /*
                                    NOTE: prompt for reduce operation
                                    - <reduce_instruction>
                                    - <intermediate_answers>
                                    - <task_instruction>
                                    - <input>
                                    */
                                    data.prompt_exchange.prompted_string = Some(vec![
                                        data.instructions[self.instruction_size - 1].clone(), // NOTE: self.instruction_size - 1: reduce instruction
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

