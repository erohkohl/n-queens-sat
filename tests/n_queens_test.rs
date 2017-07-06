extern crate dpll;

use std::collections::HashSet;

use dpll::modelling::n_queens as queens;
use dpll::backtracking::sat as solver;

#[test]
fn at_least_one_queen_in_every_row_case_one(){
    let n = 2;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![11, 12]);
    expected.insert(vec![21, 22]);

    let result:HashSet<Vec<i32>> = queens::at_least_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_row_case_two(){
    let n = 4;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![11, 12, 13, 14]);
    expected.insert(vec![21, 22, 23, 24]);
    expected.insert(vec![31, 32, 33, 34]);
    expected.insert(vec![41, 42, 43, 44]);

    let result:HashSet<Vec<i32>> = queens::at_least_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_row_case_three(){
    let n = 8;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![11, 12, 13, 14, 15, 16, 17, 18]);
    expected.insert(vec![21, 22, 23, 24, 25, 26, 27, 28]);
    expected.insert(vec![31, 32, 33, 34, 35, 36, 37, 38]);
    expected.insert(vec![41, 42, 43, 44, 45, 46, 47, 48]);
    expected.insert(vec![51, 52, 53, 54, 55, 56, 57, 58]);
    expected.insert(vec![61, 62, 63, 64, 65, 66, 67, 68]);
    expected.insert(vec![71, 72, 73, 74, 75, 76, 77, 78]);
    expected.insert(vec![81, 82, 83, 84, 85, 86, 87, 88]);

    let result:HashSet<Vec<i32>> = queens::at_least_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_column_case_one(){
    let n = 2;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![11, 21]);
    expected.insert(vec![12, 22]);

    let result:HashSet<Vec<i32>> = queens::at_least_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_column_case_two(){
    let n = 4;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![11, 21, 31, 41]);
    expected.insert(vec![12, 22, 32, 42]);
    expected.insert(vec![13, 23, 33, 43]);
    expected.insert(vec![14, 24, 34, 44]);

    let result:HashSet<Vec<i32>> = queens::at_least_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_column_case_three(){
    let n = 8;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![11, 21, 31, 41, 51, 61, 71, 81]);
    expected.insert(vec![12, 22, 32, 42, 52, 62, 72, 82]);
    expected.insert(vec![13, 23, 33, 43, 53, 63, 73, 83]);
    expected.insert(vec![14, 24, 34, 44, 54, 64, 74, 84]);
    expected.insert(vec![15, 25, 35, 45, 55, 65, 75, 85]);
    expected.insert(vec![16, 26, 36, 46, 56, 66, 76, 86]);
    expected.insert(vec![17, 27, 37, 47, 57, 67, 77, 87]);
    expected.insert(vec![18, 28, 38, 48, 58, 68, 78, 88]);

    let result:HashSet<Vec<i32>> = queens::at_least_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_row_case_one(){
    let n = 2;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-11, -12]);
    expected.insert(vec![-21, -22]);
    let result:HashSet<Vec<i32>> = queens::at_most_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_row_case_two(){
    let n = 4;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-11, -12]);
    expected.insert(vec![-11, -13]);
    expected.insert(vec![-11, -14]);
    expected.insert(vec![-12, -13]);
    expected.insert(vec![-12, -14]);
    expected.insert(vec![-13, -14]);

    expected.insert(vec![-21, -22]);
    expected.insert(vec![-21, -23]);
    expected.insert(vec![-21, -24]);
    expected.insert(vec![-22, -23]);
    expected.insert(vec![-22, -24]);
    expected.insert(vec![-23, -24]);

    expected.insert(vec![-31, -32]);
    expected.insert(vec![-31, -33]);
    expected.insert(vec![-31, -34]);
    expected.insert(vec![-32, -33]);
    expected.insert(vec![-32, -34]);
    expected.insert(vec![-33, -34]);

    expected.insert(vec![-41, -42]);
    expected.insert(vec![-41, -43]);
    expected.insert(vec![-41, -44]);
    expected.insert(vec![-42, -43]);
    expected.insert(vec![-42, -44]);
    expected.insert(vec![-43, -44]);

    let result:HashSet<Vec<i32>> = queens::at_most_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_column_case_one(){
    let n = 2;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-11, -21]);
    expected.insert(vec![-12, -22]);

    let result:HashSet<Vec<i32>> = queens::at_most_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_column_case_two(){
    let n = 4;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-11, -21]);
    expected.insert(vec![-11, -31]);
    expected.insert(vec![-11, -41]);
    expected.insert(vec![-21, -31]);
    expected.insert(vec![-21, -41]);
    expected.insert(vec![-31, -41]);

    expected.insert(vec![-12, -22]);
    expected.insert(vec![-12, -32]);
    expected.insert(vec![-12, -42]);
    expected.insert(vec![-22, -32]);
    expected.insert(vec![-22, -42]);
    expected.insert(vec![-32, -42]);

    expected.insert(vec![-13, -23]);
    expected.insert(vec![-13, -33]);
    expected.insert(vec![-13, -43]);
    expected.insert(vec![-23, -33]);
    expected.insert(vec![-23, -43]);
    expected.insert(vec![-33, -43]);

    expected.insert(vec![-14, -24]);
    expected.insert(vec![-14, -34]);
    expected.insert(vec![-14, -44]);
    expected.insert(vec![-24, -34]);
    expected.insert(vec![-24, -44]);
    expected.insert(vec![-34, -44]);

    let result:HashSet<Vec<i32>> = queens::at_most_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_diagonal_case_one(){
    let n = 2;
    let mut expected:HashSet<Vec<i32>> = HashSet::new();
    expected.insert(vec![-12, -21]);
    expected.insert(vec![-11, -22]);

    let result:HashSet<Vec<i32>> = queens::at_most_one_queen_in_every_diagonal(n);
    assert_eq!(result, expected);
}


#[test]
fn generate_cnf_case_one(){
    let n:usize = 4;
    let cnf:HashSet<Vec<i32>> = queens::generate_cnf(n);
    //println!("{:?}", cnf);

    let part_assign: Vec<i32> = Vec::with_capacity(n * n);
    let (is_sat, model):(bool, Vec<i32>) = solver::dpll(cnf, part_assign);
    println!("{:?}", model);
    assert!(is_sat);
}
