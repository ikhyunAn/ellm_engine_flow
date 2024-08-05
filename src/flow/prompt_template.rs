pub trait PromptTemplate {
    type Params;

    fn fill(&self, params: &Self::Params) -> String;
}

/* TODO: Define Template structs for each Prompt Template */
pub struct MapTemplate;
pub struct ReduceTemplate;

pub struct MapParams {
    pub map_instruction: String,
    pub chunk: String,
}

pub struct ReduceParams {
    pub reduce_instruction: String,
    pub map_result: String,
    pub task_instruction: String,
    pub user_query: String,
}

impl PromptTemplate for MapTemplate {
    type Params = MapParams;

    fn fill(&self, params: &Self::Params) -> String {
        format!(
            r#"
{map_instruction}
{chunk}
"#,
            map_instruction = params.map_instruction,
            chunk = params.chunk
        )
    }
}

impl PromptTemplate for ReduceTemplate {
    type Params = ReduceParams;

    fn fill(&self, params: &Self::Params) -> String {
        format!(
            r#"
{reduce_instruction}
{map_result}
{task_instruction}
{user_query}
"#,
            reduce_instruction = params.reduce_instruction,
            map_result = params.map_result,
            task_instruction = params.task_instruction,
            user_query = params.user_query
        )
    }
}