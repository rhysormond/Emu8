use sprites::SPRITE_SHEET;

/// # Chip-8
/// Chip-8 is a virtual machine and corresponding interpreted language.
///
/// ## CPU
///
/// Registers
/// - 16 primary 8-bit registers (V0..VF)
///     - the first 15 (V0..VE) are general purpose registers
///     - the 16th (VF) is the carry flag
/// - a 16-bit memory address register
///
/// Counter
/// - a 16-bit program counter
///
/// Pointer
/// - a 8-bit stack pointer
///
/// Timers
/// - 2 8-bit timers (delay & sound)
///     - they decrement sequentially once per tick
///     - when the sound timer is above 0 it plays a beep
///
/// ## Memory
/// - 64 byte stack
///     - stores return addresses when subroutines are called
/// - 32x64 byte frame buffer
///     - stores the contents of the next frame to be drawn
/// - 4096 bytes of addressable memory
pub struct Chip8 {
    v_registers: [u8; 16],
    address_register: u16,
    program_counter: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    stack: Stack,
    memory: Memory,
    pub frame_buffer: FrameBuffer,
}

pub type FrameBuffer = [[u8; 32]; 64];
pub type Memory = [u8; 4096];
pub type Stack = [u8; 16];

impl Chip8 {
    pub fn new() -> Self {
        // 0x000 - 0x080 is reserved for a sprite sheet
        let mut memory: Memory = [0; 4096];
        memory[0..80].copy_from_slice(&SPRITE_SHEET);

        // 0x200 is where ROMs are loaded into memory
        let program_counter: u16 = 0x200;

        Chip8 {
            v_registers: [0; 16],
            address_register: 0,
            program_counter,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            memory,
            frame_buffer: [[0; 32]; 64],
        }
    }

    /// Handles input from event loop.
    pub fn key_press(&mut self, key: u8) {
        // TODO Actually handle input
    }

    pub fn key_release(&mut self, key: u8) {
        // TODO Actually handle input
    }

    /// Executes a single CPU cycle
    pub fn cycle(&mut self) {
        // Get and execute the next opcode
        let op: u16 = self.get_op();
        self.execute_op(op);

        // The delay timer decrements every CPU cycle
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        // Each time the sound timer is decremented it triggers a beep
        if self.sound_timer > 0 {
            // TODO Make some sound
            self.sound_timer -= 1;
        }
    }

    /// Gets the opcode pointed to by the program_counter
    /// Interpreter memory is stored as bytes, but opcodes are 16 bits.
    /// Because of this we need to combine subsequent bytes.
    fn get_op(&self) -> u16 {
        let left = u16::from(self.memory[self.program_counter as usize]);
        let right = u16::from(self.memory[self.program_counter as usize + 1]);
        left << 8 | right
    }

    /// Execute a single opcode
    fn execute_op(&self, op: u16) {
        // TODO All the things
    }
}
