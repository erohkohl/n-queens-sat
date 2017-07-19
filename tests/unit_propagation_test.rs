extern crate dpll;

use std::collections::HashSet;

use dpll::logic::sat as sat;

#[test]
fn up_empyt_clause() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert!(cnf.is_empty());
}

#[test]
fn up_empyt_clauset_part_assig() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert_eq!(partial_assignment, vec![]);
}


#[test]
fn up_one_clausel_one_lit_cnf_empty() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert!(cnf.is_empty());
}

#[test]
fn up_one_clausel_one_lit_part_assig() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![1]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert_eq!(partial_assignment, vec![1]);
}


#[test]
fn up_one_clausel_two_lit_cnf_not_empty() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert!(!cnf.is_empty());
}

#[test]
fn up_one_clausel_two_lit_part_assig_empty() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert!(partial_assignment.is_empty());
}

#[test]
fn up_one_clausel_two_lit_cnf() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);

    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![2, 3]);
    assert_eq!(cnf, cnf_expected);
}


#[test]
fn up_two_clausel_two_lit_cnf_empty() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![-2]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert!(cnf.is_empty());
}

#[test]
fn up_two_clausel_two_lit_part_assig() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![-2]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert_eq!(partial_assignment, vec![-2, 3]);
}


#[test]
fn up_three_clausel_four_lit_cnf_not_empty() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![-2]);
    cnf.insert(vec![1, 4]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert!(!cnf.is_empty());
}

#[test]
fn up_three_clausel_four_lit_cnf() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![-2]);
    cnf.insert(vec![1, 4]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);

    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![1, 4]);
    assert_eq!(cnf, cnf_expected);
}

#[test]
fn up_three_clausel_four_lit_part_assig() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![-2]);
    cnf.insert(vec![1, 4]);
    let mut partial_assignment: Vec<i32> = vec![];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert_eq!(partial_assignment, vec![-2, 3]);
}


#[test]
fn up_three_clausel_five_lit_cnf_not_empty() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![1, -2]);
    cnf.insert(vec![3, 4, 5]);
    let mut partial_assignment: Vec<i32> = vec![-2];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert!(cnf.is_empty());
}

#[test]
fn up_three_clausel_five_lit_no_conflict() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![1, -2]);
    cnf.insert(vec![3, 4, 5]);
    let mut partial_assignment: Vec<i32> = vec![-2];
    let no_conflict = sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert!(no_conflict);
}

#[test]
fn up_three_clausel_five_lit_cnf() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![1, -2]);
    cnf.insert(vec![-3, 4, 5]);
    let mut partial_assignment: Vec<i32> = vec![-2];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);

    let mut cnf_expected: HashSet<Vec<i32>> = HashSet::new();
    cnf_expected.insert(vec![4, 5]);
    assert_eq!(cnf, cnf_expected);
}

#[test]
fn up_three_clausel_five_lit_part_assig() {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2, 3]);
    cnf.insert(vec![1, -2]);
    cnf.insert(vec![3, 4, 5]);
    let mut partial_assignment: Vec<i32> = vec![-2];
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    assert_eq!(partial_assignment, vec![-2, 3]);
}

#[test]
fn up_two_clausel_two_lit_trivial_part_assig() {
    let number_of_var: usize = 2;
    let mut partial_assignment: Vec<i32> = Vec::with_capacity(number_of_var);
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    cnf.insert(vec![2]);
    cnf.insert(vec![1]);
    sat::unit_propagation(&mut cnf, &mut partial_assignment);
    let assert: bool = partial_assignment == vec![1, 2] ||
        partial_assignment == vec![2, 1];
    assert!(assert);
}
