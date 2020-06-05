
pub mod adders {

    pub fn half_adder(a: i32, b: i32) -> (i32, i32) {
        if a + b == 2 {
            return (1, 1)
        }
        else if a == 1 && b == 0 {
            return (1, 0)
        }
        else if a == 0 && b  == 1 {
            return (1, 0)
        }
        else {
            return (0, 0)
        }


    }

}
