struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
     red:u32,
     green:u32,
     blue:u32,

}

struct ColorTupleStruct(/* TODO: Add the fields that the test `tuple_structs` expects */
u32,u32,u32);

#[derive(Debug)]
struct UnitStruct;
// 直接实现了接口方法
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green =ColorRegularStruct{red:0,green:255,blue:0};

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green =ColorTupleStruct(0,255,0);
        // 使用元祖初始化和java差不多，但是没有树形
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct =UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}