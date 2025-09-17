mod display;

use std::path::PathBuf;

use crossterm::event::{self, Event};
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
const ALL_PIXELS: crate::display::Frame = [0; 32];

fn render(frame: &mut ratatui::Frame) {
    let area = frame.area();
    let area_width = f64::from(area.width);
    let area_height = f64::from(area.height);
    let rectangle_width = 1.0;
    let rectangle_height = 1.0;
    frame.render_widget(
        Canvas::default()
            .marker(ratatui::symbols::Marker::Dot)
            .paint(|ctx| {
                for i in 0..32_u8 {
                    for j in 0..64_u8 {
                        ctx.draw(&Rectangle {
                            y: area_height - rectangle_height - (rectangle_height * f64::from(i)),
                            x: f64::from(j) * rectangle_width,
                            width: rectangle_width,
                            height: rectangle_height,
                            color: if (ALL_PIXELS[usize::from(i)] >> j) & 0b1 == 1 {
                                Color::White
                            } else {
                                Color::Black
                            },
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
