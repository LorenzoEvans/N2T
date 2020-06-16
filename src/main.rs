use nand2_tetris::ALU::ALU::{ALU};
#[allow(unused_variables)]
fn main() {
  let mux_16_4w_arr_1 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  let mux_16_4w_arr_3 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  
  
  
  let add_16_arr_1 = [0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
  let add_16_arr_2 = [0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
  let out = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  let mut alu = ALU::default();
  alu.zx = 3;
  println!("alu: {:?}", alu);
  // println!("add_16_arr_1: {:?}", add_16_arr_1);
  // println!("add_16_arr_2: {:?}", add_16_arr_2);
  // println!("add_16      :{:?}", add_16(add_16_arr_1, add_16_arr_2));
}
