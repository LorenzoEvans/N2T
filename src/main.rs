// use crate::l_g::{and, or_nand, or_8w};
use nand2_tetris::l_g::l_g::{mux_16_4w};
fn main() {
          let mux_16_4w_arr_1 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let mux_16_4w_arr_2 = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let mux_16_4w_arr_3 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let mux_16_4w_arr_4 = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let out = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        println!("Mux164w: {:?}", nand2_tetris::l_g::l_g::mux_16_4w(mux_16_4w_arr_1, 
                                            mux_16_4w_arr_2, 
                                            mux_16_4w_arr_3, 
                                            mux_16_4w_arr_4, 
                                            [0, 0]));
    }
    // println!("{}", or_nand(1, 0));
    // println!("{:?}", and_16(and_16_arr_1, and_16_arr_2, &mut out));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_2, and_16_arr_1, 0));
    // println!("or_8w is: {:?}", l_g::or_8w(or_8w_arr_1, or_8w_arr_2));
    // println!("{}", or(0, 0));
    // println!("mux_16 is: {:?}", mux_16(and_16_arr_1, out, 1));