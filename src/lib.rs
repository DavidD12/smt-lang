pub mod error;
pub mod parser;
pub mod problem;
pub mod solution;
pub mod solve;

pub mod combine;
pub use combine::*;

#[macro_use]
extern crate lalrpop_util;

pub fn ok_entry<S: Into<String>>(title: S) -> d_stuff::Entry {
    let title = d_stuff::Text::new(
        title,
        termion::style::Bold.to_string(),
        termion::color::Blue.fg_str(),
    );
    let message = d_stuff::Text::new(
        "OK",
        termion::style::Reset.to_string(),
        termion::color::Green.fg_str(),
    );
    d_stuff::Entry::new(d_stuff::Status::Success, title, Some(message), vec![])
}

pub fn load_file(
    pretty: &mut d_stuff::Pretty,
    problem: &mut problem::Problem,
    filename: &str,
    verbose: u8,
) -> Result<(), error::Error> {
    // Parsing
    match parser::parse_file(problem, filename) {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Parse   "));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    // Duplicate
    match problem.duplicate() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Unicity "));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    // Resolve
    match problem.resolve_type() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("ResolveT"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    match problem.resolve() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("ResolveE"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    // Check Interval
    match problem.check_interval() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Interval"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    // Check Bounded
    match problem.check_bounded() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Bounded "));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    // Check Type
    match problem.check_type() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Type    "));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
