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

fn and(x: bool, y: bool) -> bool {
    if x == true && y == true {
        return true
    }
    else { return false }
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
    } else { return false }
}

fn xor(x: bool, y: bool) -> {
    if x == true && y == false {
        return true
    }
    else if x == false && y == true {
        return true
    }
    else if x == true && y == true {
        return false
    } else { return false }
}
fn or_nand(x: bool, y: bool) -> bool {
    if nand(x, y) == true {
        return true 
    } else {return false}
    if !(nand(x, y)) {
        return false 
    } else { return true }
}

fn mux(x: bool, y: bool, sel: bool) -> {
    if sel == false {
        return x
    } else { return y}
}

fn dmux(in_bit: bool, sel: bool) {
    // How would we test this?
    // Well, if we send in a 1, as the in_bit, and the sel bit is 1,
    // we should be able to return a/1/true, so the output from our dmux
    // on those values should be usable in an and test with true, which should
    // return true.
    if sel == false {
        let a = in_bit;
        return a
    }
    else {
        let b = in_bit;
        return b
    }
}
fn main() {
    let or_nand_gate_1 = or_nand(true, false);
    let or_nand_gate_2 = or_nand(false, false);
    let or_nand_gate_3 = or_nand(true, true)

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
    println!("{}", or_nand(true, false));
}
