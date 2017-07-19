extern crate dpll;

use std::collections::HashSet;

use dpll::logic::cnf as cnf;

#[test]
fn test_cnf_equals_one() {
    let cnf: HashSet<Vec<i32>> = HashSet::new();
    assert!(cnf::equals(cnf.clone(), cnf.clone()));
}

#[test]
fn test_cnf_equals_two() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    assert!(cnf::equals(cnf.clone(), cnf.clone()));
}

#[test]
fn test_cnf_equals_three() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    cnf.insert(vec![2]);
    assert!(cnf::equals(cnf.clone(), cnf.clone()));
}

#[test]
fn test_cnf_equals_four() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1, 3]);
    cnf.insert(vec![2]);
    assert!(cnf::equals(cnf.clone(), cnf.clone()));
}

#[test]
fn test_cnf_equals_five() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2]);
    cnf.insert(vec![1, 3]);
    let mut cnf_two: HashSet<Vec<i32>> = HashSet::new();
    cnf_two.insert(vec![1, 3]);
    cnf_two.insert(vec![2]);
    assert!(cnf::equals(cnf, cnf_two));
}