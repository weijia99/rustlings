fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let  vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());
        // 已经传递给下一个vec了，无法使用，所以你需要复制
        // When we pass `vec0` into `fill_vec`, it's
        // being "moved" into `vec1`, meaning we can't access
        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
