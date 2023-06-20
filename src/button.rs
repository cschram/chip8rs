use crate::{
  error::Chip8Result,
  theme,
};

use ggez::{
  context::{Context, Has},
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
  text: Text,
  mesh: Mesh,
  pos: Vec2,
}

impl Button {
  pub fn new(label: &str, size: f32, pos: Vec2, ctx: &Context) -> Chip8Result<Self> {
    let text = Text::new(TextFragment {
      text: label.to_owned(),
      color: Some(theme::TEXT),
      font: None,
      scale: Some(PxScale::from(size))
    });
    let text_dim = text.dimensions(ctx).unwrap();
    let mesh = Mesh::new_rectangle(
      ctx,
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

  pub fn hover(&self, x: f32, y: f32, ctx: &Context) -> bool {
    let dim = self.dimensions(ctx).unwrap();
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
