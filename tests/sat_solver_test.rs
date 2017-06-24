extern crate dpll;

use std::collections::HashSet;

use dpll::backtracking::sat as SAT;

#[test]
fn dpll_empty_cnf_is_sat() {
    let cnf:HashSet<Vec<i32>> = HashSet::new();
    let (res_bool, _ ):(bool, Vec<i32>) = SAT::dpll(cnf, vec![]);
    assert!(res_bool);
}

#[test]
fn dpll_empty_cnf_has_empty_model() {
    let cnf:HashSet<Vec<i32>> = HashSet::new();
    let (_, model):(bool, Vec<i32>) = SAT::dpll(cnf, vec![]);
    assert_eq!(model, vec![]);
}

#[test]
fn dpll_one_clause_one_lit_is_sat() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    let (res_bool, _ ):(bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(res_bool);
}

#[test]
fn dpll_one_clause_one_lit_model() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    let (_, model):(bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert_eq!(model, vec![1]);
}

#[test]
fn dpll_two_clause_two_lit_is_sat() {
    let number_of_var:usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    cnf.insert(vec![2]);
    let (res_bool, _ ):(bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert!(res_bool);
}
/*
#[test]
fn dpll_two_clause_two_lit_model() {
    let number_of_var:usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    cnf.insert(vec![2]);
    let (_, model):(bool, Vec<i32>) = SAT::dpll(cnf, vec);
    assert_eq!(model, vec![1, 2]);
}*/

/*
#[test]
fn test_dpll_case_two() {
    let number_of_var:usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _ ):(bool, Vec<i32>) = solver::dpll(vec![vec![1], vec![2]], vec);
    assert!(res_bool);
}

#[test]
fn test_dpll_case_three() {
    let number_of_var:usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _ ):(bool, Vec<i32>) = solver::dpll(vec![vec![-1], vec![2]], vec);
    assert!(res_bool);
}


#[test]
fn test_dpll_case_four() {
    let number_of_var:usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _ ):(bool, Vec<i32>) = solver::dpll(vec![vec![-1], vec![1, 2]], vec);
    assert!(res_bool);
}

#[test]
fn test_dpll_case_five() {
    let number_of_var:usize = 3;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _ ):(bool, Vec<i32>) =
        solver::dpll(vec![vec![-1, -2], vec![1, 2, -3], vec![1, 3]], vec);
    assert!(res_bool);
}

#[test]
fn test_dpll_case_six() {
    let number_of_var:usize = 2;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _ ):(bool, Vec<i32>) = solver::dpll(vec![vec![-1, -2], vec![1]], vec);
    assert!(res_bool);
}

#[test]
#[should_panic]
fn test_dpll_unsat_case_one() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _ ):(bool, Vec<i32>) = solver::dpll(vec![vec![-1], vec![1]], vec);
    assert!(res_bool);
}

#[test]
#[should_panic]
fn test_dpll_unsat_case_two() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _ ):(bool, Vec<i32>) = solver::dpll(vec![vec![-1], vec![1, 2], vec![-2]], vec);
    assert!(res_bool);
}
*/