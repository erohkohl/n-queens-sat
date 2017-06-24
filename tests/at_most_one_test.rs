extern crate dpll;

use dpll::modelling::n_queens as queens;

#[test]
fn at_most_one_test_case_one() {
    let xs:Vec<i32> = vec![1];
    let expected:Vec<Vec<i32>> =
        vec![vec![1]];
    let result:Vec<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_test_case_two() {
    let xs:Vec<i32> = vec![1,2];
    let expected:Vec<Vec<i32>> =
        vec![vec![-1, -2]];
    let result:Vec<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_test_case_three() {
    let xs:Vec<i32> = vec![1,2,3];
    let expected:Vec<Vec<i32>> =
        vec![vec![-1, -2], vec![-1, -3], vec![-2, -3]];
    let result:Vec<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_test_case_four() {
    let xs:Vec<i32> = vec![1,2,3,4];
    let expected:Vec<Vec<i32>> =
        vec![vec![-1, -2], vec![-1, -3], vec![-1, -4], vec![-2, -3], vec![-2, -4], vec![-3, -4]];
    let result:Vec<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}


#[test]
fn at_most_one_test_case_five() {
    let xs:Vec<i32> = vec![1,4,5,6];
    let expected:Vec<Vec<i32>> =
        vec![vec![-1,-4], vec![-1, -5], vec![-1, -6], vec![-4, -5], vec![-4, -6], vec![-5, -6]];
    let result:Vec<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}


#[test]
#[should_panic]
fn at_most_one_test_case_six() {
    let xs:Vec<i32> = vec![1,2,3,4];
    let expected:Vec<Vec<i32>> =
        vec![vec![-1, -2], vec![-1, -3], vec![-2, -3], vec![-2, -4], vec![-3, -4]];
    let result:Vec<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn at_most_one_test_case_seven() {
    let xs:Vec<i32> = vec![1,2,3,4];
    let expected:Vec<Vec<i32>> =
        vec![vec![-1, -2], vec![-1, -3], vec![-2, -3], vec![-2, -4], vec![-3, -4]];
    let result:Vec<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn at_most_one_test_case_eight() {
    let xs:Vec<i32> = vec![1,2,3,4];
    let expected:Vec<Vec<i32>> =
        vec![vec![1, -2], vec![-1, -3], vec![-1, -4], vec![-2, -3], vec![-2, -4], vec![-3, -4]];
    let result:Vec<Vec<i32>> = queens::at_most_one(xs);
    assert_eq!(result, expected);
}