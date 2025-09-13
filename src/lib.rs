use anyhow::anyhow;
use std::io::BufRead;

const PROGRAM_MEMORY_START: usize = 0x200;
const AVAILABLE_PROGRAM_MEMORY: usize = 0x1000 - PROGRAM_MEMORY_START;

struct Chip8 {
    memory: [u8; 4096],
    index: u16,
    counter: u16,
}

impl Chip8 {
    fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            index: 0,
            counter: 0,
        }
    }
    fn load<R: BufRead>(&mut self, program_reader: &mut R) -> anyhow::Result<()> {
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
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::Chip8;

    #[test]
    fn load_normal_program() {
        let mut chip8 = Chip8::new();
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
        let mut chip8 = Chip8::new();
        let mut program_reader = BufReader::new(Cursor::new([]));

        chip8
            .load(&mut program_reader)
            .expect_err("Expected empty program to raise error");
    }

    #[test]
    fn load_returns_error_for_larger_than_buffer_file() {
        let mut chip8 = Chip8::new();
        let mut program_reader = BufReader::new(Cursor::new([1u8; 4096]));

        chip8
            .load(&mut program_reader)
            .expect_err("Expected large program to raise error");
    }
}
