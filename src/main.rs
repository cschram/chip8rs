mod cpu;
mod emulator;
mod memory;

use crate::emulator::Emulator;
use ggez::{
    ContextBuilder,
    GameResult,
    conf::{WindowSetup, WindowMode, FullscreenType, NumSamples},
};

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("Chip 8 Emulator", "Corey Schram")
        .window_setup(WindowSetup {
            title: "Chip 8 Emulator".to_owned(),
            samples: NumSamples::One,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(WindowMode {
            width: 1024.0,
            height: 1024.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 1.0,
            max_width: 0.0,
            min_height: 1.0,
            max_height: 0.0,
            resizable: false,
            visible: true,
            resize_on_scale_factor_change: false,
            transparent: false,
            logical_size: None,
        });
    let (mut ctx, event_loop) = cb.build()?;
    let state = Emulator::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, state)
}
