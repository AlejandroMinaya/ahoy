#[repr(u16)]
#[derive(Debug)]
enum AhoyInstruction {
    ClearScreen = 0x00E0,
    Jump(u16),
    UnknownInstruction,
}
impl From<u16> for AhoyInstruction {
    fn from(value: u16) -> Self {
        match value {
            0x00E0 => Self::ClearScreen,
            instruction => match instruction >> 0xC {
                1 => Self::Jump(instruction & 0x0FFF),
                _ => Self::UnknownInstruction,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::*;

    #[test]
    fn from_decodes_static_instructions() {
        assert!(matches!(0x00E0.into(), AhoyInstruction::ClearScreen));
    }

    #[test]
    fn from_decode_jump_instruction() {
        assert!(matches!(0x1FE0.into(), AhoyInstruction::Jump(0xFE0)));
        assert!(matches!(0x1ABC.into(), AhoyInstruction::Jump(0xABC)));
        assert!(matches!(0x1000.into(), AhoyInstruction::Jump(0x000)));

        assert!(!matches!(0x2ABC.into(), AhoyInstruction::Jump(0xABC)));
    }
}
