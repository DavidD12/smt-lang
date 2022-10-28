use super::*;
use crate::problem::*;
use crate::solution::Solution;

pub fn solve(problem: &Problem, verbose: u8) -> Response {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::new(problem, &cfg, &ctx, &solver);
    // Initialize
    smt.init();
    if verbose >= 3 {
        info!("SMT Problem\n{}", solver);
    }
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => Response::NoSolution,
        z3::SatResult::Unknown => Response::Unknown,
        z3::SatResult::Sat => {
            let z3_model = solver.get_model().unwrap();
            if verbose >= 3 {
                info!("SMT Model\n{}", z3_model);
            }
            Response::Solution(Solution::new(&smt, &z3_model))
        }
    }
}
