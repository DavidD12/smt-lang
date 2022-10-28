use clap::Parser;
use smt_lang::{load_file, problem::*, solve::solve};
use std::env;
#[macro_use]
extern crate log;

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
    let args = Args::parse();
    if args.verbose > 0 {
        //
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "info")
        }
        // pretty_env_logger::init();
        env_logger::init();
    }
    //
    let mut problem = Problem::new();

    match load_file(&mut problem, &args.file) {
        Ok(_) => {
            if args.verbose >= 2 {
                info!("Problem\n{}", problem);
            }
            let response = solve(&problem, args.verbose);
            info!("{}", response.to_lang(&problem));
        }
        Err(_) => {}
    }
}
