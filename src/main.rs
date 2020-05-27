fn nand(x: i32, y: i32) -> i32 {
    if x == 1 && y == 1 {
        return 0
    }
    else if x == 1 && y == 0 {
        return 1
    } else { return 1}
}

fn not(x: i32) -> i32 {
    if x == 1 {
        return 0
    }
    else { return 1 }
}

fn not_16(array: &mut [i32; 16]) -> &mut [i32; 16] {
    if array.len() > 15 {
        println!("You may lose {} bits from this {} bit array!", (array.len() - 15), array.len());
    }
    for i in 1..15 {
        array[i] = not(array[i]) 
    }
    return array
}

fn and(x: i32, y: i32) -> i32 {
    if x == 1 && y == 1 {
        return 1
    }
    else { return 0 }
}

fn and_16(array_a: [i32; 16], array_b: [i32; 16], out: &mut [i32; 16]) -> [i32; 16] {
    for i in 0..15 {
        out[i] = and(array_a[i], array_b[i])
    }
    return *out
}

fn or(x: i32, y: i32) -> i32 {
    if x == 1 && y == 0 {
        return 1
    }
    else if x == 0 && y == 1 {
        return 1
    }
    else if x == 1 && y == 1 {
        return 1
    } 
    else if x == 0 && y == 0 { 
        return 0 
    }
    else {
        return 0
    }
}

fn or_16(array_a: [i32; 16], array_b: [i32; 16], out: &mut [i32; 16]) -> [i32; 16] {
    for i in 0..15 {
        out[i] = or(array_a[i], array_b[i]);
    }
    return *out
} 


fn xor(x: i32, y: i32) -> i32 {
    if x == 1 && y == 0 {
        return 1
    }
    else if x == 0 && y == 1 {
        return 1
    }
    else if x == 1 && y == 1 {
        return 0
    } else if x == 0 && y == 0 {
        return 0
    } else {return 0}
}
fn or_nand(x: i32, y: i32) -> i32 {
    if nand(x, y) == 1 {
        return 1 
    } 
    else if !(nand(x, y)) {
        return 0 
    } else { return 1 }
}

fn or_8w(array_a: [i32; 4], array_b: [i32;4]) -> [i32; 4] {
    // instead of one set of 8 bits, we could do two sets of...
    let mut out: [i32;4] = [1; 4];
    for i in 0..4 {
        out[i] = or(array_a[i], array_b[i])
    }
    // for i in 0..3 {
    //     let x = array[i];
    //     let y = array[i + 1];
    //     while array[i + 1] {
    //         let out_bit = or(x, y);
    //         out[i] = &out_bit
    //     }
    //     // if i + 1 > array.len() {
    //     //     return *out
    //     // } else {out[i] = or(x, y);}
    // }
    // // while j > array.len() {
    // //     out[i] = or(array[i], array[j]);
    // //     i = i + 1;
    // // }
    return out
}
fn mux(x: i32, y: i32, sel: i32) -> i32 {
    // Mux returns one of it's 
    if sel == 0 {
        return x
    } else { return y}
}

fn mux_16(array_a: [i32; 16], array_b: [i32; 16], sel: i32) -> [i32;16] {
    let mut out: [i32; 16] = [1; 16];
    for i in 0..15 {
        out[i] = mux(array_a[i], array_a[i], sel);
    }
    return out
}


fn mux_16_4w(array_a: [i32; 16], array_b: [i32; 16], array_c: [i32; 16], array_d: [i32; 16], sel: [i32; 2]) -> [i32; 16] {
    let mut out: [i32; 16] = [1; 16];
    
    let sel_1 = sel[0];
    let sel_2 = sel[1];
    if sel_1 == 0 && sel_2 == 0 {
        for i in 0..15 {
            out[i] = array_a[i]
        }
    }
    else if sel_1 == 1 && sel_2 == 1 {
        out[i] = array_d[i]
    }
    else if sel_1 == 1 && sel_2 == 0 {
        out[i] = array_c[i]
    }
    else {out[i] = array_b[i]}

    return out
}

fn mux_16_8w(
    array_a: [i32; 16], // 000 // 0
    array_b: [i32; 16], // 001 // 1
    array_c: [i32; 16], // 010 // 2
    array_d: [i32; 16], // 011 // 3
    array_e: [i32; 16], // 100 // 4
    array_f: [i32; 16], // 101 // 5
    array_g: [i32; 16], // 110 // 6
    array_h: [i32; 16], // 111 // 7
    sel: [i32; 3]) -> [i32; 16] {
    let mut out: [i32; 16] = [1; 16];
    
    let sel_1 = sel[0];
    let sel_2 = sel[1];
    let sel_3 = sel[2];
    if sel_1 == 0 && sel_2 == 0 && sel_3 == 0 { // 000
        for i in 0..15 {
            out[i] = array_a[i]
        }
    }
    else if sel_1 == 0 && sel_2 == 0 && sel_3 == 1 { // 001
        for i in 0..15 {
            out[i] = array_b[i]
        }
    }
    else if sel_1 == 0 && sel_2 == 1 && sel_3 == 0 { // 010
        for in 0..15 {
            out[i] = array_c[i]
        }
    }
    else if sel_1 == 0 && sel_2 == 1 && sel_3 == 1 { // 011
        for in 0..15 {
            out[i] = array_d[i]
        }
    }
    else if sel_1 == 1 && sel_2 == 0 && sel_3 == 0 { // 100
        for in 0..15 
            out[i] = array_e[i]
    }
    else if sel_1 == 1 && sel_2 == 0 && sel_3 == 1 { // 101
        out[i] = array_f[i]
    }
    else if sel_1 == 1 && sel_2 == 1 && sel_3 == 0 { // 110
        out[i] = array_g[i]
    }
    else if sel_1 == 1 && sel_2 == 1 && sel_3 == 1 { // 110
        out[i] = array_h[i]
    }
    else {return 0}

    return out
}



fn dmux(in_bit: i32, sel: i32) -> i32 {
    // How would we test this?
    // Well, if we send in a 1, as the in_bit, and the sel bit is 1,
    // we should be able to return a/1/1, so the output from our dmux
    // on those values should be usable in an and test with 1, which should
    // return 1.
    if sel == 0 {
        let a = in_bit;
        let b = 0;
        (a, b)
    } else { let b = in_bit; let a = 0; (a, b)}
}

fn dmux_4w(in_bit: i32, sel: [i32;2]) -> {
    let (sel_1, sel_2) = (sel[0], sel[1]);

    if sel_1 == 0 && sel_2 == 0 {
        let a = in_bit;
        let (b, c, d) = (0, 0, 0);
        return (a, b, c, d)
    }
    else if sel_1 == 0 && sel_2 == 1 {
        let b = in_bit;
        let (a, c, d) = (0, 0, 0);
        return (a, b, c, d)
    }
    else if sel_1 == 1 && sel_2 == 0 {
        let c = in_bit;
        let (a, b, d) = (0, 0, 0);
        return (a, b, c, d)
        
    }
    else {
        let d = in_bit;
        let (a, b, c) = (0, 0, 0);
        return (a, b, c, d)
    }
}
fn dmux_8w(in_bit: i32, sel: [i32;3]) -> {
    let out_bits: (i32, i32, i32, i32, i32, i32, i32, i32, ) = (0, 0, 0, 0, 0, 0, 0, 0, ) ;
    let (sel_1, sel_2, sel_3) = (sel[0], sel[1], sel[2]);
                    // 100 // 4
                    // 101 // 5
                    // 110 // 6
                    // 111 // 7
    if sel_1 == 0 && sel_2 == 0 && sel_3 == 0 { // 000 // 0
        let a = in_bit;
        return (a, b, c, d, e, f, g, h)

    }
    if sel_1 == 0 && sel_2 == 1 && sel_3 == 0 { // 001 // 1
        let b = in_bit;
        let (a, _, c, d, e, f, g, h) = out_bits;
        return (a, b, c, d, e, f, g, h)

    }
    else if sel_1 == 0 && sel_2 == 1 && sel_3 == 0 { // 010 // 2
        let c = in_bit;
        let (a, b, _, d, e, f, g, h) = out_bits;
        return (a, b, c, d, e, f, g, h)

    }
    else if sel_1 == 0 && sel_2 == 1 && sel_3 == 1 { // 011 // 3
        let d = in_bit;
        let (a, b, c, _, e, f, g, h) = out_bits;
        return (a, b, c, d, e, f, g, h)

    }
    else if sel_1 == 1 && sel_2 == 0 && sel_3 == 0 { // 100 // 4
        let e = in_bit;
        let (a, b, c, d, _, f, g, h) = out_bits;
        return (a, b, c, d, e, f, g, h)

    }
    else if sel_1 == 1 && sel_2 == 0 && sel_3 == 1 { // 101 // 5
        let f = in_bit;
        let (a, b, c, d, e, _, g, h) = out_bits;
        return (a, b, c, d, e, f, g, h)

    }
    else if sel_1 == 1 && sel_2 == 1 && sel_3 == 0 { // 110 // 6
        let g = in_bit;
        let (a, b, c, d, e, f, _, h) = out_bits;
        return (a, b, c, d, e, f, g, h)

    }
    else if sel_1 == 1 && sel_2 == 1 && sel_3 == 1 { // 111 // 67
        let h = in_bit;
        let (a, b, c, d, e, f, g, _) = out_bits;
        return (a, b, c, d, e, f, g, h)

    } else { return false}
}

fn add_gate(x: i32, y: i32, cin: i32) -> i32 {

    // We understand this somewhat in terms of the i32ean,
    // conditions, but thinking about how these bit operations
    // correspond to addition is a bit out of reach at the moment.
    // The carry in bit signifies a place holder for the 10's or 100's
    // place if the previous operands overflow from the previous...register?
    if x == 0 && y == 0 && cin == 0 { // 000
        return 0
    }
    else if x == 0 && y == 0 && cin == 1 { // 001
        return 0
    }
    else if x == 0 && y == 1 && cin == 1 { // 011
        return 1
    }
    else if x == 0 && y == 1 && cin == 0 {// 010
        return 0
    }
    else if x == 1 && y == 1 && cin == 0 {// 110
        return 1
    }
    else if x == 1 && y == 1 && cin == 1 {// 110
        return 1
    }
    else if x == 1 && y == 0 && cin == 1 {// 110
        return 1
    }
    else { return 0 }
}
fn main() {
    let or_nand_gate_1 = or_nand(1, 0);
    let or_nand_gate_2 = or_nand(0, 0);
    let or_nand_gate_3 = or_nand(1, 1);
    let out = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    let and_16_arr_1 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    let and_16_arr_2 = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    let or_8w_arr_1 = [0, 0, 0, 0];
    let or_8w_arr_2 = [ 1, 1, 0, 0];
    let mut or_out_1 = [1, 1, 1, 1];
   #[cfg(test)]
   mod tests {
       fn test_or_nand() {
        let or_nand_gate_1 = or_nand(1, 0);
        let or_nand_gate_2 = or_nand(0, 0);

           assert_eq!(or_nand_gate_1, 1);
           assert_eq!(or_nand_gate_2, 0);
           assert_eq!(or_nand_gate_3, 0);
       }

       fn test_and() {
           let and_gate_1 = and(1, 1);
           let and_gate_2 = and(1, 0);
           let and_gate_3 = and(0, 0);

           assert_eq!(and_gate_1, 1);
           assert_eq!(and_gate_2, 0);
           assert_eq!(and_gate_3, 0);
       }
   }
    // println!("{}", or_nand(1, 0));
    // println!("{:?}", and_16(and_16_arr_1, and_16_arr_2, &mut out));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_2, and_16_arr_1, 0));
    println!("or_8w is: {:?}", or_8w(or_8w_arr_1, or_8w_arr_2));
    // println!("{}", or(0, 0));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_1, out, 1));
}