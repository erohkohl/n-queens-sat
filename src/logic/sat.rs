use std::collections::HashSet;

/// This method provides an additional abstraction layer between the caller of sat solver and
/// the DPLL algorithm. Therefore it initializes the partial assignment with an empty vec and
/// delegates the call to the DPLL algorithm.
pub fn solve(cnf: HashSet<Vec<i32>>, number_variables: usize) -> (bool, Vec<i32>) {
    let part_assign: Vec<i32> = Vec::with_capacity(number_variables);
    return dpll(cnf, part_assign);
}


///   DAVIS, PUTNAM, LOGEMANN, LOVELAND
///   Algorithm first runs complete boolean unit propagation. Second it decides the first
///   variable of the first clausel and spawns a new branch in our logic tree.
pub fn dpll(mut cnf: HashSet<Vec<i32>>, mut partial_assignment: Vec<i32>) -> (bool, Vec<i32>) {
    if !unit_propagation(&mut cnf, &mut partial_assignment) {
        // in case cnf is not satisfiable return empty model
        return (false, vec![]);
    } else {
        let next_clause: Vec<i32>;

        match cnf.iter().next() {
            Some(x) => next_clause = x.to_vec(),
            None => return (true, partial_assignment),
        }
        let mut lit: i32 = 0i32;
        if !next_clause.to_vec().is_empty() {
            lit = next_clause.to_vec().pop().unwrap();
        }

        if is_lit_conflict_with_part_assign(&partial_assignment, &lit) {
            return (false, vec![]);
        } else {
            let mut left: Vec<i32> = partial_assignment.clone();
            let mut right: Vec<i32> = partial_assignment.clone();
            left.push(lit);
            right.push(lit * (-1));

            let (left_bool, left_assign): (bool, Vec<i32>)
                = dpll(trim_cnf(cnf.clone(), lit), left);
            let (right_bool, right_assign): (bool, Vec<i32>)
                = dpll(trim_cnf(cnf, lit * (-1)), right);

            // makes new branching in our decision tree
            if left_bool {
                return (left_bool, left_assign);
            } else {
                return (right_bool, right_assign);
            }
        }
    }
}


///   Function applies partial assignment to Conjunctive Normalform, in the way that it
///   removes all clausels which will become true and eliminates all negated variable occurrences.
pub fn trim_cnf(cnf: HashSet<Vec<i32>>, decided_literal: i32) -> HashSet<Vec<i32>> {
    //  Make physical copy of cnf, otherwise it will be borrowed to the loop
    let mut trimed_cnf: HashSet<Vec<i32>> = cnf.clone();

    for clausel in &cnf {
        for literal in clausel {
            if *literal == decided_literal {
                trimed_cnf.remove(clausel);
            }
            if literal * (-1) == decided_literal && clausel.len() > 1 {
                let mut trimed_clausel = clausel.clone();
                trimed_cnf.remove(clausel);
                trimed_clausel.retain(|x| x + 0 != *literal);
                trimed_cnf.insert(trimed_clausel.to_vec());
            }
        }
    }
    return trimed_cnf;
}


///   Function calculates the boolean constraint propagation for a given partial assignment
///   and returns false, if the propagation lead to a conflict.
pub fn unit_propagation(cnf: &mut HashSet<Vec<i32>>, partial_assignment: &mut Vec<i32>) -> bool {
    // Apply partial assignment to conjunctive normal form.
    for lit in partial_assignment.to_vec() {
        if !is_lit_conflict_with_part_assign(&partial_assignment, &lit) {
            *cnf = trim_cnf(cnf.clone(), lit);
        } else {
            return false;
        }
    }

    let cnf_clone: HashSet<Vec<i32>> = cnf.clone();

    for clausel in cnf_clone.iter() {
        // In case clause has just one literal, this lit must be unit.
        if clausel.len() == 1 && !cnf.is_empty() {
            let is_cnf_empty: Option<i32> = clausel.to_vec().pop();
            let lit: i32;
            match is_cnf_empty {
                Some(unit_lit) => lit = unit_lit,
                None => return true,
            }

            if is_lit_conflict_with_part_assign(&partial_assignment, &lit) {
                return false;
            } else {
                partial_assignment.push(lit);
                *cnf = trim_cnf(cnf.clone(), lit);
                if !cnf.is_empty() {
                    unit_propagation(cnf, partial_assignment);
                }
            }
        }
    }
    return true;
}


/// Function checks whether one decided literal is conflict with actual partial assignment.
fn is_lit_conflict_with_part_assign(partial_assignment: &Vec<i32>, literal: &i32) -> bool {
    for j in partial_assignment.to_vec() {
        if (*literal) * -1 == j {
            return true;
        }
    }
    return false;
}