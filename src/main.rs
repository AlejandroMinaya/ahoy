mod display;

use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

use ahoy::Ahoy;
use cli_log::init_cli_log;
use crossterm::event::{self, Event};
use display::{AhoyDisplay, DISPLAY_HEIGHT, RatatuiAhoyDisplay};
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
    init_cli_log!();

    let args = Args::parse();
    let file = File::open(args.program)?;
    let mut reader = BufReader::new(file);

    let mut ahoy = Ahoy::default();
    ahoy.load(&mut reader)?;

    let mut display = RatatuiAhoyDisplay::default();
    loop {
        ahoy.process()?;
        display.draw(&ahoy.current_frame)?;
        if event::poll(Duration::from_millis(2))? && matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
