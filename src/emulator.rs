use crate::{
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
        s.mem.read_rom("roms/IBM Logo.ch8")?;
        Ok(s)
    }
}

impl EventHandler<GameError> for Emulator {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.cpu.update(&mut self.mem)?;

        let gui_ctx = self.gui.ctx();
        egui::Window::new("Chip 8")
            .show(&gui_ctx, |ui| {
                ui.label("Emulator");
                if ui.button("Test 123").clicked() {
                    println!("test");
                }
            });
        self.gui.update(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(
            ctx,
            Color::from_rgb(131, 56, 236),
        );
        canvas.draw(
			&self.gui, 
			DrawParam::default(),
		);
        canvas.finish(ctx)
    }
}
