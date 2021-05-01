use crate::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::opcode::Opcode;
use crate::state::State;

/// Selects the correct Instruction for a given Opcode
pub fn execute(op: &dyn Opcode, state: &State, pressed_keys: [u8; 16]) -> State {
    let instruction = match op.nibbles() {
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
    };
    instruction(op, state, pressed_keys)
}

/// clear
fn clr(_op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: state.pc + 0x2,
        frame_buffer: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        draw_flag: true,
        ..*state
    }
}

/// PC = STACK.pop()
fn rts(_op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: state.stack[state.sp as usize] + 0x2,
        sp: state.sp - 0x1,
        ..*state
    }
}

/// PC = addr
fn jump(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: op.addr(),
        ..*state
    }
}

/// STACK.push(PC); PC = addr
fn call(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut sp = state.sp;
    sp += 0x1;
    let mut stack = state.stack;
    stack[sp as usize] = state.pc;
    State {
        pc: op.addr(),
        sp,
        stack,
        ..*state
    }
}

/// if Vx == kk then pc += 2
fn ske(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let pc = if state.v[op.x() as usize] == op.kk() {
        state.pc + 0x4
    } else {
        state.pc + 0x2
    };
    State { pc, ..*state }
}

/// if Vx != kk then pc += 2
fn skne(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let pc = if state.v[op.x() as usize] != op.kk() {
        state.pc + 0x4
    } else {
        state.pc + 0x2
    };
    State { pc, ..*state }
}

/// if Vx == Vy then pc += 2
fn skre(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let pc = if state.v[op.x() as usize] == state.v[op.y() as usize] {
        state.pc + 0x4
    } else {
        state.pc + 0x2
    };
    State { pc, ..*state }
}

/// Vx = kk
fn load(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    v[op.x() as usize] = op.kk();
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx += kk
/// Add kk to Vx; allow for overflow but implicitly drop it
fn add(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let (res, _) = state.v[op.x() as usize].overflowing_add(op.kk());
    let mut v = state.v;
    v[op.x() as usize] = res;
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx = Vy
fn mv(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    v[op.x() as usize] = v[op.y() as usize];
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx |= Vy
fn or(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    v[op.x() as usize] |= v[op.y() as usize];
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx &= Vy
fn and(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    v[op.x() as usize] &= v[op.y() as usize];
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx ^= Vy
fn xor(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    v[op.x() as usize] ^= v[op.y() as usize];
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx += Vy; VF = overflow
fn addr(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let (res, over) = state.v[op.x() as usize].overflowing_add(state.v[op.y() as usize]);
    let mut v = state.v;
    v[0xF] = if over { 0x1 } else { 0x0 };
    v[op.x() as usize] = res;
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx -= Vy; VF = !underflow
fn sub(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let (res, under) = state.v[op.x() as usize].overflowing_sub(state.v[op.y() as usize]);
    let mut v = state.v;
    v[0xF] = if under { 0x0 } else { 0x1 };
    v[op.x() as usize] = res;
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx /= 2; VF = underflow
fn shr(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    v[0xF] = v[op.x() as usize] & 0x1;
    v[op.x() as usize] /= 0x2;
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx -= Vy; VF = underflow
fn subn(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let (res, under) = state.v[op.y() as usize].overflowing_sub(state.v[op.x() as usize]);
    let mut v = state.v;
    v[0xF] = if under { 0x0 } else { 0x1 };
    v[op.x() as usize] = res;
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// Vx *= 2; VF = overflow
fn shl(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let (res, over) = state.v[op.x() as usize].overflowing_mul(2);
    let mut v = state.v;
    v[0xF] = if over { 0x1 } else { 0x0 };
    v[op.x() as usize] = res;
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// if Vx != Vy then pc +=2
fn skrne(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let pc = if state.v[op.x() as usize] != state.v[op.y() as usize] {
        state.pc + 0x4
    } else {
        state.pc + 0x2
    };
    State { pc, ..*state }
}

/// I = addr
fn loadi(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: state.pc + 0x2,
        i: op.addr(),
        ..*state
    }
}

/// PC = V0 + addr
fn jumpi(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: u16::from(state.v[0x0]) + op.addr(),
        ..*state
    }
}

/// Vx = rand_byte + kk
fn rand(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let rand_byte: u8 = rand::random();
    let mut v = state.v;
    v[op.x() as usize] = rand_byte & op.kk();
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// draw_sprite(x=Vx y=Vy size=n)
/// XORs a sprite from memory i..n at position x, y on the FrameBuffer with wrapping.
/// Sets VF if any pixels would be erased
fn draw(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    let mut frame_buffer = state.frame_buffer;

    // Reset the carry flag (used for collision detection)
    v[0xF] = 0x0;

    for byte in 0..op.n() as usize {
        let y = (state.v[op.y() as usize] as usize + byte) % DISPLAY_HEIGHT;
        for bit in 0..8 {
            let x = (state.v[op.x() as usize] as usize + bit) % DISPLAY_WIDTH;
            let pixel_value = (state.memory[state.i as usize + byte] >> (7 - bit) as u8) & 1;
            v[0xF] |= pixel_value & state.frame_buffer[y as usize][x as usize];
            frame_buffer[y as usize][x as usize] ^= pixel_value;
        }
    }

    State {
        pc: state.pc + 0x2,
        draw_flag: true,
        v,
        frame_buffer,
        ..*state
    }
}

/// if Vx.pressed then pc += 2
fn skpr(op: &dyn Opcode, state: &State, pressed_keys: [u8; 16]) -> State {
    let pc = if pressed_keys[state.v[op.x() as usize] as usize] == 0x1 {
        state.pc + 0x4
    } else {
        state.pc + 0x2
    };
    State { pc, ..*state }
}

/// if !Vx.pressed then pc += 2
fn skup(op: &dyn Opcode, state: &State, pressed_keys: [u8; 16]) -> State {
    let pc = if pressed_keys[state.v[op.x() as usize] as usize] == 0x0 {
        state.pc + 0x4
    } else {
        state.pc + 0x2
    };
    State { pc, ..*state }
}

/// Vx = DT
fn moved(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    v[op.x() as usize] = state.delay_timer;
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}

/// await keypress for Vx
fn keyd(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: state.pc + 0x2,
        register_needing_key: Some(op.x()),
        ..*state
    }
}

/// DT = Vx
fn loads(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: state.pc + 0x2,
        delay_timer: state.v[op.x() as usize],
        ..*state
    }
}

/// ST = Vx
fn ld(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: state.pc + 0x2,
        sound_timer: state.v[op.x() as usize],
        ..*state
    }
}

/// I += Vx
fn addi(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: state.pc + 0x2,
        i: state.i + u16::from(state.v[op.x() as usize]),
        ..*state
    }
}

/// I = Vx * 5
/// Set I to the memory address of the sprite for Vx
/// See sprites::SPRITE_SHEET for more details
fn ldspr(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    State {
        pc: state.pc + 0x2,
        i: u16::from(state.v[op.x() as usize]) * 5,
        ..*state
    }
}

/// mem[I..I+3] = bcd(Vx)
/// Store BCD repr of Vx in memory starting at address i
fn bcd(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let bcd = [
        (state.v[op.x() as usize] / 100 % 10),
        (state.v[op.x() as usize] / 10 % 10),
        (state.v[op.x() as usize] % 10),
    ];
    let mut memory = state.memory;
    memory[state.i as usize..(state.i + 0x3) as usize].copy_from_slice(&bcd);
    State {
        pc: state.pc + 0x2,
        memory,
        ..*state
    }
}

/// mem[I..I+x] = V0..Vx
/// Fill memory starting at address i with V0..Vx+1
fn stor(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut memory = state.memory;
    memory[state.i as usize..=(state.i + u16::from(op.x())) as usize]
        .copy_from_slice(&state.v[0x0 as usize..=op.x() as usize]);
    State {
        pc: state.pc + 0x2,
        memory,
        ..*state
    }
}

/// V0..Vx = mem[I..I+x]
/// Fill V0..Vx+1 with memory starting at address i
fn read(op: &dyn Opcode, state: &State, _pressed_keys: [u8; 16]) -> State {
    let mut v = state.v;
    v[0x0 as usize..=op.x() as usize]
        .copy_from_slice(&state.memory[state.i as usize..=(state.i + u16::from(op.x())) as usize]);
    State {
        pc: state.pc + 0x2,
        v,
        ..*state
    }
}
