use std::collections::VecDeque;
use std::io::Error;

use constants::{CPU_CYCLES_PER_TIMER_CYCLE, MAX_SAVED_STATES};
use opcode::Opcode;
use state::{FrameBuffer, State};

/// # Chip-8
/// Chip-8 is a virtual machine and corresponding interpreted language.
///
/// Tracks current (state) as well as (previous_states) for the purposes of rewinding.
/// Supplies interfaces for:
/// - loading roms
/// - pressing and releasing keys
/// - advancing and reversing the CPU
/// - advancing its timers
/// - inspecting its frame buffer for rendering by some display
pub struct Chip8 {
    state: State,
    previous_states: VecDeque<State>,
}

// TODO explore time/memory efficiency of more compact representations of past states (e.g. diffs)
impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            state: State::new(),
            previous_states: VecDeque::with_capacity(MAX_SAVED_STATES),
        }
    }

    /// Load a rom from a source file
    ///
    /// # Arguments
    /// * `file` a file reader that contains a ROM
    pub fn load_rom(&mut self, reader: &mut std::io::Read) -> Result<(), Error> {
        reader.read_exact(&mut self.state.memory[0x200..])
    }

    /// Returns the FrameBuffer if the display should be redrawn
    pub fn get_frame(&self) -> Option<FrameBuffer> {
        if self.state.draw_flag {
            Some(self.state.frame_buffer)
        } else {
            None
        }
    }

    /// Set the pressed status of key
    ///
    /// # Arguments
    /// * `key` the 8-bit representation of the key that was pressed
    pub fn key_press(&mut self, key: u8) {
        self.state.pressed_keys[key as usize] = 0x1;
        if let Some(register) = self.state.register_needing_key {
            self.state.v[register as usize] = key;
            self.state.register_needing_key = None;
        }
    }

    /// Unset the pressed status of key
    ///
    /// # Arguments
    /// * `key` the 8-bit representation of the key that was released
    pub fn key_release(&mut self, key: u8) {
        self.state.pressed_keys[key as usize] = 0x0;
    }

    /// Advances the CPU by a single cycle
    /// - breaks if awaiting a keypress
    /// - gets and executes the next opcode
    pub fn advance_cpu(&mut self) {
        if self.state.register_needing_key == None {
            let op: u16 = self.get_op();
            self.state = op.execute(&self.state);
        };
        self.save_state();
    }

    /// Reverses the CPU by a single cycle if possible
    /// - if there are previous_states, pops the last one and restores it
    pub fn reverse_cpu(&mut self) {
        self.load_state();
    }

    /// Puts the current state in previous_states
    /// - if there are already MAX_SAVED_STATES saved then the oldest is dropped
    fn save_state(&mut self) {
        if self.previous_states.len() == MAX_SAVED_STATES {
            self.previous_states.pop_back();
        }
        // TODO reevaluate whether we actually need to save pressed_keys in the state
        let mut state = self.state;
        // wipe pressed_keys before saving
        state.pressed_keys  = [0; 16];
        self.previous_states.push_front(state);
    }

    /// Puts the current state in previous_states
    /// - if there are already MAX_SAVED_STATES saved then the oldest is dropped
    fn load_state(&mut self) {
        let maybe_old_state: Option<State> = self.previous_states.pop_front();
        if let Some(state) = maybe_old_state {
            self.state = state
        }
    }

    /// Handles delay counter and timers
    /// - decrements the delay counter
    /// - decrements timers when the counter hits 0
    pub fn advance_timers(&mut self) {
        if self.state.delay_counter == 0 {
            self.state.delay_counter = CPU_CYCLES_PER_TIMER_CYCLE;

            if self.state.delay_timer > 0 {
                self.state.delay_timer -= 1;
            }

            if self.state.sound_timer > 0 {
                // TODO make some sound
                self.state.sound_timer -= 1;
            }
        } else {
            self.state.delay_counter -= 1;
        }
    }

    /// Gets the opcode currently pointed at by the pc.
    /// Memory is stored as bytes, but opcodes are 16 bits so we combine two subsequent bytes.
    fn get_op(&self) -> u16 {
        let left = u16::from(self.state.memory[self.state.pc as usize]);
        let right = u16::from(self.state.memory[self.state.pc as usize + 1]);
        left << 8 | right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8_gets_op() {
        let mut chip8 = Chip8::new();
        chip8.state.memory[0x200..0x202].copy_from_slice(&[0xAA, 0xBB]);
        assert_eq!(chip8.get_op(), 0xAABB);
    }

    #[test]
    fn test_cycles_while_no_register_needs_key() {
        let mut chip8 = Chip8::new();
        let starting_pc = chip8.state.pc;
        // insert a cls opcode so we don't panic at reading from empty memory
        chip8.state.memory[0x200..0x202].copy_from_slice(&[0x00, 0xE0]);
        chip8.advance_cpu();
        assert_eq!(chip8.state.pc, starting_pc + 0x2);
    }

    #[test]
    fn test_captures_key_presses() {
        let mut chip8 = Chip8::new();
        chip8.state.register_needing_key = Some(0x1);
        chip8.key_press(0xE);
        assert_eq!(chip8.state.register_needing_key, None);
        assert_eq!(chip8.state.v[0x1], 0xE);
    }

    #[test]
    fn test_doesnt_cycle_while_register_needs_key() {
        let mut chip8 = Chip8::new();
        let starting_pc = chip8.state.pc;
        chip8.state.register_needing_key = Some(0x1);
        chip8.advance_cpu();
        assert_eq!(chip8.state.pc, starting_pc);
    }

    #[test]
    fn test_chip8_saves_state() {
        let mut chip8 = Chip8::new();
        chip8.save_state();
        assert_eq!(chip8.previous_states.len(), 1);
    }

    #[test]
    fn test_chip8_clears_pressed_keys_when_saving_state() {
        let mut chip8 = Chip8::new();
        chip8.state.pressed_keys[0] = 0x1;
        chip8.save_state();
        assert_eq!(chip8.previous_states[0].pressed_keys[0], 0);
    }

    #[test]
    fn test_chip8_drops_old_saved_states() {
        let mut chip8 = Chip8::new();
        for _ in 0..MAX_SAVED_STATES {
            chip8.save_state();
        }
        assert_eq!(MAX_SAVED_STATES, chip8.previous_states.len());
        chip8.save_state();
        assert_eq!(MAX_SAVED_STATES, chip8.previous_states.len());
    }

    #[test]
    fn test_chip8_loads_states() {
        let mut chip8 = Chip8::new();
        let saved_state = chip8.state;
        chip8.previous_states.push_front(saved_state);
        chip8.state.delay_counter += 1;
        assert_eq!(chip8.state.delay_counter, 1);
        chip8.load_state();
        assert_eq!(chip8.state.delay_counter, 0);
    }
}
