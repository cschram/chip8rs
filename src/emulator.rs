use std::env;

use crate::{
  button::Button,
  cpu::Cpu,
  display::Display,
  error::Chip8Result,
  memory::Memory,
  theme,
};

use ggez::{
  Context,
  GameResult,
  GameError,
  event::{EventHandler, MouseButton},
  graphics::{Canvas, DrawParam},
  glam::Vec2,
};
use native_dialog::FileDialog;

const TARGET_FPS: u32 = 60;

pub const DISPLAY_SCALE: f32 = 10.0;

pub fn screen_width() -> f32 {
  64.0 * DISPLAY_SCALE
}

pub fn screen_height() -> f32 {
  (32.0 * DISPLAY_SCALE) + 40.0
}

pub struct Emulator {
  cpu: Cpu,
  mem: Memory,
  display: Display,
  rom_loaded: bool,
  load_button: Button,
  reset_button: Button,
}

impl Emulator {
  pub fn new(ctx: &mut Context) -> Chip8Result<Self> {
    let mut reset_button = Button::new("reset", 24.0, Vec2::new(0.0, 0.0), ctx)?;
    reset_button.pos = Vec2::new(
      screen_width() - reset_button.width(ctx),
      0.0,
    );

    Ok(Self {
      cpu: Cpu::default(),
      mem: Memory::default(),
      display: Display::new(ctx)?,
      rom_loaded: false,
      load_button: Button::new("load", 24.0, Vec2::ZERO, ctx)?,
      reset_button,
    })
  }

  pub fn load_rom(&mut self) {
    let cwd = env::current_dir().unwrap();
    let path = FileDialog::new()
      .set_location(&cwd)
      .add_filter("Chip 8 ROM", &["ch8"])
      .show_open_single_file()
      .unwrap();
    if let Some(path) = path {
      match self.mem.load_rom(&path) {
        Ok(_) => {
          self.rom_loaded = true;
          println!("Loaded {}", path.display())
        },
        Err(e) => println!("Could not load {}: {}", path.display(), &e),
      }
    }
  }
  
  pub fn reset(&mut self) {
    self.rom_loaded = false;
    self.cpu.reset();
    self.mem.reset();
    self.display.reset();
    println!("Reset emulator");
  }
}

impl EventHandler<GameError> for Emulator {
  fn mouse_button_down_event(
    &mut self,
    ctx: &mut Context,
    button: MouseButton,
    x: f32,
    y: f32,
  ) -> GameResult {
    if button == MouseButton::Left {
      if self.load_button.hover(x, y, ctx) {
        self.load_rom();
      }
      if self.reset_button.hover(x, y, ctx) {
        self.reset();
      }
    }
    Ok(())
  }

  fn update(&mut self, ctx: &mut Context) -> GameResult {
    while ctx.time.check_update_time(TARGET_FPS) {
      if self.rom_loaded {
        self.cpu.tick(&mut self.mem, ctx.time.delta().as_secs_f32())?;
      }
    }
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
    canvas.draw(
      &self.reset_button, 
      DrawParam::default(),
    );
    canvas.draw(&self.display, DrawParam::default());
    canvas.finish(ctx)
  }
}
