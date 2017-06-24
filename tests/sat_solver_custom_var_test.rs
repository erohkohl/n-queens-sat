extern crate dpll;

use dpll::backtracking::sat as solver;

/*
#[test]
fn sat_custom_var_order_case_one() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _):(bool, Vec<i32>) = solver::dpll(vec![vec![23]], vec);
    assert!(res_bool);
}

#[test]
fn model_custom_var_order_case_one() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (_, model):(bool, Vec<i32>) = solver::dpll(vec![vec![23]], vec);
    let expected:Vec<i32> = vec![23];
    assert_eq!(model, expected);
}

#[test]
fn sat_custom_var_order_case_two() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _):(bool, Vec<i32>) = solver::dpll(vec![vec![12, 23]], vec);
    assert!(res_bool);
}

#[test]
fn model_custom_var_order_case_two() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (_, model):(bool, Vec<i32>) = solver::dpll(vec![vec![12, 23]], vec);
    let expected:Vec<Vec<i32>> = vec![vec![12], vec![23], vec![12, 23]];
    assert!(expected.contains(&model));
}

#[test]
fn sat_custom_var_order_case_three() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _):(bool, Vec<i32>) = solver::dpll(vec![vec![12, 23], vec![-12, 23]], vec);
    assert!(res_bool);
}
*/
/*
#[test]
fn model_custom_var_order_case_three() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (_, model):(bool, Vec<i32>) = solver::dpll(vec![vec![12, 23], vec![-12, 23]], vec);
    let expected:Vec<Vec<i32>> = vec![vec![23], vec![12, 23], vec![-12, 23]];
    assert!(expected.contains(&model));
}

#[test]
fn sat_custom_var_order_case_four() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (res_bool, _):(bool, Vec<i32>) = solver::dpll(
        vec![
            vec![-23, -12]
            , vec![12, -34]
        ], vec);
    assert!(res_bool);
}

#[test]
fn model_custom_var_order_case_four() {
    let number_of_var:usize = 1;
    let vec: Vec<i32> = Vec::with_capacity(number_of_var);
    let (_, model):(bool, Vec<i32>) = solver::dpll(
        vec![
            vec![-23, -12]
            , vec![12, -34]
        ], vec);
    let expected:Vec<Vec<i32>> = vec![vec![-34, -12]];
    assert!(expected.contains(&model));
}*/