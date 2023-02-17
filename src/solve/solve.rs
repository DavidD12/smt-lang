use super::*;
use crate::problem::*;
use crate::solution::Solution;

pub fn solve(
    pretty: &mut d_stuff::Pretty,
    problem: &Problem,
    verbose: u8,
    threads: u32,
    optimize: bool,
) -> Response {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    // Solver
    let solver = if optimize || problem.search().is_optimize() {
        Z3Solver::Optimize(z3::Optimize::new(&ctx))
    } else {
        Z3Solver::Solve(z3::Solver::new(&ctx))
    };
    // Patam
    if threads > 1 {
        solver.set_threads(&ctx, threads);
    }
    // SMT
    let mut smt = Smt::new(problem, &cfg, &ctx, &solver);
    // Initialize
    smt.init();
    if verbose >= 3 {
        pretty.add(smt.solver_to_entry());
        pretty.print();
    }

    match problem.search() {
        Search::Solve => match solver.check() {
            z3::SatResult::Unsat => Response::NoSolution,
            z3::SatResult::Unknown => Response::Unknown,
            z3::SatResult::Sat => {
                let z3_model = solver.get_model().unwrap();
                if verbose >= 3 {
                    pretty.add(smt.model_to_entry());
                    pretty.print();
                }
                Response::Solution(Solution::new(&smt, &z3_model))
            }
        },
        Search::Optimize(expr, bound, minimize) => {
            let expr = expr.type_inference(problem);
            match bound {
                Bound::Int(value) => {
                    let e = smt.to_int(&expr);
                    let b = smt.to_int(&Expr::IntValue(*value, None));
                    if *minimize {
                        solver.assert(&e.ge(&b));
                        solver.minimize_int(&e);
                    } else {
                        solver.assert(&e.le(&b));
                        solver.maximize_int(&e);
                    }
                    match solver.check() {
                        z3::SatResult::Unsat => Response::NoSolution,
                        z3::SatResult::Unknown => Response::Unknown,
                        z3::SatResult::Sat => {
                            let z3_model = solver.get_model().unwrap();
                            if verbose >= 3 {
                                pretty.add(smt.model_to_entry());
                                pretty.print();
                            }
                            Response::Solution(Solution::new(&smt, &z3_model))
                        }
                    }
                }
                Bound::Real(value) => {
                    let e = smt.to_real(&expr);
                    let b = smt.to_real(&Expr::RealValue(*value, None));
                    if *minimize {
                        solver.assert(&e.ge(&b));
                        solver.minimize_real(&e);
                    } else {
                        solver.assert(&e.le(&b));
                        solver.maximize_real(&e);
                    }
                    match solver.check() {
                        z3::SatResult::Unsat => Response::NoSolution,
                        z3::SatResult::Unknown => Response::Unknown,
                        z3::SatResult::Sat => {
                            let z3_model = solver.get_model().unwrap();
                            if verbose >= 3 {
                                pretty.add(smt.model_to_entry());
                                pretty.print();
                            }
                            Response::Solution(Solution::new(&smt, &z3_model))
                        }
                    }
                }
            }
        }
    }
}

pub fn current_optim(value: String) -> d_stuff::Entry {
    d_stuff::Entry::new(
        d_stuff::Status::Info,
        d_stuff::Text::new(
            "Optimize",
            termion::style::Bold.to_string(),
            termion::color::Blue.fg_str(),
        ),
        Some(d_stuff::Text::new(
            "Current Solution",
            termion::style::Reset.to_string(),
            termion::color::Green.fg_str(),
        )),
        vec![d_stuff::Message::new(
            None,
            d_stuff::Text::new(
                value,
                termion::style::Reset.to_string(),
                termion::color::White.fg_str(),
            ),
        )],
    )
}
