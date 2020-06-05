
pub mod adders {

    pub fn half_adder(a: i32, b: i32) -> (i32, i32) {
        if a + b == 2 {
            let sum = 1;
            let carry = 1;
            return (sum, carry)
        }
        else {
            return (0, 0)
        }

    }
}
