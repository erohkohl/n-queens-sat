extern crate dpll;

#[test]
fn test_vec_retain() {
    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|x| x + 0 != 2);
    assert_eq!(vec, [1,3,4]);
}

#[test]
fn test_vec_eq() {
    let x = vec![1, 2];
    assert_eq!(x, [1, 2]);
}


#[test]
fn test_vec_retain_with_vec() {
    let mut vec = vec![vec![1, 2], vec![3, 4]];
    vec.retain(|x| x.to_vec() != [1, 2]);
    assert_eq!(vec, [[3,4]]);
}