// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec:  Vec<i32>) -> Vec<i32> {
    // 一般是对对象名称加mut
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
