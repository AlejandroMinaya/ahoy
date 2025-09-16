use std::path::PathBuf;

use crossterm::event::{self, Event};
use ratatui::{
    self, Frame,
    widgets::canvas::{Canvas, Map, Rectangle},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg()]
    program: PathBuf,
}
fn render(frame: &mut Frame) {
    let area = frame.area();
    let area_width = f64::from(area.width);
    let area_height = f64::from(area.height);
    let rectangle_width = 10.0;
    let rectangle_height = 5.0;
    frame.render_widget(
        Canvas::default()
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| {
                ctx.draw(&Rectangle {
                    x: (area_width - rectangle_width) / 2.0,
                    y: (area_height - rectangle_height) / 2.0,
                    width: rectangle_width,
                    height: rectangle_height,
                    color: ratatui::style::Color::Yellow,
                });
            })
            .x_bounds([0.0, area_width])
            .y_bounds([0.0, area_height]),
        area,
    );
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
