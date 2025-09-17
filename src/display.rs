use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Rectangle},
};

pub type Frame = [u64; 32];

pub trait AhoyDisplay {
    fn draw(&mut self, frame: &Frame) -> anyhow::Result<()>;
}

struct RatatuiAhoyDisplay {
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
impl AhoyDisplay for RatatuiAhoyDisplay {
    fn draw(&mut self, frame: &Frame) -> anyhow::Result<()> {
        self.terminal.draw(|ratatui_frame| {
            let area = ratatui_frame.area();
            let area_width = f64::from(area.width);
            let area_height = f64::from(area.height);
            ratatui_frame.render_widget(
                Canvas::default()
                    .marker(ratatui::symbols::Marker::Dot)
                    .paint(|ctx| {
                        for i in 0..32_u8 {
                            for j in 0..64_u8 {
                                ctx.draw(&Rectangle {
                                    y: area_height - 1.0 - (1.0 * f64::from(i)),
                                    x: f64::from(j),
                                    width: 1.0,
                                    height: 1.0,
                                    color: if (frame[usize::from(i)] >> j) & 0b1 == 1 {
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
        })?;
        Ok(())
    }
}
