// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn fill_vec_with_poiner(vec: &mut Vec<i32>) -> &Vec<i32> {
    vec.push(999999);
    vec
}

fn main() {
    // You can optionally experiment here.
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0);
    // println!("{}", vec0[0]); // not valid, ownership moved to fill_vec
    println!("{}", vec1[0]);

    let mut vec3 = vec![22, 44, 66];
    let vec4 = fill_vec_with_poiner(&mut vec3);
    println!("mutable last: {}", vec4[3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
