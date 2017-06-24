extern crate dpll;

use dpll::modelling::n_queens as queens;
use dpll::backtracking::sat as solver;

/*
#[test]
fn at_least_one_queen_in_every_row_case_one(){
    let n = 2;
    let expected:Vec<Vec<i32>> = vec![vec![11, 12], vec![21, 22]];
    let result:Vec<Vec<i32>> = queens::at_least_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_row_case_two(){
    let n = 4;
    let expected:Vec<Vec<i32>> =
        vec![vec![11, 12, 13, 14], vec![21, 22, 23, 24], vec![31, 32, 33, 34], vec![41, 42, 43, 44]];
    let result:Vec<Vec<i32>> = queens::at_least_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_row_case_three(){
    let n = 8;
    let expected:Vec<Vec<i32>> =
            vec![
                vec![11, 12, 13, 14, 15, 16, 17, 18]
                ,vec![21, 22, 23, 24, 25, 26, 27, 28]
                ,vec![31, 32, 33, 34, 35, 36, 37, 38]
                ,vec![41, 42, 43, 44, 45, 46, 47, 48]
                ,vec![51, 52, 53, 54, 55, 56, 57, 58]
                ,vec![61, 62, 63, 64, 65, 66, 67, 68]
                ,vec![71, 72, 73, 74, 75, 76, 77, 78]
                ,vec![81, 82, 83, 84, 85, 86, 87, 88]
            ];
    let result:Vec<Vec<i32>> = queens::at_least_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_column_case_one(){
    let n = 2;
    let expected:Vec<Vec<i32>> =
        vec![vec![11, 21], vec![12, 22]];
    let result:Vec<Vec<i32>> = queens::at_least_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_column_case_two(){
    let n = 4;
    let expected:Vec<Vec<i32>> =
        vec![vec![11, 21, 31, 41], vec![12, 22, 32, 42], vec![13, 23, 33, 43], vec![14, 24, 34, 44]];
    let result:Vec<Vec<i32>> = queens::at_least_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_least_one_queen_in_every_column_case_three(){
    let n = 8;
    let expected:Vec<Vec<i32>> =
        vec![
             vec![11, 21, 31, 41, 51, 61, 71, 81]
            ,vec![12, 22, 32, 42, 52, 62, 72, 82]
            ,vec![13, 23, 33, 43, 53, 63, 73, 83]
            ,vec![14, 24, 34, 44, 54, 64, 74, 84]
            ,vec![15, 25, 35, 45, 55, 65, 75, 85]
            ,vec![16, 26, 36, 46, 56, 66, 76, 86]
            ,vec![17, 27, 37, 47, 57, 67, 77, 87]
            ,vec![18, 28, 38, 48, 58, 68, 78, 88]
        ];
    let result:Vec<Vec<i32>> = queens::at_least_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_row_case_one(){
    let n = 2;
    let expected:Vec<Vec<i32>> = vec![vec![-11, -12], vec![-21, -22]];
    let result:Vec<Vec<i32>> = queens::at_most_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_row_case_two(){
    let n = 4;
    let expected:Vec<Vec<i32>> =
        vec![
            vec![-11, -12], vec![-11, -13], vec![-11, -14]
            , vec![-12, -13], vec![-12, -14]
            , vec![-13, -14]

            , vec![-21, -22], vec![-21, -23], vec![-21, -24]
            , vec![-22, -23], vec![-22, -24]
            , vec![-23, -24]

            , vec![-31, -32], vec![-31, -33], vec![-31, -34]
            , vec![-32, -33], vec![-32, -34]
            , vec![-33, -34]

            , vec![-41, -42], vec![-41, -43], vec![-41, -44]
            , vec![-42, -43], vec![-42, -44]
            , vec![-43, -44]

        ];
    let result:Vec<Vec<i32>> = queens::at_most_one_queen_in_every_row(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_column_case_one(){
    let n = 2;
    let expected:Vec<Vec<i32>> = vec![vec![-11, -21], vec![-12, -22]];
    let result:Vec<Vec<i32>> = queens::at_most_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_column_case_two(){
    let n = 4;
    let expected:Vec<Vec<i32>> =
        vec![
            vec![-11, -21], vec![-11, -31], vec![-11, -41]
            , vec![-21, -31], vec![-21, -41]
            , vec![-31, -41]

            , vec![-12, -22], vec![-12, -32], vec![-12, -42]
            , vec![-22, -32], vec![-22, -42]
            , vec![-32, -42]

            , vec![-13, -23], vec![-13, -33], vec![-13, -43]
            , vec![-23, -33], vec![-23, -43]
            , vec![-33, -43]

            , vec![-14, -24], vec![-14, -34], vec![-14, -44]
            , vec![-24, -34], vec![-24, -44]
            , vec![-34, -44]

        ];
    let result:Vec<Vec<i32>> = queens::at_most_one_queen_in_every_column(n);
    assert_eq!(result, expected);
}

#[test]
fn at_most_one_queen_in_every_diagonal_case_one(){
    let n = 2;
    let expected:Vec<Vec<i32>> = vec![vec![-11, -21], vec![-12, -22]];
    let result:Vec<Vec<i32>> = queens::at_most_one_queen_in_every_diagonal(n);
    assert_eq!(result, expected);
}

#[test]
fn generate_cnf_case_one(){
    let n:usize = 4;
    let cnf:Vec<Vec<i32>> = queens::generate_cnf(n);
    println!("{:?}", cnf);

    let part_assign: Vec<i32> = Vec::with_capacity(n * n);
    let (is_sat, model):(bool, Vec<i32>) = solver::dpll(cnf, part_assign);
    assert!(is_sat);
}
*/