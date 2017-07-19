extern crate dpll;


use dpll::model::n_queens as queens;

#[test]
fn at_least_one_test_case_one() {
    let xs: Vec<i32> = vec![1];
    let expected: Vec<i32> = vec![1];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_test_case_two() {
    let xs: Vec<i32> = vec![1, 2];
    let expected: Vec<i32> = vec![1, 2];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_test_case_three() {
    let xs: Vec<i32> = vec![1, 2, 3];
    let expected: Vec<i32> = vec![1, 2, 3];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_test_case_four() {
    let xs: Vec<i32> = vec![1, 2, 4];
    let expected: Vec<i32> = vec![1, 2, 4];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_test_case_five() {
    let xs: Vec<i32> = vec![1, 2, 4, 6];
    let expected: Vec<i32> = vec![1, 2, 4, 6];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn at_least_one_test_case_six() {
    let xs: Vec<i32> = vec![1, 2, 4, 6];
    let expected: Vec<i32> = vec![1, 2, 4, -6];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn at_least_one_test_case_seven() {
    let xs: Vec<i32> = vec![1, 2];
    let expected: Vec<i32> = vec![-1, 2];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn at_least_one_test_case_eigth() {
    let xs: Vec<i32> = vec![1, 2];
    let expected: Vec<i32> = vec![-1, -2];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn at_least_one_test_case_nine() {
    let xs: Vec<i32> = vec![1, 2];
    let expected: Vec<i32> = vec![1];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_test_case_ten() {
    let xs: Vec<i32> = vec![11, 12];
    let expected: Vec<i32> = vec![11, 12];
    let result: Vec<i32> = queens::at_least_one(xs);
    assert_eq!(result, expected);
}
