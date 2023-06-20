use crate::memory::Memory;

use ggez::GameResult;

pub struct Cpu {
  pc: u16,
  l: u16,
  stack: Vec<u16>,
  delay_timer: u8,
  sound_timer: u8,
  registers: [u8; 16],
}

impl Default for Cpu {
  fn default() -> Self {
    Self {
      pc: 0,
      l: 0,
      stack: Vec::new(),
      delay_timer: 0,
      sound_timer: 0,
      registers: [0; 16],
    }
  }
}

impl Cpu {
  pub fn reset(&mut self) {
    self.pc = 0;
    self.l = 0;
    self.stack = Vec::new();
    self.delay_timer = 0;
    self.sound_timer = 0;
    self.registers = [0; 16];
  }

  pub fn tick(&mut self, _mem: &mut Memory) -> GameResult {
    Ok(())
  }
}
