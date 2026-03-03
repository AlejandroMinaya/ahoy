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
    /* TODO:
     * You have two competing app loops. The one below this comment with its own key handling,
     * and the one within the NativeDisplay event_loop. I believe it makes sense to choose only
     * one or to add to the AhoyDisplay a looping/event-handling situation since for ratatui
     * it makes sense to use the crossterm events but for wgpu it makes sense to use the
     * winit events.
     * */
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
