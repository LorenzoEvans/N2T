#[cfg(test)]
mod l_g_tests {
    use nand2_tetris::{nand, or_nand, and, or};
    // let out = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    // let and_16_arr_1 = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    // let and_16_arr_2 = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    // let or_8w_arr_1 = [0, 0, 0, 0];
    // let or_8w_arr_2 = [ 1, 1, 0, 0];
    // let mut or_out_1 = [1, 1, 1, 1];
    #[test]
    fn test_nand() {
        let nand_1 = (1, 1);
        let nand_2 = (1, 0);
        let nand_3 = (0, 0);
        let nand_4 = (0, 1);

        assert_eq!(nand(nand_1.1, nand_1.0), 0);
        assert_eq!(nand(nand_2.0, nand_2.1), 1);
        assert_eq!(nand(nand_3.0, nand_3.1), 1);
        assert_eq!(nand(nand_4.0, nand_4.1), 1);
    }
    #[test]
    fn test_or_nand() {
        let or_nand_gate_1 = or_nand(1, 0);
        let or_nand_gate_2 = or_nand(0, 0);
        let or_nand_gate_3 = or_nand(1, 1);

        assert_eq!(or_nand_gate_1, 1);
        assert_eq!(or_nand_gate_2, 1);
        assert_eq!(or_nand_gate_3, 0);
    }
    #[test]
    fn test_and() {
        let and_gate_1 = and(1, 1);
        let and_gate_2 = and(1, 0);
        let and_gate_3 = and(0, 0);
        let and_gate_4 = and(0, 1);

        assert_eq!(and_gate_1, 1);
        assert_eq!(and_gate_2, 0);
        assert_eq!(and_gate_3, 0);
        assert_eq!(and_gate_3, 0);
    }
    #[test]
    fn test_or() {
        let or_gate_1 = or(0, 0);
        let or_gate_2 = or(1, 0);
        let or_gate_3 = or(0, 1);
        let or_gate_4 = or(1, 1);

        assert_eq!(or_gate_1, 0);
        assert_eq!(or_gate_2, 1);
        assert_eq!(or_gate_3, 1);
        assert_eq!(or_gate_4, 1);
    }
}
