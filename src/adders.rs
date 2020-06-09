pub mod adders {
    use std::cell::Cell;
    use std::option::Option;
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

    pub fn inc_16 (in_arr: [i32; 16]) -> [i32;16] {
        let mut out: [i32; 16] = [1; 16];
        for i in 0..16 {
            out[i] = in_arr[i] + 1
        }

        return out
    }

    #[derive(Debug, Clone)]
    pub struct ALU {
        // **** By using Cell<T>, you can emulate field-level mutability. ****
        // We instruct the ALU on which function to compute, by setting
        // six input bits, called control bits, to selected binary values.
        // Each control but instructs the ALU to carry out a function.
        // 6 operations => 2^6 = 64 possible function outputs.
        // So, let's take the function (x - 1):
            // The control bit sequence for this is: [0|0|1|1|1|0]
        // Because the zx, and nx bits ([0|0...]) are set to zero,
        // the x input is neither zeroed, nor negated.
        // The zy, and ny bits are both 1, so the y input is zeroed,
        // then negated bitwise.
        pub x: Vec<i32>, // 16 bit data inputs
        y: Vec<i32>, // 16 bit data inputs 
        // These bits toggle the x input
        pub zx: i32, // Zero x input
        nx: i32, // Negate x input
        // These bits toggle the y input
        zy: i32, // Zero y input
        ny: i32, // Negate x input
        // This toggles between And/Add
        f: i32, 
        // This bit instructs out to set out.
        no: i32, // negate output
    }

    impl ALU {
        fn new(x: Vec<i32>,
                y: Vec<i32>,
                zx: i32,
                nx: i32,
                zy: i32,
                ny: i32,
                f: i32,
                no: i32) -> ALU {
                ALU {    
                    x: x,
                    y: y,
                    zx: zx,
                    nx: nx,
                    zy: zy,
                    ny: ny,
                    f:  f,
                    no: no
                }
            }
        fn z_x(&self, zx: i32, x: [i32;16] ) {
            let mut x_vec = Vec::new();
            for i in 0..16 {x_vec.push(0)}
            let mut y_vec = x_vec.clone();            
            if zx == 1 {
                let x: ALU = Default::default();
                print!("ALU: {:?}", x);
            }
        }
    }
    impl Default for ALU {
        fn default() -> ALU {
            let mut x_vec = Vec::new();
            for i in 0..16 {x_vec.push(0)}
            let mut y_vec = x_vec.clone();
            
            ALU {
                x: x_vec,
                y: y_vec,
                zx: 0,
                nx: 0,
                zy: 0,
                ny: 0,
                f:  0,
                no: 0,
            }
        }
    }


}

