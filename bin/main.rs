use clap::Parser;
use d_stuff::*;
use smt_lang::{load_file, problem::*, solve::solve};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input file
    #[arg(short, long)]
    file: String,
    /// verbose level
    #[arg(short, long, default_value_t = 1)]
    verbose: u8,
}

fn main() {
    let mut pretty = d_stuff::Pretty::new();

    let args = Args::parse();
    //
    let mut problem = Problem::new();

    match load_file(&mut pretty, &mut problem, &args.file, args.verbose) {
        Ok(_) => {
            if args.verbose >= 2 {
                pretty.add(problem.to_entry());
            }
            let response = solve(&mut pretty, &problem, args.verbose);
            pretty.add(response.to_entry(&problem));
        }
        Err(e) => pretty.add(e.to_entry(&problem)),
    }

    pretty.print();
}
