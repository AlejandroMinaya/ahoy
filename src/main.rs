mod display;

use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

use ahoy::Ahoy;
use cli_log::init_cli_log;
use crossterm::event::{self, Event};
use display::{AhoyDisplay, native_display::NativeDisplay, ratatui_display::RatatuiAhoyDisplay};

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

    // let mut display = RatatuiAhoyDisplay::default();
    let mut display = NativeDisplay::new()?;
    ahoy.process()?;
    display.draw(&ahoy.current_frame)?;
    ratatui::restore();
    Ok(())
}
