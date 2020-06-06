
pub mod adders {
    /// Adds two bits, where the least significant bit of the addition is 
    /// called sum, and the most significant bit is called carry.
    ///
    /// #Examples
    ///
    /// ```
    /// let a = 1;
    /// let b = 1;
    /// let answer = half_adder(a, b); 
    /// assert_eq!(answer, (1, 0));
    /// 
    /// let c = 1
    /// let d = 0
    /// let answer_2 = half_adder(c, d)
    /// assert_eq!(answer_2, (0, 1))
    ///
    /// ```
    ///
    /// Values will be returned in a tuple, as (carry, sum) respectively.
    pub fn half_adder(a: i32, b: i32) -> (i32, i32) {
        if a == 1 && b == 1 {
            return (1, 0)
        }
        else if a == 1 && b == 0 {
            return (0, 1)
        }
        else if a == 0 && b  == 1 {
            return (0, 1)
        }
        else {
            return (0, 0)
        }


    }
    /// Adds three bits, where the least significant bit of the addition is 
    /// called sum, and the most significant bit is called carry.
    ///
    /// #Examples
    ///
    /// ```
    /// let a = 1;
    /// let b = 1;
    /// let c = 1
    /// let answer = full_adder(a, b, c); 
    /// assert_eq!(answer, (1, 1));
    /// let d = 0
    /// let e = 0
    /// let f = 1
    /// 
    /// let answer_2 = full_adder(d, e, f)
    /// assert_eq!(answer_2, (0, 1))
    ///
    /// ```
    ///
    /// Values will be returned in a tuple, as (carry, sum) respectively.
    pub fn full_adder(a: i32, b: i32, c: i32) -> (i32, i32) {
        if a == 0 && b == 0 && c == 0  {
            return (0, 0)
        }
        else if a == 0 && b == 0 && c == 1  {
            return (0, 1)
        }
        else if a == 0 && b == 1 && c == 0  {
            return (0, 1)
        }
        else if a == 0 && b == 1 && c == 1  {
            return (1, 0)
        }
        else if a == 1 && b == 0 && c == 0  {
            return (0, 1)
        }
        else if a == 1 && b == 0 && c == 1  {
            return (1, 0)
        }
        else if a == 1 && b == 1 && c == 0  {
            return (1, 0)
        }
        else if a == 1 && b == 1 && c == 1  {
            return (1, 1)
        }
        else {
            panic!("Invalid input")
        }



    }

    pub fn add_16(array_a: [i32; 16], array_b: [i32; 16]) -> [i32;16] {
        let mut out: [i32; 16] = [1; 16];

        for i in 0..16 {
            out[i] = half_adder(array_a[i], array_b[i]).1
        }

        return out
    }

}