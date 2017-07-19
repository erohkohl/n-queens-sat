use std::collections::HashSet;

pub fn at_most_one(clause: Vec<i32>) -> HashSet<Vec<i32>> {
    if clause.len() == 1 {
        let mut min_clause: HashSet<Vec<i32>> = HashSet::new();
        min_clause.insert(vec![1]);
        return min_clause;
    } else {
        let mut clauseln_to_add: HashSet<Vec<i32>> = HashSet::new();

        for i in clause.iter() {
            for j in clause.iter() {
                if i < j {
                    let a: i32 = (*i as i32) * (-1);
                    let b: i32 = (*j as i32) * (-1);
                    clauseln_to_add.insert(vec![a, b]);
                }
            }
        }
        return clauseln_to_add;
    }
}


pub fn at_least_one(clause: Vec<i32>) -> Vec<i32> {
    let n = clause.len() + 1;
    let mut cnf: Vec<i32> = Vec::with_capacity(n);
    for i in clause.iter() {
        let a: i32 = *i as i32;
        cnf.push(a);
    }
    return cnf;
}


pub fn at_least_one_queen_in_every_row(n: usize) -> HashSet<Vec<i32>> {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    for i in 1..n + 1 {
        let mut clause_to_add: Vec<i32> = Vec::with_capacity(n);
        for j in 1..n + 1 {
            let var: i32 = (i as i32) * 10 + (j as i32);
            clause_to_add.push(var);
        }
        cnf.insert(at_least_one(clause_to_add));
    }
    return cnf;
}

pub fn at_least_one_queen_in_every_column(n: usize) -> HashSet<Vec<i32>> {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    for i in 1..n + 1 {
        let mut clause_to_add: Vec<i32> = Vec::with_capacity(2);
        for j in 1..n + 1 {
            let var: i32 = (j as i32) * 10 + (i as i32);
            clause_to_add.push(var);
        }
        cnf.insert(at_least_one(clause_to_add));
    }
    return cnf;
}


pub fn at_most_one_queen_in_every_row(n: usize) -> HashSet<Vec<i32>> {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            for k in 1..n + 1 {
                if j < k {
                    let mut clause_to_add: Vec<i32> = Vec::with_capacity(2);
                    let var_1: i32 = (i as i32) * 10 + (j as i32);
                    let var_2: i32 = (i as i32) * 10 + (k as i32);
                    clause_to_add.push(var_1);
                    clause_to_add.push(var_2);
                    for i in at_most_one(clause_to_add) {
                        cnf.insert(i);
                    }
                }
            }
        }
    }
    return cnf;
}


pub fn at_most_one_queen_in_every_column(n: usize) -> HashSet<Vec<i32>> {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    for j in 1..n + 1 {
        for i in 1..n + 1 {
            for k in 1..n + 1 {
                if i < k {
                    let mut clause_to_add: Vec<i32> = Vec::with_capacity(n);
                    let var_1: i32 = (i as i32) * 10 + (j as i32);
                    let var_2: i32 = (k as i32) * 10 + (j as i32);
                    clause_to_add.push(var_1);
                    clause_to_add.push(var_2);
                    for i in at_most_one(clause_to_add) {
                        cnf.insert(i);
                    }
                }
            }
        }
    }
    return cnf;
}


pub fn at_most_one_queen_in_every_diagonal(n: usize) -> HashSet<Vec<i32>> {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    for i in 1..n + 1 {
        for k in 1..n + 1 {
            for j in 1..n + 1 {
                for m in 1..n + 1 {
                    let diagonal_left: bool = (i as i32 - k as i32) == (m as i32 - j as i32);
                    let diagonal_right: bool = (i as i32 - k as i32) == (j as i32 - m as i32);

                    if 0 < i && i < k && k < n + 1 && (diagonal_left || diagonal_right) {
                        let mut clause_to_add: Vec<i32> = Vec::with_capacity(2);
                        let var_1: i32 = -1 * ((i as i32) * 10 + (j as i32));
                        let var_2: i32 = -1 * ((k as i32) * 10 + (m as i32));
                        clause_to_add.push(var_1);
                        clause_to_add.push(var_2);
                        clause_to_add.sort_by(|x, y| y.cmp(x));
                        cnf.insert(clause_to_add);
                    }
                }
            }
        }
    }
    return cnf;
}


pub fn generate_cnf(n: usize) -> HashSet<Vec<i32>> {
    let mut cnf: HashSet<Vec<i32>> = HashSet::new();
    for i in at_least_one_queen_in_every_column(n) { cnf.insert(i); };
    for i in at_least_one_queen_in_every_row(n) { cnf.insert(i); };
    for i in at_most_one_queen_in_every_row(n) { cnf.insert(i); };
    for i in at_most_one_queen_in_every_column(n) { cnf.insert(i); };
    for i in at_most_one_queen_in_every_diagonal(n) { cnf.insert(i); };
    return cnf;
}