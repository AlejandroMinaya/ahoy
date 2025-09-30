#[derive(Debug)]
struct JumpInstruction(u16);

#[repr(u16)]
#[derive(Debug)]
enum AhoyInstruction {
    ClearScreen = 0x00E0,
    Jump(JumpInstruction),
    UnknownInstruction,
}
impl From<u16> for AhoyInstruction {
    fn from(value: u16) -> Self {
        match value {
            0x00E0 => Self::ClearScreen,
            _ => Self::UnknownInstruction,
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
}
