use std::collections::HashSet;

pub fn at_most_one(xs:Vec<i32>) -> HashSet<Vec<i32>>{
    if xs.len() == 1{
        let mut min_clause:HashSet<Vec<i32>> = HashSet::new();
        min_clause.insert(vec![1]);
        return min_clause;

    }else {
        let number_prop_var = xs.len() + 1;
        let mut clauseln_to_add:HashSet<Vec<i32>> = HashSet::new();

        for i in xs.iter() {
            for j in xs.iter() {
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


pub fn at_least_one(xs:Vec<i32>) -> Vec<i32>{
    let n = xs.len() + 1;
    let mut cnf:Vec<i32> = Vec::with_capacity(n);
    for i in xs.iter(){
        let a:i32 = *i as i32;
        cnf.push(a);
    }
    return cnf;
}


pub fn at_least_one_queen_in_every_row(n:usize) -> HashSet<Vec<i32>>{
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    for i in 1..n + 1{
        let mut xs:Vec<i32> = Vec::with_capacity(n);
        for j in 1..n + 1 {
            let var: i32 = (i as i32) * 10 + (j as i32);
            xs.push(var);
        }
        cnf.insert(at_least_one(xs));
    }
    return cnf;
}

pub fn at_least_one_queen_in_every_column(n:usize) -> HashSet<Vec<i32>>{
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    for i in 1..n + 1{
        let mut xs:Vec<i32> = Vec::with_capacity(n);
        for j in 1..n + 1 {
            let var: i32 = (j as i32) * 10 + (i as i32);
            xs.push(var);
        }
        cnf.insert(at_least_one(xs));
    }
    return cnf;
}


pub fn at_most_one_queen_in_every_row(n:usize) -> HashSet<Vec<i32>>{
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            for k in 1..n + 1 {
                if j < k {
                    let mut xs: Vec<i32> = Vec::with_capacity(n);
                    let var_1: i32 = (i as i32) * 10 + (j as i32);
                    let var_2: i32 = (i as i32) * 10 + (k as i32);
                    xs.push(var_1);
                    xs.push(var_2);
                    for i in at_most_one(xs){
                        cnf.insert(i);
                    }
                }
            }
        }
    }
    return cnf;
}


pub fn at_most_one_queen_in_every_column(n:usize) -> HashSet<Vec<i32>>{
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    for j in 1..n + 1 {
        for i in 1..n + 1 {
            for k in 1..n + 1 {
                if i < k {
                    let mut xs: Vec<i32> = Vec::with_capacity(n);
                    let var_1: i32 = (i as i32) * 10 + (j as i32);
                    let var_2: i32 = (k as i32) * 10 + (j as i32);
                    xs.push(var_1);
                    xs.push(var_2);
                    for i in at_most_one(xs){
                        cnf.insert(i);
                    }
                }
            }
        }
    }
    return cnf;
}


pub fn at_most_one_queen_in_every_diagonal(n:usize) -> HashSet<Vec<i32>>{
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            for k in 1..n + 1 {
                for m in 1..n + 1 {
                    if i != k && j != m  {
                        if i + k == j + m || i as i32 - k as i32 == j as i32 - m as i32 {
                            let mut xs: Vec<i32> = Vec::with_capacity(n);
                            let var_1: i32 = (i as i32) * 10 + (j as i32);
                            let var_2: i32 = (k as i32) * 10 + (m as i32);
                            xs.push(var_1);
                            xs.push(var_2);
                            for clause in at_most_one(xs){
                                cnf.insert(clause);
                            }
                        }
                    }
                }
            }
        }
    }
    return cnf;
}


pub fn generate_cnf(n:usize) -> HashSet<Vec<i32>>{
    let mut cnf:HashSet<Vec<i32>> = HashSet::new();
    for i in at_least_one_queen_in_every_column(n){cnf.insert(i);};
    for i in at_least_one_queen_in_every_row(n){cnf.insert(i);};
    for i in at_most_one_queen_in_every_row(n){cnf.insert(i);};
    for i in at_most_one_queen_in_every_column(n){cnf.insert(i);};
    for i in at_most_one_queen_in_every_diagonal(n){cnf.insert(i);};
    return cnf;
}