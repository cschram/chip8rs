use std::env;

use crate::{
  cpu::Cpu,
  memory::Memory,
  theme,
};

use ggegui::{egui, Gui};
use ggez::{
  Context,
  GameResult,
  GameError,
  event::EventHandler,
  graphics::{Canvas, Color, DrawParam},
};
use native_dialog::FileDialog;

pub struct Emulator {
  cpu: Cpu,
  mem: Memory,
  gui: Gui,
}

impl Emulator {
  pub fn new(ctx: &mut Context) -> Self {
    let mut s = Self {
      cpu: Cpu::default(),
      mem: Memory::default(),
      gui: Gui::new(ctx),
    };
    s.gui.ctx().set_style(theme::egui_style());
    s
  }
}

impl EventHandler<GameError> for Emulator {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.cpu.update(&mut self.mem)?;

    let gui_ctx = self.gui.ctx();
    egui::CentralPanel::default().show(&gui_ctx, |ui| {
      ui.heading(egui::RichText::from("Chip 8"));
      ui.horizontal(|ui| {
        if ui.button("load").clicked() {
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
        if ui.button("reset").clicked() {
          self.mem.reset();
        }
        if ui.button("quit").clicked() {
          ctx.request_quit();
        }
      });
    });
    self.gui.update(ctx);

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(
      ctx,
      Color::BLACK,
    );
    canvas.draw(
      &self.gui, 
      DrawParam::default(),
    );
    canvas.finish(ctx)
  }
}
