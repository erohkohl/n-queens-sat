extern crate dpll;

use dpll::model::n_queens as queens;
use dpll::logic::sat as sat;
use dpll::view::renderservice as ui;

pub fn main() {
    let n = 4;
    let (_, model) = sat::solve(queens::generate_cnf(n), n * n);
    println!("{}", ui::print_chess_board(model));
}