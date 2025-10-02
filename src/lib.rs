mod instructions;

use anyhow::anyhow;
use std::{collections::VecDeque, io::BufRead};

const PROGRAM_MEMORY_START: usize = 0x200;
const MAX_MEMORY: usize = 0x1000;
const AVAILABLE_PROGRAM_MEMORY: usize = MAX_MEMORY - PROGRAM_MEMORY_START;
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct Ahoy {
    memory: [u8; MAX_MEMORY],
    index: u16,
    counter: u16,
    stack: VecDeque<u16>,
    delay_timer: u8,
    sound_timer: u8,
}

impl Default for Ahoy {
    fn default() -> Self {
        let mut memory = [0; MAX_MEMORY];

        memory[0x050..=0x09F].copy_from_slice(&FONT);

        Ahoy {
            memory,
            index: 0,
            counter: 0,
            stack: VecDeque::with_capacity(256),
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

impl Ahoy {
    pub fn load<R: BufRead>(&mut self, program_reader: &mut R) -> anyhow::Result<()> {
        let mut total_bytes_read = 0_usize;

        while let Ok(curr_bytes_read) =
            program_reader.read(&mut self.memory[PROGRAM_MEMORY_START..])
        {
            if curr_bytes_read == 0 {
                break;
            }
            total_bytes_read += curr_bytes_read;
        }

        if total_bytes_read == 0 {
            return Err(anyhow!("Received empty program"));
        }

        if total_bytes_read > AVAILABLE_PROGRAM_MEMORY {
            return Err(anyhow!("Program exceeds memory limits"));
        }

        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        let usize_counter = self.counter as usize;

        let first_nibble: u16 = self.memory[usize_counter].into();
        let second_nibble: u16 = self.memory[usize_counter + 1].into();

        let instruction = (first_nibble << 8) | second_nibble;

        self.counter = (self.counter + 2) % (MAX_MEMORY as u16);
        instruction
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::Ahoy;

    #[test]
    fn load_normal_program() {
        let mut chip8 = Ahoy::default();
        let mut program_reader = BufReader::new(Cursor::new([
            0x01_u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
        ]));

        chip8.load(&mut program_reader).unwrap();

        assert_eq!(chip8.memory[0x200], 0x01);
        assert_eq!(chip8.memory[0x201], 0x02);
        assert_eq!(chip8.memory[0x202], 0x03);
        assert_eq!(chip8.memory[0x203], 0x04);
        assert_eq!(chip8.memory[0x204], 0x05);
        assert_eq!(chip8.memory[0x205], 0x06);
        assert_eq!(chip8.memory[0x206], 0x07);
        assert_eq!(chip8.memory[0x207], 0x08);
        assert_eq!(chip8.memory[0x208], 0x09);
        assert_eq!(chip8.memory[0x209], 0x0A);
    }

    #[test]
    fn load_returns_error_for_empty_file() {
        let mut chip8 = Ahoy::default();
        let mut program_reader = BufReader::new(Cursor::new([]));

        chip8
            .load(&mut program_reader)
            .expect_err("Expected empty program to raise error");
    }

    #[test]
    fn load_returns_error_for_larger_than_buffer_file() {
        let mut chip8 = Ahoy::default();
        let mut program_reader = BufReader::new(Cursor::new([1u8; 4096]));

        chip8
            .load(&mut program_reader)
            .expect_err("Expected large program to raise error");
    }

    #[test]
    fn fetch_increments_program_counter_by_two() {
        let mut chip8 = Ahoy::default();

        chip8.fetch();
        assert_eq!(chip8.counter, 2u16);

        chip8.fetch();
        assert_eq!(chip8.counter, 4u16);
    }

    #[test]
    fn fetch_loops_back_program_counter_to_zero_when_overflowing_12bits() {
        let mut chip8 = Ahoy {
            counter: 4094,
            ..Default::default()
        };

        chip8.fetch();
        assert_eq!(chip8.counter, 0);

        chip8.fetch();
        assert_eq!(chip8.counter, 2);
    }
    #[test]
    fn fetch_retrieves_expected_bytes_from_memory_beginning() {
        let mut chip8 = Ahoy::default();
        chip8.memory[0..2].copy_from_slice(&[0xF0, 0x0F]);

        let expected_instruction = 0xF00F;

        let actual_instruction = chip8.fetch();
        assert_eq!(expected_instruction, actual_instruction);
    }

    #[test]
    fn fetch_retrieves_expected_bytes_from_arbitrary_position() {
        let mut chip8 = Ahoy {
            counter: 0x6F,
            ..Default::default()
        };
        chip8.memory[0x6F..0x73].copy_from_slice(&[0xAB, 0xBC, 0xCD, 0xDE]);

        let expected_instruction = 0xABBC;
        let actual_instruction = chip8.fetch();

        assert_eq!(expected_instruction, actual_instruction);
    }
}
