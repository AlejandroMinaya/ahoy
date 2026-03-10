use std::sync::mpsc::{Receiver, Sender};

use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Rectangle},
};

use crate::display::AhoyDisplayEvents;

use super::{AhoyFrame, AhoyIO, DISPLAY_HEIGHT, DISPLAY_WIDTH, Size};

pub struct RatatuiAhoyDisplay {
    terminal: ratatui::DefaultTerminal,
}
impl Default for RatatuiAhoyDisplay {
    fn default() -> Self {
        Self {
            terminal: ratatui::init(),
        }
    }
}
impl Drop for RatatuiAhoyDisplay {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

impl AhoyIO for RatatuiAhoyDisplay {
    fn draw(&mut self, frame: &AhoyFrame) -> anyhow::Result<()> {
        let rectangle_size = Size::new(1.0, 1.0);
        let display_size = Size::new(
            DISPLAY_WIDTH as f64 * rectangle_size.width * 1.0,
            DISPLAY_HEIGHT as f64 * rectangle_size.height * 1.0,
        );
        self.terminal.draw(|ratatui_frame| {
            let area = ratatui_frame.area();
            ratatui_frame.render_widget(
                Canvas::default()
                    .marker(ratatui::symbols::Marker::Block)
                    .paint(|ctx| {
                        for (row_number, row) in frame.iter().rev().enumerate() {
                            for col in 0..DISPLAY_WIDTH {
                                let pixel = row >> col;
                                ctx.draw(&Rectangle {
                                    x: (rectangle_size.width * (DISPLAY_WIDTH - 1 - col) as f64),
                                    y: (rectangle_size.height * row_number as f64),
                                    width: rectangle_size.width,
                                    height: rectangle_size.height,
                                    color: if pixel & 0b1 == 1 {
                                        Color::White
                                    } else {
                                        Color::Black
                                    },
                                });
                            }
                        }
                    })
                    .x_bounds([0.0, display_size.width])
                    .y_bounds([0.0, display_size.height]),
                area,
            );
        })?;
        Ok(())
    }

    fn connect_new(io_tx: Sender<AhoyDisplayEvents>, processor_rx: Receiver<AhoyFrame>) -> Self {
        todo!()
    }

    fn start(&mut self, processor_rx: Receiver<AhoyFrame>) -> anyhow::Result<()> {
        todo!()
    }
}
