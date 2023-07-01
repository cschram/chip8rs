mod interpreter;
mod error;

use crate::{
  interpreter::Chip8,
  error::Chip8Error,
};

use std::{
  io::{Read, BufReader},
  fs::File,
  env::current_dir,
};
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::{
  dpi::LogicalSize,
  event::{Event, VirtualKeyCode},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use native_dialog::FileDialog;

pub fn main() -> Result<(), Chip8Error> {
  env_logger::init();

  // Chip8 init, load rom
  let mut chip8 = Chip8::default();
  let rom = {
    let cwd = current_dir().unwrap();
    let path = FileDialog::new()
      .set_location(&cwd)
      .add_filter("Chip-8 ROM", &["ch8"])
      .show_open_single_file()
      .unwrap();
    let f = File::open(path.expect("File missing")).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::<u8>::new();
    reader.read_to_end(&mut buffer).expect("Unable to read file");
    buffer
  };
  chip8.load_rom(&rom)?;

  // Window init
  let event_loop = EventLoop::new();
  let mut input = WinitInputHelper::new();
  let window = {
    let size = LogicalSize::new(Chip8::screen_width(), Chip8::screen_height());
    WindowBuilder::new()
      .with_title("Chip-8")
      .with_inner_size(size)
      .with_min_inner_size(size)
      .build(&event_loop)
      .unwrap()
  };

  // Pixel frame buffer init
  let mut pixels = {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(Chip8::screen_width() as u32, Chip8::screen_height() as u32, surface_texture)?
  };

  // Run event loop
  event_loop.run(move |event, _, control_flow| {
    if let Event::RedrawRequested(_) = event {
      // Copy interprefer frame buffer to pixels frame buffer
      let frame = chip8.frame();
      for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
        let x = (i % Chip8::screen_width() as usize) / Chip8::display_scale() as usize;
        let y = (i / Chip8::screen_width() as usize) / Chip8::display_scale() as usize;
        let frame_index = (y * 64) + x;
        let rgba = if frame[frame_index] == 1 {
          [0xFF, 0xFF, 0xFF, 0xFF]
        } else {
          [0x00, 0x00, 0x00, 0xFF]
        };
        pixel.copy_from_slice(&rgba);
      }

      // Render frame buffer
      if let Err(err) = pixels.render() {
        error!("{err}");
        *control_flow = ControlFlow::Exit;
        return;
      }
    }

    // Handle updates
    if input.update(&event) {
      if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
        *control_flow = ControlFlow::Exit;
        return;
      }

      if let Err(err) = chip8.update() {
        error!("{err}");
        *control_flow = ControlFlow::Exit;
        return;
      }
    }
  });
}
