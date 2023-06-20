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
    Text,
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
  pub fn new(label: &str, pos: Vec2, ctx: &Context) -> Chip8Result<Self> {
    let mut text = Text::new(label);
    text.set_scale(24.0);
    let text_dim = text.dimensions(ctx).unwrap();
    println!("{:?}", text_dim);
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
}

impl Drawable for Button {
  fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
    canvas.draw(&self.mesh, self.pos);
    canvas.draw(
      &self.text, 
      DrawParam::default()
        .color(theme::TEXT)
        .offset(self.pos + PADDING)
    );
  }

  fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
    self.mesh.dimensions(gfx)
  }
}
