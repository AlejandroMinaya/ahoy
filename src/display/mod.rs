use std::sync::mpsc::{Receiver, Sender};

pub mod native_display;
pub mod ratatui_display;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const PIXEL_COUNT: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;
pub const SPRITE_WIDTH: usize = 8;
pub type AhoyFrame = [u64; DISPLAY_HEIGHT];

#[derive(Debug)]
pub enum AhoyInputEvent {
    TurnOff,
}
#[derive(Debug)]
pub enum AhoyOutputEvent {
    NewFrame(AhoyFrame),
}

pub trait AhoyIO {
    fn connect_new(input_tx: Sender<AhoyInputEvent>, output_rx: Receiver<AhoyOutputEvent>) -> Self;
    fn draw(&mut self, frame: &AhoyFrame) -> anyhow::Result<()>;
    fn start(&mut self) -> anyhow::Result<()>;
}

struct Size {
    width: f64,
    height: f64,
}
impl Size {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}
impl Default for Size {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}
