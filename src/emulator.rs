use std::env;

use crate::{
  button::Button,
  cpu::Cpu,
  error::Chip8Result,
  memory::Memory,
  theme,
};

use ggez::{
  Context,
  GameResult,
  GameError,
  event::EventHandler,
  graphics::{Canvas, DrawParam},
  glam::Vec2,
};
use native_dialog::FileDialog;

pub struct Emulator {
  cpu: Cpu,
  mem: Memory,
  load_button: Button,
}

impl Emulator {
  pub fn new(ctx: &mut Context) -> Chip8Result<Self> {
    Ok(Self {
      cpu: Cpu::default(),
      mem: Memory::default(),
      load_button: Button::new("Load", Vec2::ZERO, ctx)?,
    })
  }

  pub fn _load_rom(&mut self) {
    let cwd = env::current_dir().unwrap();
    let path = FileDialog::new()
      .set_location(&cwd)
      .add_filter("Chip 8 ROM", &["ch8"])
      .show_open_single_file()
      .unwrap();
    if let Some(path) = path {
      match self.mem.load_rom(&path) {
        Ok(_) => println!("Loaded {}", path.display()),
        Err(e) => println!("Could not load {}: {}", path.display(), &e),
      }
    }
  }
}

impl EventHandler<GameError> for Emulator {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    self.cpu.update(&mut self.mem)?;
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(
      ctx,
      theme::BACKGROUND,
    );
    canvas.draw(
      &self.load_button, 
      DrawParam::default(),
    );
    canvas.finish(ctx)
  }
}
