use crate::{
  error::Chip8Result,
  emulator::DISPLAY_SCALE,
};

use ggez::{
  context::Has,
  graphics::{
    Canvas,
    Color,
    Drawable,
    DrawMode,
    DrawParam,
    GraphicsContext,
    Mesh,
    Rect,
  },
};

pub struct Display {
  pub data: [bool; 2048],
  pixel_mesh: Mesh
}

impl Display {
  pub fn new(gfx: &impl Has<GraphicsContext>) -> Chip8Result<Self> {
    Ok(Self {
      data: [false; 2048],
      pixel_mesh: Mesh::new_rectangle(
        gfx,
        DrawMode::fill(),
        Rect { x: 0.0, y: 0.0, w: DISPLAY_SCALE, h: DISPLAY_SCALE },
        Color::WHITE,
      )?
    })
  }

  pub fn reset(&mut self) {
    self.data = [false; 2048];
  }
}

impl Drawable for Display {
  fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
    for x in 0..64 {
      for y in 0..32 {
        if self.data[(y * 64) + x] {
          canvas.draw(
            &self.pixel_mesh,
            [x as f32 * DISPLAY_SCALE, (y as f32 * DISPLAY_SCALE) + 40.0],
          );
        }
      }
    }
  }

  fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
    Some(Rect {
      x: 0.0,
      y: 0.0,
      w: 64.0 * DISPLAY_SCALE,
      h: 32.0 * DISPLAY_SCALE,
    })
  }
}
