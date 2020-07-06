extern crate nand2_tetris;
#[cfg(test)]
mod alu_test {
    use nand2_tetris::ALU::ALU::{ALU};
    #[test]
    fn test_zero_y_input() {
        let mut alu = ALU::default();
        let expctd_otpt: [i32;16] = [0;16];
        let y_input: [i32;16] = [1;16];

        alu.y = y_input;
        let y_otpt = alu.z_y(alu.y);
        assert_eq!(expctd_otpt, y_otpt)
        
    }

}