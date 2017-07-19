extern crate dpll;

use std::collections::HashSet;

use dpll::model::n_queens as queens;
use dpll::logic::sat as solver;

#[test]
fn at_least_one_queen_in_every_row() {
    let n = 4;
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![11, 12, 13, 14]);
    expected.insert(vec![21, 22, 23, 24]);
    expected.insert(vec![31, 32, 33, 34]);
    expected.insert(vec![41, 42, 43, 44]);

    let result: HashSet<Vec<i32>> = queens::at_least_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_column() {
    let n = 4;
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![11, 21, 31, 41]);
    expected.insert(vec![12, 22, 32, 42]);
    expected.insert(vec![13, 23, 33, 43]);
    expected.insert(vec![14, 24, 34, 44]);

    let result: HashSet<Vec<i32>> = queens::at_least_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_row() {
    let n = 4;
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-11, -12]);
    expected.insert(vec![-11, -13]);
    expected.insert(vec![-11, -14]);
    expected.insert(vec![-12, -13]);
    expected.insert(vec![-12, -14]);
    expected.insert(vec![-13, -14]);

    expected.insert(vec![-21, -22]);
    expected.insert(vec![-21, -23]);
    expected.insert(vec![-21, -24]);
    expected.insert(vec![-22, -23]);
    expected.insert(vec![-22, -24]);
    expected.insert(vec![-23, -24]);

    expected.insert(vec![-31, -32]);
    expected.insert(vec![-31, -33]);
    expected.insert(vec![-31, -34]);
    expected.insert(vec![-32, -33]);
    expected.insert(vec![-32, -34]);
    expected.insert(vec![-33, -34]);

    expected.insert(vec![-41, -42]);
    expected.insert(vec![-41, -43]);
    expected.insert(vec![-41, -44]);
    expected.insert(vec![-42, -43]);
    expected.insert(vec![-42, -44]);
    expected.insert(vec![-43, -44]);

    let result: HashSet<Vec<i32>> = queens::at_most_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_column() {
    let n = 4;
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-11, -21]);
    expected.insert(vec![-11, -31]);
    expected.insert(vec![-11, -41]);
    expected.insert(vec![-21, -31]);
    expected.insert(vec![-21, -41]);
    expected.insert(vec![-31, -41]);

    expected.insert(vec![-12, -22]);
    expected.insert(vec![-12, -32]);
    expected.insert(vec![-12, -42]);
    expected.insert(vec![-22, -32]);
    expected.insert(vec![-22, -42]);
    expected.insert(vec![-32, -42]);

    expected.insert(vec![-13, -23]);
    expected.insert(vec![-13, -33]);
    expected.insert(vec![-13, -43]);
    expected.insert(vec![-23, -33]);
    expected.insert(vec![-23, -43]);
    expected.insert(vec![-33, -43]);

    expected.insert(vec![-14, -24]);
    expected.insert(vec![-14, -34]);
    expected.insert(vec![-14, -44]);
    expected.insert(vec![-24, -34]);
    expected.insert(vec![-24, -44]);
    expected.insert(vec![-34, -44]);

    let result: HashSet<Vec<i32>> = queens::at_most_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_diagonal() {
    let n = 4;
    let mut expected: HashSet<Vec<i32>> = HashSet::new();

    expected.insert(vec![-11, -22]);
    expected.insert(vec![-12, -23]);
    expected.insert(vec![-13, -24]);
    expected.insert(vec![-21, -32]);
    expected.insert(vec![-22, -33]);
    expected.insert(vec![-22, -44]);
    expected.insert(vec![-23, -34]);
    expected.insert(vec![-31, -42]);
    expected.insert(vec![-32, -43]);
    expected.insert(vec![-33, -44]);
    expected.insert(vec![-11, -33]);
    expected.insert(vec![-11, -44]);
    expected.insert(vec![-12, -34]);
    expected.insert(vec![-21, -43]);

    expected.insert(vec![-12, -21]);
    expected.insert(vec![-13, -22]);
    expected.insert(vec![-14, -23]);
    expected.insert(vec![-22, -31]);
    expected.insert(vec![-23, -32]);
    expected.insert(vec![-24, -33]);
    expected.insert(vec![-32, -41]);
    expected.insert(vec![-33, -42]);
    expected.insert(vec![-34, -43]);
    expected.insert(vec![-13, -31]);
    expected.insert(vec![-14, -32]);
    expected.insert(vec![-14, -41]);
    expected.insert(vec![-23, -41]);
    expected.insert(vec![-24, -42]);

    let result: HashSet<Vec<i32>> = queens::at_most_one_queen_in_every_diagonal(n);
    assert_eq!(result, expected);
}

#[test]
fn generate_cnf_is_sat_n_four() {
    let n: usize = 4;
    let cnf: HashSet<Vec<i32>> = queens::generate_cnf(n);
    let part_assign: Vec<i32> = Vec::with_capacity(n * n);
    let (is_sat, _): (bool, Vec<i32>) = solver::dpll(cnf, part_assign);
    assert!(is_sat);
}

#[test]
fn generate_cnf_check_model() {
    let n: usize = 4;
    let cnf: HashSet<Vec<i32>> = queens::generate_cnf(n);
    let part_assign: Vec<i32> = Vec::with_capacity(n * n);
    let mut expected_one: Vec<i32> =
        vec![-42, -22, -34, -44,
             43, -33, -23, -21,
             24, -14, -13, -32,
             31, 12, -11, -41];

    let mut expected_two: Vec<i32> =
        vec![42, 34, 21, 13,
             -11, -12, -14, -22,
             -23, -24, -31, -32,
             -33, -41, -43, -44];

    expected_one.sort_by(|x, y| y.cmp(x));
    expected_two.sort_by(|x, y| y.cmp(x));

    let (_, mut model): (bool, Vec<i32>) = solver::dpll(cnf, part_assign);
    model.sort_by(|x, y| y.cmp(x));
    assert!(expected_one == model || expected_two == model);
}