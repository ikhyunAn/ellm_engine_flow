pub trait PromptTemplate {
    type Params;

    fn fill(&self, params: &Self::Params) -> String;
}

pub struct MapTemplate;
pub struct ReduceTemplate;
pub struct BasicTemplate;
pub struct Refine1Template;
pub struct Refine2Template;
pub struct MultiReduce1Template;
pub struct MultiReduce2Template;

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

pub struct BasicParams {
    pub task_instruction: String,
    pub chunk: String,
    pub user_query: String,
}

pub struct Refine1Params {
    pub task_instruction: String,
    pub chunk: String,
    pub user_query: String,
}

pub struct Refine2Params {
    pub refine_instruction: String,
    pub prev_result: String,
    pub task_instruction: String,
    pub chunk: String,
    pub user_query: String,
}

pub struct MultiReduce1Params {
    pub instruction: String,
    pub chunk: String,
    pub user_query: String,
}

pub struct MultiReduce2Params {
    pub reduce_instruction: String,
    pub prev_result: String,
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

impl PromptTemplate for BasicTemplate {
    type Params = BasicParams;

    fn fill(&self, params: &Self::Params) -> String {
        format!(
            r#"
{task_instruction}
{chunk}
{user_query}
"#,
            task_instruction = params.task_instruction,
            chunk = params.chunk,
            user_query = params.user_query
        )
    }
}

impl PromptTemplate for Refine1Template {
    type Params = Refine1Params;

    fn fill(&self, params: &Self::Params) -> String {
        format!(
            r#"
{task_instruction}
{chunk}
{user_query}
"#,
            task_instruction = params.task_instruction,
            chunk = params.chunk,
            user_query = params.user_query
        )
    }
}

impl PromptTemplate for Refine2Template {
    type Params = Refine2Params;

    fn fill(&self, params: &Self::Params) -> String {
        format!(
            r#"
{refine_instruction}
{prev_result}
{task_instruction}
{chunk}
{user_query}
"#,
            refine_instruction = params.refine_instruction,
            prev_result = params.prev_result,
            task_instruction = params.task_instruction,
            chunk = params.chunk,
            user_query = params.user_query
        )
    }
}

impl PromptTemplate for MultiReduce1Template {
    type Params = MultiReduce1Params;

    fn fill(&self, params: &Self::Params) -> String {
        format!(
            r#"
{instruction}
{chunk}
{user_query}
"#,
            instruction = params.instruction,
            chunk = params.chunk,
            user_query = params.user_query
        )
    }
}

impl PromptTemplate for MultiReduce2Template {
    type Params = MultiReduce2Params;

    fn fill(&self, params: &Self::Params) -> String {
        format!(
            r#"
{reduce_instruction}
{prev_result}
{task_instruction}
{user_query}
"#,
            reduce_instruction = params.reduce_instruction,
            prev_result = params.prev_result,
            task_instruction = params.task_instruction,
            user_query = params.user_query
        )
    }
}