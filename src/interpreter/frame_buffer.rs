use super::error::*;

pub const DISPLAY_SCALE: f32 = 10.0;
pub const SCREEN_WIDTH: f32 = 64.0 * DISPLAY_SCALE;
pub const SCREEN_HEIGHT: f32 = 32.0 * DISPLAY_SCALE;
const BUFFER_SIZE: usize = 2048;

pub struct FrameBuffer {
  buffer: [u8; BUFFER_SIZE],
}

impl Default for FrameBuffer {
  fn default() -> Self {
    Self {
      buffer: [0; BUFFER_SIZE],
    }
  }
}

impl FrameBuffer {
  pub fn get_i(&self, index: u16) -> Result<bool, InterpreterError> {
    if (index as usize) < BUFFER_SIZE {
      Ok(self.buffer[index as usize] == 1)
    } else {
      Err(InterpreterError::InvalidFrameBufferIndex(index))
    }
  }

  pub fn set_i(&mut self, index: u16, value: bool) -> Result<(), InterpreterError> {
    if (index as usize) < BUFFER_SIZE {
      self.buffer[index as usize] = if value { 1 } else { 0 };
      Ok(())
    } else {
      Err(InterpreterError::InvalidFrameBufferIndex(index))
    }
  }

  pub fn get_xy(&self, x: u16, y: u16) -> Result<bool, InterpreterError> {
    self.get_i((y * 64) + x)
  }

  pub fn set_xy(&mut self, x: u16, y: u16, value: bool) -> Result<(), InterpreterError> {
    self.set_i((y * 64) + x, value)
  }

  pub fn frame(&self) ->  &[u8] {
    &self.buffer
  }
}
