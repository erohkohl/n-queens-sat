use std::collections::HashSet;

pub fn equals(mut a:Vec<Vec<i32>>, mut b:Vec<Vec<i32>>) -> bool{
    if a.is_empty() && b.is_empty(){
        return true;
    }
    for i in 0..a.len(){
        if a[0] == b[i]{
            a.remove(0);
            b.remove(i);
            return equals(a, b);
        };
    }
    return false;
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
