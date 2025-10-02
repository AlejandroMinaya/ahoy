use crate::Ahoy;

#[derive(Debug)]
struct RegisterInstruction {
    register: u8,
    value: u8,
}
impl From<u16> for RegisterInstruction {
    fn from(value: u16) -> Self {
        let instruction = value & 0x0FFF;
        Self {
            register: (instruction >> 8) as u8,
            value: (instruction & 0x0FF) as u8,
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum AhoyInstruction {
    Jump(u16),
    CallSubroutine(u16),
    SetRegister(RegisterInstruction),
    AddToRegister(RegisterInstruction),
    SetIndex(u16),
    Display {
        x_register: u8,
        y_register: u8,
        sprite_height: u8,
    },
    ClearScreen = 0x00E0,
    StopSubroutine = 0x00EE,
    UnknownInstruction,
}

impl From<u16> for AhoyInstruction {
    fn from(value: u16) -> Self {
        match value {
            0x00E0 => Self::ClearScreen,
            0x00EE => Self::StopSubroutine,
            instruction => match instruction >> 0xC {
                1 => Self::Jump(instruction & 0x0FFF),
                2 => Self::CallSubroutine(instruction & 0x0FFF),
                6 => Self::SetRegister(RegisterInstruction::from(instruction)),
                7 => Self::AddToRegister(RegisterInstruction::from(instruction)),
                0xA => Self::SetIndex(instruction & 0x0FFF),
                0xD => Self::Display {
                    x_register: ((instruction >> 8) & 0xF) as u8,
                    y_register: ((instruction >> 4) & 0xF) as u8,
                    sprite_height: (instruction & 0xF) as u8,
                },
                _ => Self::UnknownInstruction,
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
            AhoyInstruction::SetRegister(RegisterInstruction {
                register: 0xF,
                value: 0xE0
            })
        ));
        assert!(matches!(
            0x6EEE.into(),
            AhoyInstruction::SetRegister(RegisterInstruction {
                register: 0xE,
                value: 0xEE
            })
        ));
        assert!(matches!(
            0x6A00.into(),
            AhoyInstruction::SetRegister(RegisterInstruction {
                register: 0xA,
                value: 0x00
            })
        ));
    }

    #[test]
    fn decode_add_value_to_register_instruction() {
        assert!(matches!(
            0x7808.into(),
            AhoyInstruction::AddToRegister(RegisterInstruction {
                register: 0x8,
                value: 0x8
            })
        ));
        assert!(matches!(
            0x7D1E.into(),
            AhoyInstruction::AddToRegister(RegisterInstruction {
                register: 0xD,
                value: 0x1E
            })
        ));
        assert!(matches!(
            0x7BEE.into(),
            AhoyInstruction::AddToRegister(RegisterInstruction {
                register: 0xB,
                value: 0xEE
            })
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
