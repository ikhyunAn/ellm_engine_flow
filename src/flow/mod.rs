use std::str::FromStr;

use crate::ProcessResult;
use crate::flow::statemachine::{NewStateMachine, StateMachine};
use crate::{send_to_vllm, respond_to_client, Data};

pub mod statemachine;

enum FlowState {
    // Connect,
    // CheckChoice,
    Factory,
    InProgress,
    Done,
}

pub struct Flow {
    state: FlowState,
    // llm_response: Option<Data>,
    // task_choice_count: u32,
    task_type: Option<ProcessPatternType>,
    process_patterns: Vec<Box<dyn StateMachine>>,
    current_pattern: Option<Box<dyn StateMachine>>,
}

impl Flow {
    fn new() -> Self {
        Flow {
            state: FlowState::Factory,
            // llm_response: None,
            // task_choice_count: 0, // always check three times
            task_type: None,
            process_patterns: Vec::new(),
            current_pattern: None,
        }
    }

    // NOTE: Return ProcessResult:: Incomplete / Complete for I/O to deallocate Flow struct?
    pub fn run(&mut self, data: Data) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        // TODO: Check inifinite loop case
        loop {
            match self.state {
                // NOTE: connection will be made before calling `run`
                // FlowState::CheckChoice => {
                //     // DONE: task choice and create state machines
                //     // [x] modify data for task_choice
                //     // [x] task choice x 3
                //     // [x] decide on task
                //     // [x] "chat", "doc_summary", "insight" => check in data.llm_task
                //     match data.llm_task.as_str() {
                //         "choice" => {
                //             // [x] LLM allows batch? -> send vector, don't loop three times
                //             for _ in 0..3 {
                //                 send_to_vllm(data.clone()); //FIXME: fix to batch
                //             }
                //             self.task_choice_count = 3;
                //             self.state = FlowState::Factory;
                //             return Ok(ProcessResult::Incomplete);
                //         }
                //         _ => {
                //             self.state = FlowState::Factory;
                //             // continue, do not return
                //         }
                //     }
                // }
                FlowState::Factory => {
                    match data.llm_task.as_str() {
                        llm_task => match ProcessPatternType::from_str(llm_task) {
                            Ok(task_type) => {
                                self.task_type = Some(task_type);
                                // self.state = FlowState::FactoryStateMachine;
                                // DONE: generate SM(s) based on the decided task
                                if let Some(task_type) = &mut self.task_type {
                                    self.process_patterns = match task_type {
                                        ProcessPatternType::BasicPrompt(machines)
                                        | ProcessPatternType::MapReduce(machines) =>
                                            /* NOTE: dyn is not Sized, move sm out of TaskType
                                                - This operation simply swaps pointers, no cloning of data
                                             */
                                            std::mem::take(machines)
                                        }
                                    };
                                    self.state = FlowState::InProgress;
                                
                                // continue, do not return
                            }
                            Err(_) => {
                                return Err(format!("Invalid task types or state machine generation not successful: {}", llm_task).into());
                            }
                        },
                    }
                }
                FlowState::InProgress => {
                    // If current_pattern exists, drive
                    if let Some(current_pattern) = &mut self.current_pattern {
                        match current_pattern.drive(data.clone()) {
                            Ok(process_result) => {
                                match process_result {
                                    ProcessResult::Incomplete => {
                                        return Ok(ProcessResult::Incomplete);  // wait for intermediate llm response
                                    }
                                    ProcessResult::Complete => {
                                        self.current_pattern = None;    // deallocates Box<dyn StateMachine>
                                        // [x] check if this was the last pattern
                                        if self.process_patterns.is_empty() {
                                            self.state = FlowState::Done;   // all state machines complete
                                        }
                                        return Ok(ProcessResult::Incomplete);  // wait for final llm response (to this sm / flow)
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(format!("Error in State Machines: {}", e).into());
                            }
                        }
                    } else {    // current_pattern is None
                        // pop and move to next pattern
                        if let Some(pattern) = self.process_patterns.remove(0).into() {
                            // update current_pattern and loop again.
                            self.current_pattern = Some(pattern);
                            // No return statement.
                        } else {
                            return Err(format!("process_patterns empty but FlowState hasn't completed.").into());
                        }
                    }
                }
                FlowState::Done => {
                    respond_to_client(data);
                    return Ok(ProcessResult::Complete);
                }
            }
        }
    }
}

pub fn create_flow() -> Flow {
    Flow::new()
}

enum ProcessPatternType {
    BasicPrompt(Vec<Box<dyn StateMachine>>),
    MapReduce(Vec<Box<dyn StateMachine>>),
    /* [ ] More Process Patterns */
}

impl ProcessPatternType {
    // NOTE: Use Vector since it's not known if all Process Patterns will be single SM.
    fn basicprompt() -> Self {
        ProcessPatternType::BasicPrompt(vec![Box::new(statemachine::basicprompt::BasicPromptStateMachine::new())])
    }
    fn mapreduce() -> Self {
        ProcessPatternType::MapReduce(vec![Box::new(statemachine::mapreduce::MapReduceStateMachine::new())])
    }
    /* [ ] More Process Pattern initializers */
}

impl FromStr for ProcessPatternType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basicprompt" | "basic_prompt" => Ok(ProcessPatternType::basicprompt()),
            "mapreduce" | "map_reduce" => Ok(ProcessPatternType::mapreduce()),
            /* [ ] More initializers */
            _ => Err(()),
        }
    }
}


// enum TaskType {
//     // NOTE: Each vector represent multihop pattern
//     Code(Vec<Box<dyn StateMachine>>),
//     Image(Vec<Box<dyn StateMachine>>),
//     Translate(Vec<Box<dyn StateMachine>>),
//     Explain(Vec<Box<dyn StateMachine>>),
//     ThisChatSummary(Vec<Box<dyn StateMachine>>),
//     ThisDocumentSummary(Vec<Box<dyn StateMachine>>),
//     SummarizeDoc(Vec<Box<dyn StateMachine>>),
//     Insight(Vec<Box<dyn StateMachine>>),
// }

// // Check actual multihop pattern for each task
// impl TaskType {
//     fn code() -> Self {
//         TaskType::Code(vec![Box::new(
//             statemachine::basicprompt::BasicPromptStateMachine::new(),
//         )])
//     }

//     fn image() -> Self {
//         TaskType::Image(vec![])
//     }

//     fn translate() -> Self {
//         TaskType::Translate(vec![])
//     }

//     fn explain() -> Self {
//         TaskType::Explain(vec![])
//     }

//     fn this_chat_summary() -> Self {
//         TaskType::ThisChatSummary(vec![])
//     }

//     fn this_document_summary() -> Self {
//         TaskType::ThisDocumentSummary(vec![])
//     }

//     fn summarize_doc() -> Self {
//         TaskType::SummarizeDoc(vec![Box::new(
//             statemachine::mapreduce::MapReduceStateMachine::new(),
//         )])
//     }

//     fn insight() -> Self {
//         TaskType::Insight(vec![Box::new(
//             statemachine::mapreduce::MapReduceStateMachine::new(),
//         )])
//     }
// }

// // vec![ProcessPattern::BasicPrompt, ProcessPattern::MapReduce]

// impl FromStr for TaskType {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str() {
//             "code" => Ok(TaskType::code()),
//             "image" => Ok(TaskType::image()),
//             "translate" => Ok(TaskType::translate()),
//             "explain" => Ok(TaskType::explain()),
//             "thischatsummary" | "this_chat_summary" => Ok(TaskType::this_chat_summary()),
//             "thisdocumentsummary" | "this_document_summary" => {
//                 Ok(TaskType::this_document_summary())
//             }
//             "summarizedoc" | "summarize_doc" | "doc_summary1" => Ok(TaskType::summarize_doc()),
//             "insight" | "insight1" => Ok(TaskType::insight()),
//             _ => Err(()),
//         }
//     }
// }
