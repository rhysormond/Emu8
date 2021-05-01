use crate::opcode::Opcode;
use crate::operations::*;
use crate::state::State;

/// Selects the correct Instruction for a given Opcode
pub fn instruction(op: &dyn Opcode) -> fn(&dyn Opcode, &State, [u8; 16]) -> State {
    match op.nibbles() {
        (0x0, 0x0, 0xE, 0x0) => clr,
        (0x0, 0x0, 0xE, 0xE) => rts,
        (0x1, ..) => jump,
        (0x2, ..) => call,
        (0x3, ..) => ske,
        (0x4, ..) => skne,
        (0x5, .., 0x0) => skre,
        (0x6, ..) => load,
        (0x7, ..) => add,
        (0x8, .., 0x0) => mv,
        (0x8, .., 0x1) => or,
        (0x8, .., 0x2) => and,
        (0x8, .., 0x3) => xor,
        (0x8, .., 0x4) => addr,
        (0x8, .., 0x5) => sub,
        (0x8, .., 0x6) => shr,
        (0x8, .., 0x7) => subn,
        (0x8, .., 0xE) => shl,
        (0x9, .., 0x0) => skrne,
        (0xA, ..) => loadi,
        (0xB, ..) => jumpi,
        (0xC, ..) => rand,
        (0xD, ..) => draw,
        (0xE, .., 0x9, 0xE) => skpr,
        (0xE, .., 0xA, 0x1) => skup,
        (0xF, .., 0x0, 0x7) => moved,
        (0xF, .., 0x0, 0xA) => keyd,
        (0xF, .., 0x1, 0x5) => loads,
        (0xF, .., 0x1, 0x8) => ld,
        (0xF, .., 0x1, 0xE) => addi,
        (0xF, .., 0x2, 0x9) => ldspr,
        (0xF, .., 0x3, 0x3) => bcd,
        (0xF, .., 0x5, 0x5) => stor,
        (0xF, .., 0x6, 0x5) => read,
        other => panic!("Opcode {:?} is not implemented", other),
    }
}

#[cfg(test)]
mod test_instruction {
    use super::instruction;
    use crate::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
    use crate::state::State;

    #[test]
    fn test_00e0_cls() {
        let mut state = State::new();
        state.frame_buffer[0][0] = 1;
        let op = 0x00E0;
        let state = instruction(&0x00E0)(&op, &state, [0; 16]);
        assert_eq!(state.frame_buffer[0][0], 0);
    }

    #[test]
    fn test_00ee_ret() {
        let mut state = State::new();
        state.sp = 0x1;
        state.stack[state.sp as usize] = 0xABCD;
        let op = 0x00EE;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.sp, 0x0);
        // Add 2 to the program as it's bumped after opcode execution
        assert_eq!(state.pc, 0xABCD + 0x2);
    }

    #[test]
    fn test_1nnn_jp() {
        let state = State::new();
        let op = 0x1ABC;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0ABC);
    }

    #[test]
    fn test_2nnn_call() {
        let mut state = State::new();
        state.pc = 0xABCD;
        let op = 0x2123;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.sp, 0x1);
        assert_eq!(state.stack[state.sp as usize], 0xABCD);
        assert_eq!(state.pc, 0x0123);
    }

    #[test]
    fn test_3xkk_se_skips() {
        let mut state = State::new();
        state.v[0x1] = 0x11;
        let op = 0x3111;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0204);
    }

    #[test]
    fn test_3xkk_se_doesntskip() {
        let state = State::new();
        let op = 0x3111;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0202);
    }

    #[test]
    fn test_4xkk_sne_skips() {
        let state = State::new();
        let op = 0x4111;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0204);
    }

    #[test]
    fn test_3xkk_sne_doesntskip() {
        let mut state = State::new();
        state.v[0x1] = 0x11;
        let op = 0x4111;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0202);
    }

    #[test]
    fn test_5xy0_se_skips() {
        let mut state = State::new();
        state.v[0x1] = 0x11;
        state.v[0x2] = 0x11;
        let op = 0x5120;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0204);
    }

    #[test]
    fn test_5xy0_se_doesntskip() {
        let mut state = State::new();
        state.v[0x1] = 0x11;
        let op = 0x5120;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0202);
    }

    #[test]
    fn test_6xkk_ld() {
        let state = State::new();
        let op = 0x6122;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x22);
    }

    #[test]
    fn test_7xkk_add() {
        let mut state = State::new();
        state.v[0x1] = 0x1;
        let op = 0x7122;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x23);
    }

    #[test]
    fn test_8xy0_ld() {
        let mut state = State::new();
        state.v[0x2] = 0x1;
        let op = 0x8120;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x1);
    }

    #[test]
    fn test_8xy1_or() {
        let mut state = State::new();
        state.v[0x1] = 0x6;
        state.v[0x2] = 0x3;
        let op = 0x8121;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x7);
    }

    #[test]
    fn test_8xy2_and() {
        let mut state = State::new();
        state.v[0x1] = 0x6;
        state.v[0x2] = 0x3;
        let op = 0x8122;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x2);
    }

    #[test]
    fn test_8xy3_xor() {
        let mut state = State::new();
        state.v[0x1] = 0x6;
        state.v[0x2] = 0x3;
        let op = 0x8123;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x5);
    }

    #[test]
    fn test_8xy4_add_nocarry() {
        let mut state = State::new();
        state.v[0x1] = 0xEE;
        state.v[0x2] = 0x11;
        let op = 0x8124;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0xFF);
        assert_eq!(state.v[0xF], 0x0);
    }

    #[test]
    fn test_8xy4_add_carry() {
        let mut state = State::new();
        state.v[0x1] = 0xFF;
        state.v[0x2] = 0x11;
        let op = 0x8124;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x10);
        assert_eq!(state.v[0xF], 0x1);
    }

    #[test]
    fn test_8xy5_sub_nocarry() {
        let mut state = State::new();
        state.v[0x1] = 0x33;
        state.v[0x2] = 0x11;
        let op = 0x8125;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x22);
        assert_eq!(state.v[0xF], 0x1);
    }

    #[test]
    fn test_8xy5_sub_carry() {
        let mut state = State::new();
        state.v[0x1] = 0x11;
        state.v[0x2] = 0x12;
        let op = 0x8125;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0xFF);
        assert_eq!(state.v[0xF], 0x0);
    }

    #[test]
    fn test_8xy6_shr_lsb() {
        let mut state = State::new();
        state.v[0x1] = 0x5;
        let op = 0x8106;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x2);
        assert_eq!(state.v[0xF], 0x1);
    }

    #[test]
    fn test_8xy6_shr_nolsb() {
        let mut state = State::new();
        state.v[0x1] = 0x4;
        let op = 0x8106;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x2);
        assert_eq!(state.v[0xF], 0x0);
    }

    #[test]
    fn test_8xy7_subn_nocarry() {
        let mut state = State::new();
        state.v[0x1] = 0x11;
        state.v[0x2] = 0x33;
        let op = 0x8127;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x22);
        assert_eq!(state.v[0xF], 0x1);
    }

    #[test]
    fn test_8xy7_subn_carry() {
        let mut state = State::new();
        state.v[0x1] = 0x12;
        state.v[0x2] = 0x11;
        let op = 0x8127;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0xFF);
        assert_eq!(state.v[0xF], 0x0);
    }

    #[test]
    fn test_8xye_shl_msb() {
        let mut state = State::new();
        state.v[0x1] = 0xFF;
        let op = 0x810E;
        let state = instruction(&op)(&op, &state, [0; 16]);
        // 0xFF * 2 = 0x01FE
        assert_eq!(state.v[0x1], 0xFE);
        assert_eq!(state.v[0xF], 0x1);
    }

    #[test]
    fn test_8xye_shl_nomsb() {
        let mut state = State::new();
        state.v[0x1] = 0x4;
        let op = 0x810E;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0x8);
        assert_eq!(state.v[0xF], 0x0);
    }

    #[test]
    fn test_9xy0_sne_skips() {
        let mut state = State::new();
        state.v[0x1] = 0x11;
        let op = 0x9120;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0204);
    }

    #[test]
    fn test_9xy0_sne_doesntskip() {
        let mut state = State::new();
        state.v[0x1] = 0x11;
        state.v[0x2] = 0x11;
        let op = 0x9120;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0202);
    }

    #[test]
    fn test_annn_ld() {
        let state = State::new();
        let op = 0xAABC;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.i, 0xABC);
    }

    #[test]
    fn test_bnnn_jp() {
        let mut state = State::new();
        state.v[0x0] = 0x2;
        let op = 0xBABC;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0xABE);
    }

    // Not testing cxkk as it generates a random number

    #[test]
    fn test_dxyn_drw_draws() {
        let mut state = State::new();
        state.v[0x0] = 0x1;
        // Draw the 0x0 sprite with a 1x 1y offset
        let op = 0xD005;
        let state = instruction(&op)(&op, &state, [0; 16]);
        let mut expected = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        expected[1][1..5].copy_from_slice(&[1, 1, 1, 1]);
        expected[2][1..5].copy_from_slice(&[1, 0, 0, 1]);
        expected[3][1..5].copy_from_slice(&[1, 0, 0, 1]);
        expected[4][1..5].copy_from_slice(&[1, 0, 0, 1]);
        expected[5][1..5].copy_from_slice(&[1, 1, 1, 1]);
        assert!(state
            .frame_buffer
            .iter()
            .zip(expected.iter())
            .all(|(a, b)| a[..] == b[..]));
    }

    #[test]
    fn test_dxyn_drw_collides() {
        let mut state = State::new();
        state.frame_buffer[0][0] = 1;
        let op = 0xD001;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0xF], 0x1)
    }

    #[test]
    fn test_dxyn_drw_xors() {
        let mut state = State::new();
        // 0 1 0 1 -> Set
        state.frame_buffer[0][2..6].copy_from_slice(&[0, 1, 0, 1]);
        // 1 1 0 0 -> Draw xor
        let op = 0xD005;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.frame_buffer[0][2..6], [1, 0, 0, 1])
    }

    #[test]
    fn test_ex9e_skp_skips() {
        let mut state = State::new();
        let mut pressed_keys = [0; 16];
        pressed_keys[0xE] = 0x1;
        state.v[0x1] = 0xE;
        let op = 0xE19E;
        let state = instruction(&op)(&op, &state, pressed_keys);
        assert_eq!(state.pc, 0x0204);
    }

    #[test]
    fn test_ex9e_skp_doesntskip() {
        let state = State::new();
        let op = 0xE19E;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0202);
    }

    #[test]
    fn test_exa1_sknp_skips() {
        let state = State::new();
        let op = 0xE1A1;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.pc, 0x0204);
    }

    #[test]
    fn test_exa1_sknp_doesntskip() {
        let mut state = State::new();
        let mut pressed_keys = [0; 16];
        pressed_keys[0xE] = 0x1;
        state.v[0x1] = 0xE;
        let op = 0xE1A1;
        let state = instruction(&op)(&op, &state, pressed_keys);
        assert_eq!(state.pc, 0x0202);
    }

    #[test]
    fn test_fx07_ld() {
        let mut state = State::new();
        state.delay_timer = 0xF;
        let op = 0xF107;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x1], 0xF);
    }

    #[test]
    fn test_fx0a_ld_setsregisterneedingkey() {
        let state = State::new();
        let op = 0xF10A;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.register_needing_key, Some(0x1));
    }

    #[test]
    fn test_fx15_ld() {
        let mut state = State::new();
        state.v[0x1] = 0xF;
        let op = 0xf115;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.delay_timer, 0xF);
    }

    #[test]
    fn test_fx18_ld() {
        let mut state = State::new();
        state.v[0x1] = 0xF;
        let op = 0xf118;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.sound_timer, 0xF);
    }

    #[test]
    fn test_fx1e_add() {
        let mut state = State::new();
        state.i = 0x1;
        state.v[0x1] = 0x1;
        let op = 0xF11E;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.i, 0x2);
    }

    #[test]
    fn test_fx29_ld() {
        let mut state = State::new();
        state.v[0x1] = 0x2;
        let op = 0xF129;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.i, 0xA);
    }

    #[test]
    fn test_fx33_ld() {
        let mut state = State::new();
        // 0x7B -> 123
        state.v[0x1] = 0x7B;
        state.i = 0x200;
        let op = 0xF133;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.memory[0x200..0x203], [0x1, 0x2, 0x3]);
    }

    #[test]
    fn test_fx_55_ld() {
        let mut state = State::new();
        state.i = 0x200;
        state.v[0x0..0x5].copy_from_slice(&[0x1, 0x2, 0x3, 0x4, 0x5]);
        let op = 0xF455;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.memory[0x200..0x205], [0x1, 0x2, 0x3, 0x4, 0x5]);
    }

    #[test]
    fn test_fx_65_ld() {
        let mut state = State::new();
        state.i = 0x200;
        state.memory[0x200..0x205].copy_from_slice(&[0x1, 0x2, 0x3, 0x4, 0x5]);
        let op = 0xF465;
        let state = instruction(&op)(&op, &state, [0; 16]);
        assert_eq!(state.v[0x0..0x5], [0x1, 0x2, 0x3, 0x4, 0x5]);
    }
}
