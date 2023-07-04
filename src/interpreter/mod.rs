pub mod error;
mod frame_buffer;
mod instructions;
mod memory;
mod registers;

use self::{
  error::*,
  frame_buffer::*,
  instructions::*,
  memory::*,
  registers::*,
};
use std::time::Duration;
use rand::prelude::*;

const INSTRUCTIONS_PER_SECOND: f32 = 700.0;

pub struct Chip8 {
  memory: Memory,
  registers: Registers,
  frame_buffer: FrameBuffer,
  instructions: InstructionSet,
  rom: Option<Vec<u8>>,
  rng: ThreadRng,
}

impl Default for Chip8 {
  fn default() -> Self {
    Self {
      memory: Memory::default(),
      registers: Registers::default(),
      instructions: InstructionSet::default(),
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

  pub fn load_rom(&mut self, rom: &[u8]) -> InterpretterResult {
    self.memory.load_rom(&rom)?;
    self.rom = Some(Vec::from(rom));
    Ok(())
  }

  pub fn update(&mut self, delta: &Duration) -> InterpretterResult {
    let secs = delta.as_secs_f32();
    let num_instructions = (INSTRUCTIONS_PER_SECOND * secs) as usize;
    for _ in 0..num_instructions {
      self.instructions.execute(
        &mut self.memory,
        &mut self.registers,
        &mut self.frame_buffer,
        &mut self.rng,
      )?;
    }
    Ok(())
  }

  pub fn frame(&self) -> &[u8] {
    self.frame_buffer.frame()
  }
}
