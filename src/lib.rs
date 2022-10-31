pub mod error;
pub mod parser;
pub mod problem;
pub mod solution;
pub mod solve;

use crate::problem::ToLang;

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub fn load_file(problem: &mut problem::Problem, filename: &str) -> Result<(), error::Error> {
    // Parsing
    match parser::parse_file(problem, filename) {
        Ok(_) => info!("Parsing OK"),
        Err(e) => {
            error!("{}", e.to_lang(problem));
            return Err(e);
        }
    }
    // Duplicate
    match problem.duplicate() {
        Ok(_) => info!("Duplicate OK"),
        Err(e) => {
            error!("{}", e.to_lang(problem));
            return Err(e);
        }
    }
    // Resolve
    match problem.resolve() {
        Ok(_) => info!("Resolve OK"),
        Err(e) => {
            error!("{}", e.to_lang(problem));
            return Err(e);
        }
    }
    // Check Interval
    match problem.check_interval() {
        Ok(_) => info!("Interval OK"),
        Err(e) => {
            error!("{}", e.to_lang(problem));
            return Err(e);
        }
    }
    // Check Type
    match problem.check_type() {
        Ok(_) => info!("Typing OK"),
        Err(e) => {
            error!("{}", e.to_lang(problem));
            return Err(e);
        }
    }

    Ok(())
}
