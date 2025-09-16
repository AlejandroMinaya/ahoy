use std::path::PathBuf;

use crossterm::event::{self, Event};
use ratatui::{self, Frame};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg()]
    program: PathBuf,
}
fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
