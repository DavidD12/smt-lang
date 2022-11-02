use crate::problem::{Problem, ToEntry, ToLang};
use crate::solution::Solution;

pub enum Response {
    NoSolution,
    Unknown,
    Solution(Solution),
}

//------------------------- To Lang -------------------------

impl ToLang for Response {
    fn to_lang(&self, problem: &Problem) -> String {
        match self {
            Response::NoSolution => "no solution".to_string(),
            Response::Unknown => "unknown".to_string(),
            Response::Solution(solution) => format!("one solution:\n{}", solution.to_lang(problem)),
        }
    }
}

//------------------------- To Entry -------------------------

impl ToEntry for Response {
    fn to_entry(&self, problem: &Problem) -> d_stuff::Entry {
        match self {
            Response::NoSolution => d_stuff::Entry::new(
                d_stuff::Status::Failure,
                d_stuff::Text::new(
                    "Solve   ",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(d_stuff::Text::new(
                    "UNSAT",
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                )),
                vec![],
            ),
            Response::Unknown => d_stuff::Entry::new(
                d_stuff::Status::Question,
                d_stuff::Text::new(
                    "Solve   ",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(d_stuff::Text::new(
                    "UNKNOWN",
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                )),
                vec![],
            ),
            Response::Solution(solution) => d_stuff::Entry::new(
                d_stuff::Status::Success,
                d_stuff::Text::new(
                    "Solve   ",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(d_stuff::Text::new(
                    "SAT",
                    termion::style::Reset.to_string(),
                    termion::color::Green.fg_str(),
                )),
                vec![d_stuff::Message::new(
                    None,
                    d_stuff::Text::new(
                        solution.to_lang(problem),
                        termion::style::Reset.to_string(),
                        termion::color::White.fg_str(),
                    ),
                )],
            ),
        }
    }
}
