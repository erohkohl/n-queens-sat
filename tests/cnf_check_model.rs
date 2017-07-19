extern crate dpll;

use std::collections::HashSet;

use dpll::logic::cnf as cnf;

#[test]
pub fn check_model_trivial_case_is_model() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1i32]);
    let model: Vec<i32> = vec![1];
    assert!(cnf::check_model(cnf, model));
}

#[test]
pub fn check_model_trivial_case_no_model() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1i32]);
    let model: Vec<i32> = vec![-1];
    assert!(!cnf::check_model(cnf, model));
}

#[test]
pub fn check_model_one_clause_two_lit_is_model() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1i32, -2i32]);
    let model: Vec<i32> = vec![1];
    assert!(cnf::check_model(cnf, model));
}

#[test]
pub fn check_model_four_clause_four_lit_is_model() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1i32, 2i32, -3i32, 4i32]);
    cnf.insert(vec![-1i32, -3i32, -4i32]);
    cnf.insert(vec![-2i32, 3i32]);
    cnf.insert(vec![2i32, -3i32, 4i32]);
    let model: Vec<i32> = vec![-1, -2, -3, -4];
    assert!(cnf::check_model(cnf, model));
}