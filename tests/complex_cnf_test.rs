extern crate dpll;

use std::collections::HashSet;

use dpll::logic::sat as solver;

#[test]
fn test_complex_cnf_case_one() {
    let number_of_var: usize = 5;
    let partial_assignment: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();

    cnf.insert(vec![1, 2, 3]);
    cnf.insert(vec![1, 2, 4]);
    cnf.insert(vec![1, 2, -4]);
    cnf.insert(vec![1, 5, -2]);
    cnf.insert(vec![1, -3, -4]);
    cnf.insert(vec![2, 3, -4]);
    cnf.insert(vec![2, 5, 4]);
    cnf.insert(vec![3, 5, -2]);
    cnf.insert(vec![3, 5, -4]);
    cnf.insert(vec![3, -2, -5]);
    cnf.insert(vec![5, 4, -1]);
    cnf.insert(vec![4, -1, -3]);
    cnf.insert(vec![4, -1, -3]);
    cnf.insert(vec![4, -2, -5]);
    cnf.insert(vec![-1, -2, -5]);
    cnf.insert(vec![-1, -3, -5]);
    cnf.insert(vec![-2, -5, -4]);
    cnf.insert(vec![-3, -5, -4]);

    let (res_bool, _): (bool, Vec<i32>) = solver::dpll(cnf, partial_assignment);
    assert!(res_bool);
}

#[test]
fn test_complex_cnf_case_two() {
    let number_of_var: usize = 5;
    let partial_assignment: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1, 2, 3]);
    cnf.insert(vec![-1]);
    cnf.insert(vec![-3]);
    cnf.insert(vec![-2]);
    cnf.insert(vec![1, 2, 4]);
    cnf.insert(vec![1, 2, -4]);
    cnf.insert(vec![1, 5, -2]);
    cnf.insert(vec![1, -3, -4]);
    cnf.insert(vec![2, 3, -4]);
    cnf.insert(vec![2, 5, 4]);
    cnf.insert(vec![3, 5, -2]);
    cnf.insert(vec![3, 5, -4]);
    cnf.insert(vec![3, -2, -5]);
    cnf.insert(vec![5, 4, -1]);
    cnf.insert(vec![4, -1, -3]);
    cnf.insert(vec![4, -1, -3]);
    cnf.insert(vec![4, -2, -5]);
    cnf.insert(vec![4, -2, -5]);
    cnf.insert(vec![-1, -2, -5]);
    cnf.insert(vec![-1, -3, -5]);
    cnf.insert(vec![-2, -5, -4]);
    cnf.insert(vec![-3, -5, -4]);

    let (res_bool, _): (bool, Vec<i32>) = solver::dpll(cnf, partial_assignment);
    assert!(!res_bool);
}