pub mod ALU {
    use std::cell::Cell;
    use std::option::Option;
    use crate::l_g::l_g::{nand,
                          or_nand,
                          and,
                          and_16,
                          or, 
                          or_16,
                          or_8w,
                          mux,
                          mux_16,
                          mux_16_4w,
                          mux_16_8w,
                          dmux,
                          dmux_4w,
                          dmux_8w,
                          xor,
                          not,
                          not_16
    };

    pub fn half_adder(a: i32, b: i32) -> (i32, i32) {
        // returns a tuple of the order (carry, sum), not accounting for overflow.
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

    pub fn full_adder(a: i32, b: i32, c: i32) -> (i32, i32) {
        // returns a tuple of the order (carry, sum), not accounting for overflow.
        // We have to re-write this.
        // Update: we don't, if a HOF can use it properly. ^_-

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

    pub fn add_16(array_a: [i32; 16], array_b: [i32; 16]) -> Vec<i32> {
        // Performs bitwise addition on two input arrays, returning a mutable vector of
        // sum bits, not accounting for overflow in terms of sum, but preventing
        // what would be a real stack overflow in a CPU.
        // Solved
            // We can generate the bitwise addition and the carries,
            // the problem is how to fold those carries back into the string of
            // bits efficiently,
        let mut out = Vec::new();
        let mut carry = 0;
        let mut result: Vec<i32> = Vec::new();
        for i in 0..16 {

            if array_a[i] + array_b[i] == 2 {
                carry = 1;
                out.push(full_adder(array_a[i], array_b[i], carry).1);

            }
            else if array_a[i] + array_b[i] == 1 {
                carry = 0;
                out.push(full_adder(array_a[i], array_b[i], carry).1);
            }
            else {
                carry = 0;
                out.push(full_adder(array_a[i], array_b[i], carry).1);
            }
        }

        return out
    }

    pub fn inc_16 (in_arr: [i32; 16]) -> [i32;16] {
        // Incrementing a 16 bit input array, not accounting for overflow.
        let mut out: [i32; 16] = [1; 16];
        for i in 0..16 {
            out[i] = in_arr[i] + 1
        }

        return out
    }

    #[derive(Debug, Clone)]
    pub struct ALU {
        // We instruct the ALU on which function to compute, by setting
        // six input bits, called control bits, to selected binary values.
        // Each control but instructs the ALU to carry out a function.
        // 6 operations => 2^6 = 64 possible function outputs.
        // The Control bits, and their bo3olean value,
        // Have to map onto the expected selector bit values.
        pub  x: [i32;16], // 16 bit data inputs
        pub  y: [i32;16], // 16 bit data inputs 
        // These bits toggle the x input
        pub zx: i32, // Zero x input
        pub nx: i32, // Negate x input
        // These bits toggle the y input
        pub zy: i32, // Zero y input
        pub ny: i32, // Negate x input
        pub f: i32, // Function code, 1 => add, 0 => and
        pub no: i32, // negate output
        pub ng: i32, // true iff out != [0;16], false otherwise
        pub zr: i32, // true iff out = [0;16], false otherwise
        pub otpt: [i32;16] // 16 bit output
    }

    impl ALU {
       pub fn new( x:  [i32;16], // 16 bit data inputs 
                y: [i32;16],  // 16 bit data inputs
                 // These bits toggle the x input
                zx: i32, // Zero x input
                nx: i32, // Negate x input
                zy: i32, // Zero y input
                ny: i32, // Negate y input
                f: i32, // Toggles bit for add/and
                no: i32, // Negate output
                ng: i32,
                zr: i32,
                otpt: [i32;16]) -> ALU {
                ALU {    
                    x: x,
                    y: y,
                    zx: zx,
                    nx: nx,
                    zy: zy,
                    ny: ny,
                    f:  f,
                    no: no,
                    ng: ng,
                    zr: zr,
                    otpt: otpt,
                }
            }
       pub fn z_x(&mut self, x: [i32;16]) {
            //  zero the x input according to the status
            // of the zx control bit.
            let mut y: [i32; 16] = [0; 16];
            let mut op_done = false;
            while op_done != true {
                if 0i32 != self.zx {
                    break;
                }
                else {
                    for i in 0..16 {
                        self.x[i] = 0;
                    }
                    op_done = true;
                }
            }
        }

       pub fn n_x(&mut self, x: [i32; 16]) {
            //  not the x input according to the status
            // of the nx control bit.
            let mut op_done = false;
            while op_done != true {
                if 0i32 != self.zx {
                    break;
                }
                else {
                    self.x = not_16(x);
                    op_done = true;
                }
            }
        }

        pub fn z_y(&mut self, y: [i32;16]) {
            //  zero the y input according to the status
            // of the zy control bit.
            let mut op_done = false;
            while op_done != true {
                if 1i32 != self.zy {
                    break;
                }
                else {
                    for i in 0..16 {
                        self.y[i] = 0;
                    }
                    op_done = true;
                }
            }
        }
        pub fn ny(&mut self, y: [i32; 16]) {
            //  not the y input according to the status
            // of the ny control bit.
            let mut op_done = false;
            while op_done != true {
                if 0i32 != self.ny {
                    break;
                }
                else {
                    self.y = not_16(y);
                    op_done = true;
                }
            }
        }
        pub fn _f (&mut self) -> [i32;16] {
            // Performs an operation according to the function control bit f.
            let mut out: [i32;16] = [0;16];
            if 1i32 == self.f {
                for i in 0..16 {
                    let mut carry = 0;
                    if self.x[i] + self.y[i] == 2 {
                        carry = 1;
                        out[i] = full_adder(self.x[i], self.y[i], carry).1;
        
                    }
                    else if self.x[i] + self.y[i] == 1 {
                        carry = 0;
                        out[i] = full_adder(self.x[i], self.y[i], carry).1;
                    }
                    else {
                        carry = 0;
                        out[i] = full_adder(self.x[i], self.y[i], carry).1;
                    }
                    self.otpt = out;
                }
            }
            else {
                for i in 0..16 {
                    out[i] = and(self.x[i], self.y[i]);
                    self.otpt = out;
                }
            }

            let output = self.otpt;
            return output

        }
        pub fn n_o (&mut self, output: [i32;16]) -> [i32;16] {
            //  not the output according to the status
            // of the no control bit.
            let mut out: [i32;16] = [0;16];
            for i in 0..16 {
                out[i] = not(output[i]);
            }
            return out
        }

        pub  fn z_o (&mut self) {
            //  zero the output according to the status
            // of the zo control bit.
            let mut z_b: [i32;16] = [0;16];
            if self.otpt == z_b {
                self.zr = 1;
            } else {
                self.zr = 0;
            }
        }

        pub fn o_gt0 (&mut self) {
            // set multiplexor(?) bits according to output value.
            let expctd_otpt: [i32;16] = [0;16];
            if self.otpt != expctd_otpt {
                self.ng = 1;
            } else {
                self.ng = 0;
            }
        }


        // fn z_y(&self, zy: i32, y: [i32;16])
    }
    impl Default for ALU {
        fn default() -> ALU {
            let x: [i32;16] = [0;16];
            let y: [i32;16] = [0;16];
            let otpt: [i32;16] = [0;16];
            ALU {
                x: x,
                y: y,
                zx: 0,
                nx: 0,
                zy: 0,
                ny: 0,
                f:  0,
                no: 0,
                ng: 0,
                zr: 0,
                otpt: otpt
            }
        }
    }


}



