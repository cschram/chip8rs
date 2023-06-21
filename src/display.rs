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
  pub data: [u8; 2048],
  pixel_mesh: Mesh
}

impl Display {
  pub fn new(gfx: &impl Has<GraphicsContext>) -> Chip8Result<Self> {
    Ok(Self {
      data: [0; 2048],
      pixel_mesh: Mesh::new_rectangle(
        gfx,
        DrawMode::fill(),
        Rect { x: 0.0, y: 0.0, w: DISPLAY_SCALE, h: DISPLAY_SCALE },
        Color::WHITE,
      )?
    })
  }

  pub fn clear(&mut self) {
    self.data = [0; 2048];
  }

  pub fn index(x: usize, y: usize) -> usize {
    (y * 64) + x
  }
}

impl Drawable for Display {
  fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
    for x in 0..64 {
      for y in 0..32 {
        if self.data[Display::index(x, y)] == 1 {
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
