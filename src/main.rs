// use crate::l_g::{and, or_nand, or_8w};
fn main() {
    // println!("{}", or_nand(1, 0));
    // println!("{:?}", and_16(and_16_arr_1, and_16_arr_2, &mut out));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_2, and_16_arr_1, 0));
    // println!("or_8w is: {:?}", l_g::or_8w(or_8w_arr_1, or_8w_arr_2));
    // println!("{}", or(0, 0));
println!("Test mux: {:?}", nand2_tetris::mux_16([0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1], [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 0));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_1, out, 1));
}