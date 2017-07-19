use std::collections::HashSet;

#[test]
fn hash_set_equals_case_one() {
    let mut cnf_one: HashSet<Vec<i32>> = HashSet::new();
    let mut cnf_two: HashSet<Vec<i32>> = HashSet::new();

    let clausel_one: Vec<i32> = vec![12, 34];
    let clausel_two: Vec<i32> = vec![11, 45];

    cnf_one.insert(clausel_one.to_vec());
    cnf_one.insert(clausel_two.to_vec());

    cnf_two.insert(clausel_two.to_vec());
    cnf_two.insert(clausel_one.to_vec());

    assert_eq!(cnf_one, cnf_two);
}

#[test]
#[should_panic]
fn vec_equals_case_one() {
    let mut cnf_one: Vec<Vec<i32>> = Vec::new();
    let mut cnf_two: Vec<Vec<i32>> = Vec::new();

    let clausel_one: Vec<i32> = vec![12, 34];
    let clausel_two: Vec<i32> = vec![11, 45];

    cnf_one.push(clausel_one.to_vec());
    cnf_one.push(clausel_two.to_vec());

    cnf_two.push(clausel_two.to_vec());
    cnf_two.push(clausel_one.to_vec());

    assert_eq!(cnf_one, cnf_two);
}