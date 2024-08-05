use crate::{send_to_vllm, Data, PromptExchange};
use super::{NewStateMachine, ProcessResult, StateMachine};

use super::prompt_template::{BasicParams, BasicTemplate, PromptTemplate};

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
    // intermediate_results
    // output for client
}

impl NewStateMachine for BasicPromptStateMachine {
    fn new() -> Self {
        BasicPromptStateMachine {
            // state: State::Start,
        }
    }
}

impl StateMachine for BasicPromptStateMachine {
    fn step(&mut self, mut data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        let basic_template = BasicTemplate;
        if let Some(fasoo) = &data.prompt_keys.fasoo {
            
            let basic_params = BasicParams {
                task_instruction: fasoo.task_instruction.clone().unwrap(),
                chunk: fasoo.chunks.clone().unwrap().get(0).unwrap().chunk_text.clone(),
                user_query: data.prompt_keys.user_query.clone().unwrap(),
            };

            // reuse passed-in Data struct and create Option<PromptExchange> to Some
            data.prompt_exchange = Some(PromptExchange{
                index: 0,
                prompted_string: basic_template.fill(&basic_params),
                llm_response: None,
            });
            send_to_vllm(data);
        };
        Ok(ProcessResult::Complete)
        
        // self.state = State::Done;
        // match self.state {
        //     State::Start => {
        //     }


        //     // FIXME: not necessary
        //     State::Done => {
        //         // respond_to_client(data);
        //         Ok(ProcessResult::Complete)
        //     }
        // }
    }
}

