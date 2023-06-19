use crate::memory::Memory;

use ggez::GameResult;

pub struct Cpu {
  _pc: u16,
}

impl Default for Cpu {
  fn default() -> Self {
    Self {
      _pc: 0,
    }
  }
}


impl Cpu {
  pub fn update(&mut self, _mem: &mut Memory) -> GameResult {
    Ok(())
  }
}
