use crate::{
  error::*,
  screen::Screen,
  instructions::*,
  memory::Memory,
  registers::Registers,
};

const INSTRUCTIONS_PER_SECOND: f32 = 700.0;


pub struct Cpu {
  instructions: Vec<Instruction>,
  registers: Registers,
}

impl Default for Cpu {
  fn default() -> Self {
    Self {
      registers: Registers::default(),
      instructions: get_instructions(),
    }
  }
}

impl Cpu {
  pub fn reset(&mut self) {
    self.registers = Registers::default();
  }

  pub fn tick(&mut self, mem: &mut Memory, display: &mut Screen, delta: f32) -> Chip8Result {
    if self.registers.delay_timer > 0 {
      self.registers.delay_timer -= 1;
    }
    if self.registers.sound_timer > 0 {
      self.registers.sound_timer -= 1;
    }
    let num_instructions = (INSTRUCTIONS_PER_SECOND * delta) as u32;
    for _ in 0..num_instructions {
      Instruction::execute(
        mem,
        &mut self.registers,
        display,
        &self.instructions
      )?;
    }
    Ok(())
  }

  pub fn keydown(&mut self, key: usize) {
    self.registers.keys[key] = true;
  }

  pub fn keyup(&mut self, key: usize) {
    self.registers.keys[key] = false;
  }
}
