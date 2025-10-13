mod constants;
mod display;
mod instructions;

use anyhow::anyhow;
use constants::FLAG_REGISTER;
use display::{AhoyFrame, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use instructions::AhoyInstruction;
use std::{collections::VecDeque, io::BufRead};

pub struct Ahoy {
    memory: [u8; constants::MAX_MEMORY],
    registers: [u8; 16],
    index: u16,
    counter: usize,
    stack: VecDeque<u16>,
    delay_timer: u8,
    sound_timer: u8,
    current_frame: AhoyFrame,
}

impl Default for Ahoy {
    fn default() -> Self {
        let mut memory = [0; constants::MAX_MEMORY];

        memory[0x050..=0x09F].copy_from_slice(&constants::FONT);

        Ahoy {
            memory,
            registers: [0; 16],
            index: 0,
            counter: 0,
            stack: VecDeque::with_capacity(256),
            delay_timer: 0,
            sound_timer: 0,
            current_frame: [0; 64],
        }
    }
}

impl Ahoy {
    pub fn load<R: BufRead>(&mut self, program_reader: &mut R) -> anyhow::Result<()> {
        let mut total_bytes_read = 0_usize;

        while let Ok(curr_bytes_read) =
            program_reader.read(&mut self.memory[constants::PROGRAM_MEMORY_START..])
        {
            if curr_bytes_read == 0 {
                break;
            }
            total_bytes_read += curr_bytes_read;
        }

        if total_bytes_read == 0 {
            return Err(anyhow!("Received empty program"));
        }

        if total_bytes_read > constants::AVAILABLE_PROGRAM_MEMORY {
            return Err(anyhow!("Program exceeds memory limits"));
        }

        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        let usize_counter = self.counter as usize;

        let first_nibble = self.memory[usize_counter] as u16;
        let second_nibble = self.memory[usize_counter + 1] as u16;

        let instruction = (first_nibble << 8) | second_nibble;

        self.counter = (self.counter + 2) % constants::MAX_MEMORY;
        instruction
    }

    fn execute(&mut self, instruction: AhoyInstruction) -> anyhow::Result<()> {
        match instruction {
            AhoyInstruction::ClearScreen => {
                self.current_frame = [0; DISPLAY_WIDTH];
            }
            AhoyInstruction::Jump(addr) => {
                self.counter = addr;
            }
            AhoyInstruction::SetRegister(register_addr, value) => {
                self.registers[register_addr] = value;
            }
            AhoyInstruction::AddToRegister(register_addr, value) => {
                let prev_value = self.registers[register_addr];
                self.registers[register_addr] = prev_value.wrapping_add(value);
            }
            AhoyInstruction::Display {
                x_register,
                y_register,
                sprite_height,
            } => {
                let x = self.registers[x_register] as usize % DISPLAY_WIDTH;
                let y = self.registers[y_register] as usize % DISPLAY_HEIGHT;

                let sprite_start = self.index as usize;
                let sprite_end = sprite_start + sprite_height as usize;
                let sprite = self.memory[sprite_start..sprite_end]
                    .iter()
                    .fold(0_u32, |sprite, b| (sprite << 1) | *b as u32);

                self.registers[FLAG_REGISTER] =
                    u8::from(((self.current_frame[x] >> y) & sprite) > 0);

                self.current_frame[x] ^= sprite << y;
            }
            _ => todo!(),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::{Ahoy, instructions::AhoyInstruction};

    #[test]
    fn load_normal_program() {
        let mut ahoy = Ahoy::default();
        let mut program_reader = BufReader::new(Cursor::new([
            0x01_u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
        ]));

        ahoy.load(&mut program_reader).unwrap();

        assert_eq!(ahoy.memory[0x200], 0x01);
        assert_eq!(ahoy.memory[0x201], 0x02);
        assert_eq!(ahoy.memory[0x202], 0x03);
        assert_eq!(ahoy.memory[0x203], 0x04);
        assert_eq!(ahoy.memory[0x204], 0x05);
        assert_eq!(ahoy.memory[0x205], 0x06);
        assert_eq!(ahoy.memory[0x206], 0x07);
        assert_eq!(ahoy.memory[0x207], 0x08);
        assert_eq!(ahoy.memory[0x208], 0x09);
        assert_eq!(ahoy.memory[0x209], 0x0A);
    }

    #[test]
    fn load_returns_error_for_empty_file() {
        let mut ahoy = Ahoy::default();
        let mut program_reader = BufReader::new(Cursor::new([]));

        ahoy.load(&mut program_reader)
            .expect_err("Expected empty program to raise error");
    }

    #[test]
    fn load_returns_error_for_larger_than_buffer_file() {
        let mut ahoy = Ahoy::default();
        let mut program_reader = BufReader::new(Cursor::new([1u8; 4096]));

        ahoy.load(&mut program_reader)
            .expect_err("Expected large program to raise error");
    }

    #[test]
    fn fetch_increments_program_counter_by_two() {
        let mut ahoy = Ahoy::default();

        ahoy.fetch();
        assert_eq!(ahoy.counter, 2_usize);

        ahoy.fetch();
        assert_eq!(ahoy.counter, 4_usize);
    }

    #[test]
    fn fetch_loops_back_program_counter_to_zero_when_overflowing_12bits() {
        let mut ahoy = Ahoy {
            counter: 4094,
            ..Default::default()
        };

        ahoy.fetch();
        assert_eq!(ahoy.counter, 0);

        ahoy.fetch();
        assert_eq!(ahoy.counter, 2);
    }
    #[test]
    fn fetch_retrieves_expected_bytes_from_memory_beginning() {
        let mut ahoy = Ahoy::default();
        ahoy.memory[0..2].copy_from_slice(&[0xF0, 0x0F]);

        let expected_instruction = 0xF00F;

        let actual_instruction = ahoy.fetch();
        assert_eq!(expected_instruction, actual_instruction);
    }

    #[test]
    fn fetch_retrieves_expected_bytes_from_arbitrary_position() {
        let mut ahoy = Ahoy {
            counter: 0x6F,
            ..Default::default()
        };
        ahoy.memory[0x6F..0x73].copy_from_slice(&[0xAB, 0xBC, 0xCD, 0xDE]);

        let expected_instruction = 0xABBC;
        let actual_instruction = ahoy.fetch();

        assert_eq!(expected_instruction, actual_instruction);
    }

    #[test]
    fn instruction_clear_screen_sets_frame_to_zeroes() {
        let mut ahoy = Ahoy {
            current_frame: [1; 64],
            ..Default::default()
        };
        ahoy.execute(AhoyInstruction::ClearScreen).unwrap();

        assert_eq!(ahoy.current_frame, [0_u32; 64]);
    }

    #[test]
    fn instruction_jump_updates_the_pc_value() {
        let mut ahoy = Ahoy::default();

        ahoy.execute(AhoyInstruction::Jump(0x0DAD)).unwrap();

        assert_eq!(ahoy.counter, 0x0DAD);
    }

    #[test]
    fn instruction_set_register_value_updates_value() {
        let mut ahoy = Ahoy::default();

        ahoy.execute(AhoyInstruction::SetRegister(0xA, 0xFE))
            .unwrap();
        ahoy.execute(AhoyInstruction::SetRegister(0xD, 0xE0))
            .unwrap();

        assert_eq!(ahoy.registers[0xA], 0xFE);

        assert_eq!(ahoy.registers[0xD], 0xE0);
    }

    #[test]
    fn instruction_add_value_to_register() {
        let mut ahoy = Ahoy::default();
        ahoy.registers[0xA] = 0x01;
        ahoy.registers[0xD] = 0x10;

        ahoy.execute(AhoyInstruction::AddToRegister(0xA, 0xFE))
            .unwrap();
        ahoy.execute(AhoyInstruction::AddToRegister(0xD, 0xE0))
            .unwrap();

        assert_eq!(ahoy.registers[0xA], 0xFF);
        assert_eq!(ahoy.registers[0xD], 0xF0);
    }

    #[test]
    fn instruction_register_wraps_around_when_adding_value() {
        let mut ahoy = Ahoy::default();
        ahoy.registers[0xA] = 0xFF;
        ahoy.registers[0xD] = 0xFF;

        ahoy.execute(AhoyInstruction::AddToRegister(0xA, 0xAB))
            .unwrap();
        ahoy.execute(AhoyInstruction::AddToRegister(0xD, 0xDE))
            .unwrap();

        assert_eq!(ahoy.registers[0xA], 0xAA);
        assert_eq!(ahoy.registers[0xD], 0xDD);
    }

    #[test]
    fn instruction_drawing_sets_value_on_empty_frame() {
        let mut ahoy = Ahoy::default();
        ahoy.memory[0..32].copy_from_slice(&[1; 32]);

        ahoy.execute(AhoyInstruction::Display {
            x_register: 0,
            y_register: 0,
            sprite_height: 15,
        })
        .unwrap();

        assert_eq!(ahoy.current_frame[0], 32_767);
        assert_eq!(ahoy.current_frame[1..], [0_u32; 63]);
    }

    #[test]
    fn instruction_drawing_sets_arbitraty_sprite_on_empty_frame() {
        let mut ahoy = Ahoy::default();
        ahoy.memory[0..32].copy_from_slice(&[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1,
            0, 0, 1,
        ]);
        ahoy.index = 17;

        ahoy.execute(AhoyInstruction::Display {
            x_register: 0,
            y_register: 0,
            sprite_height: 15,
        })
        .unwrap();

        assert_eq!(ahoy.current_frame[0], 1337);
    }

    #[test]
    fn instruction_drawing_treats_sprite_zero_bits_as_transparent() {
        let mut ahoy = Ahoy::default();
        ahoy.memory[0..4].copy_from_slice(&[1, 0, 0, 1]);
        ahoy.current_frame[0] = 0b0110;

        ahoy.execute(AhoyInstruction::Display {
            x_register: 0,
            y_register: 0,
            sprite_height: 4,
        })
        .unwrap();

        assert_eq!(ahoy.current_frame[0], 15);
    }

    #[test]
    fn instruction_drawing_considers_y_for_offset() {
        let mut ahoy = Ahoy::default();
        ahoy.memory[0..4].copy_from_slice(&[1, 0, 0, 1]);
        ahoy.current_frame[0] = 0b10010110;
        ahoy.registers[0xA] = 4;

        ahoy.execute(AhoyInstruction::Display {
            x_register: 0,
            y_register: 0xA,
            sprite_height: 4,
        })
        .unwrap();

        assert_eq!(ahoy.current_frame[0], 6);
    }

    #[test]
    fn instruction_drawing_sets_the_flag_register_when_a_bit_turned_off() {
        let mut ahoy = Ahoy::default();
        ahoy.memory[0..4].copy_from_slice(&[1, 0, 0, 1]);
        ahoy.current_frame[0] = 0b10010110;
        ahoy.registers[0xA] = 4;

        ahoy.execute(AhoyInstruction::Display {
            x_register: 0,
            y_register: 0xA,
            sprite_height: 4,
        })
        .unwrap();

        assert_eq!(ahoy.registers[0xF], 1);
    }

    #[test]
    fn instruction_drawing_unsets_the_flag_register_when_a_bits_were_only_turned_on() {
        let mut ahoy = Ahoy::default();
        ahoy.memory[0..32].copy_from_slice(&[1; 32]);

        ahoy.execute(AhoyInstruction::Display {
            x_register: 0,
            y_register: 0,
            sprite_height: 15,
        })
        .unwrap();
        assert_eq!(ahoy.registers[0xF], 0);
    }
}
