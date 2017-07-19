extern crate dpll;

use std::collections::HashSet;

use dpll::logic::sat as SAT;
use dpll::logic::cnf as CNF;

#[test]
fn dpll_empty_cnf_is_sat() {
    let cnf: HashSet<Vec<i32>> = HashSet::new();
    let (res_bool, _): (bool, Vec<i32>) = SAT::dpll(cnf, vec![]);
    assert!(res_bool);
}

#[test]
fn dpll_empty_cnf_has_empty_model() {
    let cnf: HashSet<Vec<i32>> = HashSet::new();
    let (_, model): (bool, Vec<i32>) = SAT::dpll(cnf, vec![]);
    assert_eq!(model, vec![]);
}

#[test]
fn dpll_one_clause_one_lit_is_sat() {
    let number_of_var: usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    let (res_bool, _): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(res_bool);
}

#[test]
fn dpll_one_clause_one_lit_model() {
    let number_of_var: usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    let (_, model): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert_eq!(model, vec![1]);
}

#[test]
fn dpll_two_clause_two_lit_is_sat() {
    let number_of_var: usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    cnf.insert(vec![2]);
    let (res_bool, _): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(res_bool);
}

#[test]
fn dpll_two_clause_two_lit_model() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    let cnf_clone: HashSet<Vec<i32>> = cnf.clone();
    cnf.insert(vec![1]);
    cnf.insert(vec![2]);
    assert!(CNF::check_model(cnf_clone, vec![1, 2]));
}

#[test]
fn dpll_one_clause_two_lit_is_sat() {
    let number_of_var: usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1i32, 2i32]);
    let (res_bool, _): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(res_bool);
}

#[test]
fn dpll_one_clause_two_lit_model() {
    let number_of_var: usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1, 2]);
    let (_, model): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(model.contains(&1i32) || model.contains(&2i32));
}

#[test]
fn dpll_two_clause_two_lit_is_unsat() {
    let number_of_var: usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![-1i32]);
    cnf.insert(vec![1i32]);
    let (res_bool, _): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(!res_bool);
}

#[test]
fn dpll_three_clause_two_lit_is_unsat() {
    let number_of_var: usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1i32, 2i32]);
    cnf.insert(vec![-1i32]);
    cnf.insert(vec![-2i32]);
    let (res_bool, _): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(!res_bool);
}

#[test]
fn dpll_three_clause_two_lit_model() {
    let number_of_var: usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1i32, 2i32]);
    cnf.insert(vec![-1i32]);
    cnf.insert(vec![-2i32]);
    let (_, model): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(model.is_empty());
}

#[test]
fn dpll_four_clause_four_lit_is_unsat() {
    let number_of_var: usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1i32, 2i32, -3i32, 4i32]);
    cnf.insert(vec![-1i32, -3i32, -4i32]);
    cnf.insert(vec![-2i32, 3i32]);
    cnf.insert(vec![2i32, -3i32, 4i32]);
    let (res_bool, _): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(res_bool);
}

#[test]
fn dpll_four_clause_four_lit_is_model() {
    let number_of_var: usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    let cnf_clone: HashSet<Vec<i32>> = cnf.clone();
    cnf.insert(vec![1i32, 2i32, -3i32, 4i32]);
    cnf.insert(vec![-1i32, -3i32, -4i32]);
    cnf.insert(vec![-2i32, 3i32]);
    cnf.insert(vec![2i32, -3i32, 4i32]);
    let (_, model): (bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(CNF::check_model(cnf_clone, model));
}