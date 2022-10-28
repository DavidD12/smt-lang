use crate::{problem::ToLang, solution::Solution};

pub enum Response {
    NoSolution,
    Unknown,
    Solution(Solution),
}

//------------------------- To Lang -------------------------

impl ToLang for Response {
    fn to_lang(&self, problem: &crate::problem::Problem) -> String {
        match self {
            Response::NoSolution => "no solution".to_string(),
            Response::Unknown => "unknown".to_string(),
            Response::Solution(solution) => format!("one solution:\n{}", solution.to_lang(problem)),
        }
    }
}
