mod display;

use std::path::PathBuf;

use crossterm::event::{self, Event};
use display::{AhoyDisplay, DISPLAY_WIDTH, RatatuiAhoyDisplay};
use ratatui::{
    self,
    style::Color,
    widgets::canvas::{Canvas, Rectangle},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg()]
    program: PathBuf,
}
fn main() -> anyhow::Result<()> {
    let mut display = RatatuiAhoyDisplay::default();
    loop {
        display.draw(&[0; DISPLAY_WIDTH])?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
