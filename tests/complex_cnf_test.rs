extern crate dpll;

use dpll::backtracking::sat as solver;

/*
#[test]
fn test_complex_cnf_case_one() {
    let number_of_var:usize = 5;
    let partial_assignment:Vec<i32> = Vec::with_capacity(number_of_var);
    let cnf:Vec<Vec<i32>> = vec![
                                vec![1, 2, 3],
                                vec![1, 2, 4],
                                vec![1, 2, -4],
                                vec![1, 5, -2],
                                vec![1, -3, -4],
                                vec![2, 3, -4],
                                vec![2, 5, 4],
                                vec![3, 5, -2],
                                vec![3, 5, -4],
                                vec![3, -2, -5],
                                vec![5, 4, -1],
                                vec![4, -1, -3],
                                vec![4, -1, -3],
                                vec![4, -2, -5],
                                vec![-1, -2, -5],
                                vec![-1, -3, -5],
                                vec![-2, -5, -4],
                                vec![-3, -5, -4],
                            ];
    let (res_bool, _ ):(bool, Vec<i32>) = solver::dpll(cnf, partial_assignment);
    assert!(res_bool);
}

#[test]
#[should_panic]
fn test_complex_cnf_case_two() {
    let number_of_var:usize = 5;
    let partial_assignment:Vec<i32> = Vec::with_capacity(number_of_var);
    let cnf:Vec<Vec<i32>> = vec![
                                vec![1, 2, 3],
                                vec![-1],
                                vec![-3],
                                vec![-2],
                                vec![1, 2, 4],
                                vec![1, 2, -4],
                                vec![1, 5, -2],
                                vec![1, -3, -4],
                                vec![2, 3, -4],
                                vec![2, 5, 4],
                                vec![3, 5, -2],
                                vec![3, 5, -4],
                                vec![3, -2, -5],
                                vec![5, 4, -1],
                                vec![4, -1, -3],
                                vec![4, -1, -3],
                                vec![4, -2, -5],
                                vec![-1, -2, -5],
                                vec![-1, -3, -5],
                                vec![-2, -5, -4],
                                vec![-3, -5, -4],
                            ];
    let (res_bool, _ ):(bool, Vec<i32>) = solver::dpll(cnf, partial_assignment);
    assert!(res_bool);
}
*/