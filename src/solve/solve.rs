use super::*;
use crate::problem::*;
use crate::solution::Solution;

pub fn solve(
    pretty: &mut d_stuff::Pretty,
    problem: &Problem,
    verbose: u8,
    threads: u32,
) -> Response {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let mut smt = Smt::new(problem, &cfg, &ctx);
    // Initialize
    smt.init();
    if verbose >= 3 {
        pretty.add(smt.solver_to_entry());
        pretty.print();
    }
    // Solve
    if threads > 1 {
        smt.solver().set_threads(&ctx, threads);
    }
    match smt.solver().check() {
        z3::SatResult::Unsat => Response::NoSolution,
        z3::SatResult::Unknown => Response::Unknown,
        z3::SatResult::Sat => {
            let z3_model = smt.solver().get_model().unwrap();
            if verbose >= 3 {
                pretty.add(smt.model_to_entry());
                pretty.print();
            }
            Response::Solution(Solution::new(&smt, &z3_model))
        }
    }
}
