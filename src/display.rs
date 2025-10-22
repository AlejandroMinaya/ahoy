use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Rectangle},
};

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub type AhoyFrame = [u64; DISPLAY_HEIGHT];

pub trait AhoyDisplay {
    fn draw(&mut self, frame: &AhoyFrame) -> anyhow::Result<()>;
}

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
struct Size {
    width: f64,
    height: f64,
}
impl Size {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    fn scale(self, factor: f64) -> Self {
        Self {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}
impl Default for Size {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}

impl AhoyDisplay for RatatuiAhoyDisplay {
    fn draw(&mut self, frame: &AhoyFrame) -> anyhow::Result<()> {
        let rectangle_size = Size::default().scale(10.0);
        let display_size = Size::new(
            64_f64 * rectangle_size.width,
            32_f64 * rectangle_size.height,
        );
        self.terminal.draw(|ratatui_frame| {
            let area = ratatui_frame.area();
            ratatui_frame.render_widget(
                Canvas::default()
                    .marker(ratatui::symbols::Marker::Dot)
                    .paint(|ctx| {
                        for i in 0..32_u8 {
                            for j in 0..64_u8 {
                                let row = 31_usize - usize::from(i);
                                let pixel = frame[row] >> (63 - j);
                                ctx.draw(&Rectangle {
                                    y: (rectangle_size.height * i as f64),
                                    x: (rectangle_size.width * j as f64),
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
}
