extern crate dpll;

use std::collections::HashSet;

use dpll::logic::sat as sat;

#[test]
fn test_trim_simple_one() {
    let mut cnf_to_trim: HashSet<Vec<i32>> = HashSet::new();
    cnf_to_trim.insert(vec![1]);
    cnf_to_trim.insert(vec![2]);
    let result = sat::trim_cnf(cnf_to_trim, 1);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![2]);
    assert_eq!(result, cnf_expected);
}


#[test]
fn test_trim_simple_two() {
    let mut cnf_to_trim: HashSet<Vec<i32>> = HashSet::new();
    cnf_to_trim.insert(vec![1, 2]);
    cnf_to_trim.insert(vec![3, 4]);
    let result = sat::trim_cnf(cnf_to_trim, 1);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![3, 4]);
    assert_eq!(result, cnf_expected);
}


#[test]
fn test_trim_simple_three() {
    let mut cnf_to_trim: HashSet<Vec<i32>> = HashSet::new();
    cnf_to_trim.insert(vec![1, 2]);
    cnf_to_trim.insert(vec![2, 4]);
    cnf_to_trim.insert(vec![5]);
    let result = sat::trim_cnf(cnf_to_trim, 2);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![5]);
    assert_eq!(result, cnf_expected);
}

#[test]
fn test_trim_neg_one() {
    let mut cnf_to_trim: HashSet<Vec<i32>> = HashSet::new();
    cnf_to_trim.insert(vec![1, -2]);
    cnf_to_trim.insert(vec![2, 4]);
    cnf_to_trim.insert(vec![5]);
    let result = sat::trim_cnf(cnf_to_trim, 2);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![5]);
    cnf_expected.insert(vec![1]);
    assert_eq!(result, cnf_expected);
}

#[test]
fn test_trim_neg_two() {
    let mut cnf_to_trim: HashSet<Vec<i32>> = HashSet::new();
    cnf_to_trim.insert(vec![1, -2]);
    cnf_to_trim.insert(vec![2, 4]);
    cnf_to_trim.insert(vec![5]);
    let result = sat::trim_cnf(cnf_to_trim, -2);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![5]);
    cnf_expected.insert(vec![4]);
    assert_eq!(result, cnf_expected);
}

#[test]
fn test_trim_neg_three() {
    let mut cnf_to_trim: HashSet<Vec<i32>> = HashSet::new();
    cnf_to_trim.insert(vec![1, -2]);
    cnf_to_trim.insert(vec![2, 4]);
    cnf_to_trim.insert(vec![-5]);
    let result = sat::trim_cnf(cnf_to_trim, -5);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![1, -2]);
    cnf_expected.insert(vec![2, 4]);
    assert_eq!(result, cnf_expected);
}

#[test]
fn test_trim_neg_four() {
    let mut cnf_to_trim: HashSet<Vec<i32>> = HashSet::new();
    cnf_to_trim.insert(vec![1, -2]);
    cnf_to_trim.insert(vec![2, 4]);
    cnf_to_trim.insert(vec![-2, -5]);
    let result = sat::trim_cnf(cnf_to_trim, -2);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![4]);
    assert_eq!(result, cnf_expected);
}

#[test]
fn trim_test_clausel_five_lit_cnf() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![1, -2]);
    cnf.insert(vec![3, 4, 5]);
    let result: HashSet<Vec<i32>>;
    result = sat::trim_cnf(cnf.clone(), -2);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![3]);
    cnf_expected.insert(vec![3, 4, 5]);
    assert_eq!(result, cnf_expected);
}

#[test]
fn trim_test_two_clausel_one_unit() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![3]);
    cnf.insert(vec![-3, 4, 5]);
    let result: HashSet<Vec<i32>>;
    result = sat::trim_cnf(cnf.clone(), 3);
    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![4, 5]);
    assert_eq!(result, cnf_expected);
}

#[test]
fn trim_test_two_clausel_cnf_empty() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![3]);
    cnf.insert(vec![3, 4, 5]);
    let result: HashSet<Vec<i32>>;
    result = sat::trim_cnf(cnf.clone(), 3);
    assert!(result.is_empty());
}

#[test]
fn trom_cnf_two_clause_each_one_lit() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    let cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    cnf.insert(vec![2]);
    cnf = sat::trim_cnf(cnf.clone(), 1);
    cnf = sat::trim_cnf(cnf.clone(), 2);
    assert_eq!(cnf, cnf_expected);
}