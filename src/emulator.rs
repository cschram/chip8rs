use crate::{
  colors,
  cpu::Cpu,
  memory::Memory,
};
use ggegui::{egui, Gui};
use ggez::{
  Context,
  GameResult,
  GameError,
  event::EventHandler,
  graphics::{Canvas, Color, DrawParam},
};

pub struct Emulator {
  cpu: Cpu,
  mem: Memory,
  gui: Gui,
}

impl Emulator {
  pub fn new(ctx: &mut Context) -> GameResult<Self> {
    let mut s = Self {
      cpu: Cpu::new(),
      mem: Memory::new(),
      gui: Gui::new(ctx),
    };

    let mut visuals = egui::Visuals::default();
    visuals.override_text_color = Some(colors::into_color32(colors::Text));
    visuals.panel_fill = colors::into_color32(colors::Background);
    let mut style = egui::Style::default();
    style.visuals = visuals;
    s.gui.ctx().set_style(style);

    // s.mem.read_rom("roms/IBM Logo.ch8")?;
    Ok(s)
  }
}

impl EventHandler<GameError> for Emulator {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.cpu.update(&mut self.mem)?;

    let gui_ctx = self.gui.ctx();
    egui::CentralPanel::default().show(&gui_ctx, |ui| {
      if ui.button("load").clicked() {
        println!("load rom");
      }
      if ui.button("reset").clicked() {
        println!("reset");
      }
      if ui.button("quit").clicked() {

      }
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
