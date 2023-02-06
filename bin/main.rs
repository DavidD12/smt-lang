use clap::Parser;
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
    /// parallel N (number of threads)
    #[arg(short, long, default_value_t = 1)]
    parallel: u32,
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
                pretty.print();
            }
            let response = solve(&mut pretty, &problem, args.verbose, args.parallel);
            pretty.add(response.to_entry(&problem));
            if args.verbose > 0 {
                pretty.print();
            } else {
                println!("{}", response.to_lang(&problem));
            }
        }
        Err(e) => {
            pretty.add(e.to_entry(&problem));
            if args.verbose > 0 {
                pretty.print();
            }
        }
    }
}
