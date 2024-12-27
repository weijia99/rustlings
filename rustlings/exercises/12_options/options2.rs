fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);
        // 一般就是用来进行统计some，some可能会出现none，some，用match需要多个匹配
        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        while let Some(integer) = optional_integers.pop() {
            // 因为使用pop会自带一个option，需要使用some来进行解开，只有值才可以进行使用
            if let Some(integer) = integer{
                assert_eq!(integer, cursor);
                cursor -= 1;
            }
            
        }

        assert_eq!(cursor, 0);
    }
}
