use nand2_tetris::adders::adders::{inc_16, ALU, add_16};
#[allow(unused_variables)]
fn main() {
  let mux_16_4w_arr_1 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  let mux_16_4w_arr_3 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  
  
  
  let add_16_arr_1 = [0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
  let add_16_arr_2 = [0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
  let out = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
  let mut ALU = ALU::default();
  ALU.zx = 3;
  println!("add->inc16 : {:?}", inc_16(add_16(add_16_arr_1, add_16_arr_2)));
}
