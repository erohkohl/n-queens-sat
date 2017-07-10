use std::collections::HashSet;

pub fn equals(mut a:HashSet<Vec<i32>>, mut b:HashSet<Vec<i32>>) -> bool{
    return a.eq(&b) && b.eq(&a);
}

/// Returns true if and only if a possible model solves a cnf.
pub fn check_model(mut cnf:HashSet<Vec<i32>>, model:Vec<i32>) -> bool{
    let mut cnf_clone:HashSet<Vec<i32>> = cnf.clone();

    for lit in &model{
        for clause in &cnf{
            if clause.contains(lit) {
                cnf_clone.remove(clause);
            }
        }
    }
    return cnf_clone.is_empty();
}
