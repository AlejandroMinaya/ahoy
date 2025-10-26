#[derive(Debug)]
#[repr(u16)]
pub enum AhoyInstruction {
    Jump(usize),
    CallSubroutine(u16),
    SetRegister(usize, u8),
    AddToRegister(usize, u8),
    SetIndex(u16),
    Display {
        x_register: usize,
        y_register: usize,
        sprite_height: u8,
    },
    ClearScreen = 0x00E0,
    StopSubroutine = 0x00EE,
    UnknownInstruction(u16),
}

trait RegisterInstruction {
    fn into_regsiter_instruction(self) -> (u8, u8);
}

impl RegisterInstruction for u16 {
    fn into_regsiter_instruction(self) -> (u8, u8) {
        (((self >> 8) & 0xF) as u8, (self & 0x0FF) as u8)
    }
}

impl From<u16> for AhoyInstruction {
    fn from(value: u16) -> Self {
        match value {
            0x00E0 => Self::ClearScreen,
            0x00EE => Self::StopSubroutine,
            instruction => match instruction >> 0xC {
                1 => Self::Jump((instruction & 0x0FFF) as usize),
                2 => Self::CallSubroutine(instruction & 0x0FFF),
                6 => {
                    let (addr, value) = instruction.into_regsiter_instruction();
                    Self::SetRegister(addr as usize, value)
                }
                7 => {
                    let (addr, value) = instruction.into_regsiter_instruction();
                    Self::AddToRegister(addr as usize, value)
                }
                0xA => Self::SetIndex(instruction & 0x0FFF),
                0xD => Self::Display {
                    x_register: ((instruction >> 8) & 0xF) as usize,
                    y_register: ((instruction >> 4) & 0xF) as usize,
                    sprite_height: (instruction & 0xF) as u8,
                },
                _ => Self::UnknownInstruction(instruction),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::*;

    #[test]
    fn decode_static_instructions() {
        assert!(matches!(0x00E0.into(), AhoyInstruction::ClearScreen));
        assert!(matches!(0x00EE.into(), AhoyInstruction::StopSubroutine));
    }

    #[test]
    fn decode_jump_instruction() {
        assert!(matches!(0x1FE0.into(), AhoyInstruction::Jump(0xFE0)));
        assert!(matches!(0x1ABC.into(), AhoyInstruction::Jump(0xABC)));
        assert!(matches!(0x1000.into(), AhoyInstruction::Jump(0x000)));
    }

    #[test]
    fn decode_set_register_instruction() {
        assert!(matches!(
            0x6FE0.into(),
            AhoyInstruction::SetRegister(0xF, 0xE0)
        ));
        assert!(matches!(
            0x6EEE.into(),
            AhoyInstruction::SetRegister(0xE, 0xEE)
        ));
        assert!(matches!(
            0x6A00.into(),
            AhoyInstruction::SetRegister(0xA, 0x00)
        ));
    }

    #[test]
    fn decode_add_value_to_register_instruction() {
        assert!(matches!(
            0x7808.into(),
            AhoyInstruction::AddToRegister(0x8, 0x8)
        ));
        assert!(matches!(
            0x7D1E.into(),
            AhoyInstruction::AddToRegister(0xD, 0x1E)
        ));
        assert!(matches!(
            0x7BEE.into(),
            AhoyInstruction::AddToRegister(0xB, 0xEE)
        ));
    }

    #[test]
    fn decode_run_subroutine_instruction() {
        assert!(matches!(
            0x2FE0.into(),
            AhoyInstruction::CallSubroutine(0xFE0)
        ));
        assert!(matches!(
            0x2ABC.into(),
            AhoyInstruction::CallSubroutine(0xABC)
        ));
        assert!(matches!(
            0x2000.into(),
            AhoyInstruction::CallSubroutine(0x000)
        ));
    }

    #[test]
    fn decode_set_index_instruction() {
        assert!(matches!(0xAFE0.into(), AhoyInstruction::SetIndex(0xFE0)));
        assert!(matches!(0xAABC.into(), AhoyInstruction::SetIndex(0xABC)));
        assert!(matches!(0xA000.into(), AhoyInstruction::SetIndex(0x000)));
    }

    #[test]
    fn decode_display_instruction() {
        assert!(matches!(
            0xDFE0.into(),
            AhoyInstruction::Display {
                x_register: 0xF,
                y_register: 0xE,
                sprite_height: 0
            }
        ));
        assert!(matches!(
            0xDABC.into(),
            AhoyInstruction::Display {
                x_register: 0xA,
                y_register: 0xB,
                sprite_height: 0xC
            }
        ));
        assert!(matches!(
            0xD108.into(),
            AhoyInstruction::Display {
                x_register: 1,
                y_register: 0,
                sprite_height: 8
            }
        ));
    }
}
