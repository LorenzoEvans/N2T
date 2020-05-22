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

fn or_nand(x: bool, y: bool) -> bool {
    if nand(x, y) == true {
        return true 
    } else {return false}
    if !(nand(x, y)) {
        return false 
    } else { return true }
}


fn main() {
    or_nand_gate_1 = or_nand(true, false);
    or_nand_gate_2 = or_nand(false, false);

   #[cfg(test)]
   mod tests {
       fn test_or_nand() {
           assert!()
       }
   }
    println!("{}", or_nand(true, false));
}
