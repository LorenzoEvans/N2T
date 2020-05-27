fn nand(x: bool, y: bool) -> bool {
    if x == true && y == true {
        return false
    }
    else if x == true && y == false {
        return true
    } else { return true}
}

fn not(x: bool) -> bool {
    if x == true {
        return false
    }
    else { return true }
}

fn not_16(array: &mut [bool; 16]) -> &mut [bool; 16] {
    if array.len() > 15 {
        println!("You may lose {} bits from this {} bit array!", (array.len() - 15), array.len());
    }
    for i in 1..15 {
        array[i] = not(array[i]) 
    }
    return array
}

fn and(x: bool, y: bool) -> bool {
    if x == true && y == true {
        return true
    }
    else { return false }
}

fn and_16(array_a: [bool; 16], array_b: [bool; 16], out: &mut [bool; 16]) -> [bool; 16] {
    for i in 0..15 {
        out[i] = and(array_a[i], array_b[i])
    }
    return *out
}

fn or(x: bool, y: bool) -> bool {
    if x == true && y == false {
        return true
    }
    else if x == false && y == true {
        return true
    }
    else if x == true && y == true {
        return true
    } 
    else if x == false && y == false { 
        return false 
    }
    else {
        return false
    }
}

fn or_16(array_a: [bool; 16], array_b: [bool; 16], out: &mut [bool; 16]) -> [bool; 16] {
    for i in 0..15 {
        out[i] = or(array_a[i], array_b[i]);
    }
    return *out
} 


fn xor(x: bool, y: bool) -> bool {
    if x == true && y == false {
        return true
    }
    else if x == false && y == true {
        return true
    }
    else if x == true && y == true {
        return false
    } else if x == false && y == false {
        return false
    } else {return false}
}
fn or_nand(x: bool, y: bool) -> bool {
    if nand(x, y) == true {
        return true 
    } 
    else if !(nand(x, y)) {
        return false 
    } else { return true }
}

fn or_8w(array_a: [bool; 4], array_b: [bool;4]) -> [bool; 4] {
    // instead of one set of 8 bits, we could do two sets of...
    let mut out: [bool;4] = [true; 4];
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
fn mux(x: bool, y: bool, sel: bool) -> bool {
    // Mux returns one of it's 
    if sel == false {
        return x
    } else { return y}
}

fn mux_16(array_a: [bool; 16], array_b: [bool; 16], sel: bool) -> [bool;16] {
    let mut out: [bool; 16] = [true; 16];
    for i in 0..15 {
        out[i] = mux(array_a[i], array_a[i], sel);
    }
    return out
}

fn mux_16_4w(array_a: [bool; 16], array_b: [bool; 16], array_c: [bool; 16], array_d: [bool; 16], sel: [bool; 2]) -> [bool; 16] {
    let mut out: [bool; 16] = [true; 16];
    let sel_chk = |a, b| -> {
        if a == true && b == true {

        }
    }
    let sel_1 = sel[0];
    let sel_2 = sel[1];
    if sel_1 ==
}


fn dmux(in_bit: bool, sel: bool) -> bool {
    // How would we test this?
    // Well, if we send in a 1, as the in_bit, and the sel bit is 1,
    // we should be able to return a/1/true, so the output from our dmux
    // on those values should be usable in an and test with true, which should
    // return true.
    if sel == false {
        let a = in_bit;
        return a
    } else { let b = in_bit; return b }
}

fn add_gate(x: bool, y: bool, cin: bool) -> bool {

    // We understand this somewhat in terms of the boolean,
    // conditions, but thinking about how these bit operations
    // correspond to addition is a bit out of reach at the moment.
    // The carry in bit signifies a place holder for the 10's or 100's
    // place if the previous operands overflow from the previous...register?
    if x == false && y == false && cin == false { // 000
        return false
    }
    else if x == false && y == false && cin == true { // 001
        return false
    }
    else if x == false && y == true && cin == true { // 011
        return true
    }
    else if x == false && y == true && cin == false {// 010
        return false
    }
    else if x == true && y == true && cin == false {// 110
        return true
    }
    else if x == true && y == true && cin == true {// 110
        return true
    }
    else if x == true && y == false && cin == true {// 110
        return true
    }
    else { return false }
}
fn main() {
    let or_nand_gate_1 = or_nand(true, false);
    let or_nand_gate_2 = or_nand(false, false);
    let or_nand_gate_3 = or_nand(true, true);
    let out = [true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false];
    let and_16_arr_1 = [false, false, false, false, true, false, true, false, true, false, true, false, true, false, true, false];
    let and_16_arr_2 = [false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true];
    let or_8w_arr_1 = [false, false, false, false];
    let or_8w_arr_2 = [ true, true, false, false];
    let mut or_out_1 = [true, true, true, true];
   #[cfg(test)]
   mod tests {
       fn test_or_nand() {
        let or_nand_gate_1 = or_nand(true, false);
        let or_nand_gate_2 = or_nand(false, false);

           assert_eq!(or_nand_gate_1, true);
           assert_eq!(or_nand_gate_2, false);
           assert_eq!(or_nand_gate_3, false);
       }

       fn test_and() {
           let and_gate_1 = and(true, true);
           let and_gate_2 = and(true, false);
           let and_gate_3 = and(false, false);

           assert_eq!(and_gate_1, true);
           assert_eq!(and_gate_2, false);
           assert_eq!(and_gate_3, false);
       }
   }
    // println!("{}", or_nand(true, false));
    // println!("{:?}", and_16(and_16_arr_1, and_16_arr_2, &mut out));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_2, and_16_arr_1, false));
    println!("or_8w is: {:?}", or_8w(or_8w_arr_1, or_8w_arr_2));
    // println!("{}", or(false, false));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_1, out, true));
}