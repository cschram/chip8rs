use crate::memory::Memory;
use ggez::GameResult;

pub struct Cpu {
  pc: u16,
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      pc: 0,
    }
  }

  pub fn update(&mut self, mem: &mut Memory) -> GameResult {
    Ok(())
  }
}
