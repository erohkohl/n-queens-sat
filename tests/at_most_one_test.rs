extern crate dpll;

use std::collections::HashSet;

use dpll::model::n_queens as queens;

#[test]
fn at_most_one_test_case_one() {
    let xs: Vec<i32> = vec![1];
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![1]);
    let result: HashSet<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_test_case_two() {
    let xs: Vec<i32> = vec![1, 2];
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-1, -2]);
    let result: HashSet<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_test_case_three() {
    let xs: Vec<i32> = vec![1, 2, 3];
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-1, -2]);
    expected.insert(vec![-1, -3]);
    expected.insert(vec![-2, -3]);
    let result: HashSet<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_test_case_four() {
    let xs: Vec<i32> = vec![1, 4, 5, 6];
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-1, -4]);
    expected.insert(vec![-1, -5]);
    expected.insert(vec![-1, -6]);
    expected.insert(vec![-4, -5]);
    expected.insert(vec![-4, -6]);
    expected.insert(vec![-5, -6]);
    let result: HashSet<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_test_case_five() {
    let xs: Vec<i32> = vec![1, 2, 3, 4];
    let mut expected: HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-1, -2]);
    expected.insert(vec![-1, -3]);
    expected.insert(vec![-2, -3]);
    expected.insert(vec![-2, -4]);
    expected.insert(vec![-3, -4]);
    let result: HashSet<Vec<i32>> = queens::at_most_one(xs);
    assert!(!result.eq(&expected));
}