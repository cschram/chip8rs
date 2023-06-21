use crate::{
  error::*,
  memory::Memory,
};

const INSTRUCTIONS_PER_SECOND: f32 = 700.0;


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
      pc: 512,
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
    self.pc = 512;
    self.l = 0;
    self.stack = Vec::new();
    self.delay_timer = 0;
    self.sound_timer = 0;
    self.registers = [0; 16];
  }

  pub fn tick(&mut self, mem: &mut Memory, delta: f32) -> Chip8Result {
    let num_instructions = (INSTRUCTIONS_PER_SECOND * delta) as u32;
    for _ in 0..num_instructions {
      self.execute(mem)?;
    }
    Ok(())
  }

  fn execute(&mut self, mem: &mut Memory) -> Chip8Result {
    match mem.read(self.pc as usize, 2) {
      Some(instruction) => {
        self.pc += 2;
        Ok(())
      },
      None => {
        Err(Chip8Error::InstructionError(self.pc, "Out of bounds".to_owned()))
      }
    }
  }
}
