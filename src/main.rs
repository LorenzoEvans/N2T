use nand2_tetris::adders::adders::{half_adder, add_16};
#[allow(unused_variables)]
fn main() {
  let mux_16_4w_arr_1 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  let mux_16_4w_arr_3 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  
  
  
  let mux_16_4w_arr_2 = [0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
  let mux_16_4w_arr_4 = [0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
  let out = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  println!("Add 16 : {:?}", add_16(mux_16_4w_arr_4, mux_16_4w_arr_2));
}
