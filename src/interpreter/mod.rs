pub mod error;
mod frame_buffer;
mod memory;
mod registers;

pub use self::error::*;
use self::{
  frame_buffer::*,
  memory::*,
  registers::*,
};
use rand::prelude::*;

pub struct Chip8 {
  memory: Memory,
  registers: Registers,
  frame_buffer: FrameBuffer,
  rom: Option<Vec<u8>>,
  rng: ThreadRng,
}

impl Default for Chip8 {
  fn default() -> Self {
    Self {
      memory: Memory::default(),
      registers: Registers::default(),
      frame_buffer: FrameBuffer::default(),
      rom: None,
      rng: thread_rng(),
    }
  }
}

impl Chip8 {
  pub fn screen_width() -> f32 {
    SCREEN_WIDTH
  }
  
  pub fn screen_height() -> f32 {
    SCREEN_HEIGHT
  }

  pub fn display_scale() -> f32 {
    DISPLAY_SCALE
  }

  pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), InterpreterError> {
    self.memory.load_rom(&rom)?;
    self.rom = Some(Vec::from(rom));
    Ok(())
  }

  pub fn update(&mut self) -> Result<(), InterpreterError> {
    Ok(())
  }

  pub fn frame(&self) -> &[u8] {
    self.frame_buffer.frame()
  }
}
