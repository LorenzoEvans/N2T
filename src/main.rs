// use crate::l_g::{and, or_nand, or_8w};
mod l_g;
fn main() {
    let or_nand_gate_1 = l_g::or_nand(1, 0);
    let or_nand_gate_2 = l_g::or_nand(0, 0);
    let or_nand_gate_3 = l_g::or_nand(1, 1);
    let out = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    let and_16_arr_1 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    let and_16_arr_2 = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    let or_8w_arr_1 = [0, 0, 0, 0];
    let or_8w_arr_2 = [ 1, 1, 0, 0];
    let mut or_out_1 = [1, 1, 1, 1];
   #[cfg(test)]
   mod tests {
       fn test_or_nand() {
        let or_nand_gate_1 = l_g::or_nand(1, 0);
        let or_nand_gate_2 = l_g::or_nand(0, 0);

           assert_eq!(or_nand_gate_1, 1);
           assert_eq!(or_nand_gate_2, 0);
           assert_eq!(or_nand_gate_3, 0);
       }

       fn test_and() {
           let and_gate_1 = l_g::and(1, 1);
           let and_gate_2 = l_g::and(1, 0);
           let and_gate_3 = l_g::and(0, 0);

           assert_eq!(and_gate_1, 1);
           assert_eq!(and_gate_2, 0);
           assert_eq!(and_gate_3, 0);
       }
   }
    // println!("{}", or_nand(1, 0));
    // println!("{:?}", and_16(and_16_arr_1, and_16_arr_2, &mut out));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_2, and_16_arr_1, 0));
    println!("or_8w is: {:?}", l_g::or_8w(or_8w_arr_1, or_8w_arr_2));
    // println!("{}", or(0, 0));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_1, out, 1));
}