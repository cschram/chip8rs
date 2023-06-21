use crate::{
  error::Chip8Result,
  theme,
};

use ggez::{
  context::Has,
  graphics::{
    Canvas,
    Drawable,
    DrawMode,
    DrawParam,
    GraphicsContext,
    Mesh,
    PxScale,
    Text,
    TextFragment,
    Rect,
  },
  glam::Vec2,
};

const PADDING: Vec2 = Vec2 {
  x: 8.0,
  y: 8.0,
};

pub struct Button {
  pub pos: Vec2,
  
  text: Text,
  mesh: Mesh,
}

impl Button {
  pub fn new(label: &str, size: f32, pos: Vec2, gfx: &impl Has<GraphicsContext>) -> Chip8Result<Self> {
    let text = Text::new(TextFragment {
      text: label.to_owned(),
      color: Some(theme::TEXT),
      font: None,
      scale: Some(PxScale::from(size))
    });
    let text_dim = text.dimensions(gfx).unwrap();
    let mesh = Mesh::new_rectangle(
      gfx,
      DrawMode::fill(),
      Rect::new(0.0, 0.0, text_dim.w + (PADDING.x * 2.0), text_dim.h + (PADDING.y * 2.0)),
      theme::WIDGET,
    )?;
    Ok(Self {
      text,
      mesh,
      pos,
    })
  }

  pub fn width(&self, gfx: &impl Has<GraphicsContext>) -> f32 {
    let dim = self.dimensions(gfx).unwrap();
    dim.w
  }

  pub fn height(&self, gfx: &impl Has<GraphicsContext>) -> f32 {
    let dim = self.dimensions(gfx).unwrap();
    dim.h
  }

  pub fn hover(&self, x: f32, y: f32, gfx: &impl Has<GraphicsContext>) -> bool {
    let dim = self.dimensions(gfx).unwrap();
    x >= dim.x && x <= dim.right() && y >= dim.y && y <= dim.bottom()
  }
}

impl Drawable for Button {
  fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
    canvas.draw(&self.mesh, self.pos);
    canvas.draw(&self.text, self.pos + PADDING);
  }

  fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
    let mesh_dim = self.mesh.dimensions(gfx)?;
    Some(Rect {
      x: self.pos.x,
      y: self.pos.y,
      w: mesh_dim.w,
      h: mesh_dim.h,
    })
  }
}
