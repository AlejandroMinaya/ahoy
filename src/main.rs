use std::path::PathBuf;

use crossterm::event::{self, Event};
use ratatui::{
    self, Frame,
    layout::{self, Constraint, Direction, Layout},
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
    let rectangle_width = 1.0;
    let rectangle_height = 1.0;
    frame.render_widget(
        Canvas::default()
            .marker(ratatui::symbols::Marker::Block)
            .paint(|ctx| {
                for i in 0..64 {
                    for j in 0..32 {
                        ctx.draw(&Rectangle {
                            x: f64::from(i) * rectangle_width,
                            y: area_height - rectangle_height - (rectangle_height * f64::from(j)),
                            width: rectangle_width,
                            height: rectangle_height,
                            color: ratatui::style::Color::White,
                        });
                    }
                }
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
