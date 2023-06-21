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

fn px_index(x: u16, y: u16) -> usize {
  ((y * 64) + x) as usize
}

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

  pub fn clear(&mut self) {
    self.data = [false; 2048];
  }

  pub fn pixel(&self, x: u16, y: u16) -> bool {
    self.data[px_index(x, y)]
  }

  pub fn set_pixel(&mut self, x: u16, y: u16, set: bool) {
    self.data[px_index(x, y)] = set;
  }
}

impl Drawable for Display {
  fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
    for x in 0..64 {
      for y in 0..32 {
        if self.data[px_index(x, y)] {
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
