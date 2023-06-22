use crate::error::*;

use rand::prelude::*;

pub struct Registers {
  pub pc: u16,
  pub i: u16,
  pub stack: Vec<u16>,
  pub v: [u8; 16],
  pub keys: [bool; 16],
  pub delay_timer: u8,
  pub sound_timer: u8,
  // I don't know where else to put this, so...
  pub rng: ThreadRng,
}

impl Default for Registers {
  fn default() -> Self {
      Self {
        pc: 512,
        i: 0,
        stack: Vec::new(),
        v: [0; 16],
        keys: [false; 16],
        delay_timer: 0,
        sound_timer: 0,
        rng: thread_rng(),
      }
  }
}

impl Registers {
  pub fn push(&mut self, addr: u16) {
    self.stack.push(addr);
  }

  pub fn pop(&mut self) -> Chip8Result<u16> {
    self.stack.pop().ok_or(Chip8Error::EmptyStack)
  }

  pub fn get_v(&self, index: usize) -> Chip8Result<u8> {
    if index > 15 {
      Err(Chip8Error::InvalidRegister(index))
    } else {
      Ok(self.v[index])
    }
  }

  pub fn set_v(&mut self, index: usize, value: u8) -> Chip8Result {
    if index > 15 {
      Err(Chip8Error::InvalidRegister(index))
    } else {
      self.v[index] = value;
      Ok(())
    }
  }

  pub fn _get_vf(&self) -> u8 {
    self.v[15]
  }

  pub fn set_vf(&mut self, value: u8) {
    self.v[15] = value;
  }

  pub fn keydown(&self, index: usize) -> Chip8Result<bool> {
    if index > 15 {
      Err(Chip8Error::InvalidKey(index))
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
