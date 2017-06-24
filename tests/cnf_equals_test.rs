extern crate dpll;

use dpll::backtracking::cnf as cnf;

#[test]
fn test_cnf_equals_one() {
    assert!(cnf::equals(vec![vec![]], vec![vec![]]));
}

#[test]
fn test_cnf_equals_two() {
    assert!(cnf::equals(vec![vec![1]], vec![vec![1]]));
}

#[test]
fn test_cnf_equals_three() {
    assert!(cnf::equals(vec![vec![1], vec![2]], vec![vec![1], vec![2]]));
}

#[test]
fn test_cnf_equals_four() {
    assert!(cnf::equals(vec![vec![1, 3], vec![2]], vec![vec![1, 3], vec![2]]));
}

#[test]
fn test_cnf_equals_five() {
    assert!(cnf::equals(vec![ vec![2], vec![1, 3]], vec![vec![1, 3], vec![2]]));
}

#[test]
#[should_panic]
fn test_cnf_equals_six() {
    assert!(cnf::equals(vec![ vec![2], vec![1, 5]], vec![vec![1, 3], vec![2]]));
}
