use super::error::*;

const MAX_STACK: usize = 16;

pub enum Chip8Key {
  X,
  ONE,
  TWO,
  THREE,
  Q,
  W,
  E,
  A,
  S,
  D,
  Z,
  C,
  FOUR,
  R,
  F,
  V,
}

pub struct Registers {
  pub pc: u16,
  pub i: u16,
  pub stack: Vec<u16>,
  pub v: [u8; 16],
  pub keys: [bool; 16],
  pub delay_timer: f32,
  pub sound_timer: f32,
}

impl Default for Registers {
  fn default() -> Self {
      Self {
        pc: 512,
        i: 0,
        stack: Vec::new(),
        v: [0; 16],
        keys: [false; 16],
        delay_timer: 0.0,
        sound_timer: 0.0,
      }
  }
}

impl Registers {
  pub fn push(&mut self, addr: u16) -> Result<(), InterpreterError> {
    if self.stack.len() < MAX_STACK {
      self.stack.push(addr);
      Ok(())
    } else {
      println!("{:?}", self.stack);
      Err(InterpreterError::StackOverflow)
    }
  }

  pub fn pop(&mut self) -> Result<u16, InterpreterError> {
    self.stack.pop().ok_or(InterpreterError::StackUnderflow)
  }

  pub fn get_v(&self, index: usize) -> Result<u8, InterpreterError> {
    if index > 15 {
      Err(InterpreterError::InvalidRegister(index))
    } else {
      Ok(self.v[index])
    }
  }

  pub fn set_v(&mut self, index: usize, value: u8) -> Result<(), InterpreterError> {
    if index > 15 {
      Err(InterpreterError::InvalidRegister(index))
    } else {
      self.v[index] = value;
      Ok(())
    }
  }

  #[allow(dead_code)]
  pub fn get_vf(&self) -> u8 {
    self.v[15]
  }

  pub fn set_vf(&mut self, value: u8) {
    self.v[15] = value;
  }

  pub fn get_dt(&self) -> u8 {
    self.delay_timer as u8
  }

  pub fn set_dt(&mut self, value: u8) {
    self.delay_timer = value as f32;
  }

  pub fn get_st(&self) -> u8 {
    self.sound_timer as u8
  }

  pub fn set_st(&mut self, value: u8) {
    self.sound_timer = value as f32;
  }

  pub fn keydown(&self, index: usize) -> Result<bool, InterpreterError> {
    if index > 15 {
      Err(InterpreterError::InvalidKey(index))
    } else {
      Ok(self.keys[index])
    }
  }

  pub fn first_keydown(&self) -> Option<usize> {
    for i in 0..16 {
      if self.keys[i] {
        return Some(i);
      }
    }
    None
  }
}
