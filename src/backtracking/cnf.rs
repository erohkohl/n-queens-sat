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
