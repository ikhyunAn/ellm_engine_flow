use crate::{send_to_vllm, Data};
use super::ProcessResult;

use super::{NewStateMachine, StateMachine};

#[derive(Clone)]
enum State {
    Start,      // no llm response for input
    Refine,     // exists
    Done,       // last llm response, return to client
}

#[derive(Clone)]
pub struct RefineStateMachine {
    state: State,
    _data: Option<Data>,
    map_result: Option<Vec<String>>,
    background_index: u32,
    background_size: u32,
}

impl NewStateMachine for RefineStateMachine {
    fn new() -> Self {
        RefineStateMachine {
            state: State::Start,
            _data: None,
            map_result: None,
            background_index: 0,
            background_size: 0,
        }
    }
}

impl StateMachine for RefineStateMachine {
    fn step(&mut self, mut data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        match self.state {
            State::Start => {
                // TODO: initial request to llm
                // [ ]: select first background in data - use `background_index` = 0
                // [ ]: check how many background chunks
                self.background_size = 10; //FIXME: number of background chunks
                send_to_vllm(data);
                self.background_index += 1;
                self.state = State::Refine;
                Ok(ProcessResult::Incomplete)
            }
            State::Refine => {
                // TODO: use previous response to generate another llm response
                // [ ]: append llm response to data
                // [ ]: include next background as chunk in data
                send_to_vllm(data);
                self.background_index += 1;

                // last backgorund chunk used, move to Done state
                if self.background_index == self.background_size {
                    self.state = State::Done;
                }
                Ok(ProcessResult::Incomplete)
            }
            State::Done => {
                // respond_to_client(data);
                return Ok(ProcessResult::Complete);
            }
        }
    }
}


// State::Reduce => {
//     loop {
//         // loop,
//         match &mut self.map_result {
//             None => {
//                 if let Some(doc_chunk) = data.doc_chunk_data.clone() {
//                     self.map_result = Some(vec![String::default(); doc_chunk.chunk_len]);
//                     // continue so that chunk can be inserted into the new map_result Vec
//                 }
//             }
//             Some(map_result) => {
//                 // 2. insert into Vec using index
//                 if let Some(doc_chunk) = data.doc_chunk_data.clone() {
//                     map_result.insert(doc_chunk.chunk_index, doc_chunk.chunk_text.unwrap().clone());
//                     // break loop
//                     // Check if all items have been inserted
//                     if map_result.iter().all(|item| *item != String::default()) {
//                         // all inserted
//                         // DONE: reduce and send request
//                         let reduced_result = map_result.join("\n");
//                         // [x] reuse Data which will be sent to the llm for the final, reduced query
//                         data.prompted_input = reduced_result;
//                         send_to_vllm(data);
//                         self.state = State::Done;
//                         return Ok(ProcessResult::Incomplete);
//                     } else {
//                         // return and wait for more items to come
//                         return Ok(ProcessResult::Incomplete);
//                     }
//                 }
//             }
//         }
//     }
// }


