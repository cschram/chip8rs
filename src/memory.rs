use std::io::{Read, BufReader};
use std::fs::File;
use std::path::PathBuf;

use crate::error::*;

const MEMORY_SIZE: usize = 4096;
const ROM_OFFSET: usize = 512;

pub struct Memory {
  mem: [u8; MEMORY_SIZE],
}

impl Default for Memory {
  fn default() -> Self {
    Self {
      mem: [0; MEMORY_SIZE],
    }
  }
}

impl Memory {
  pub fn _reset(&mut self) {
    self.mem = [0; 4096];
  }

  pub fn _read(&self, pos: usize, len: usize) -> Option<&[u8]> {
    if pos + len < MEMORY_SIZE {
      Some(&self.mem[pos..len])
    } else {
      None
    }
  }

  pub fn write(&mut self, pos: usize, data: &[u8]) -> Chip8Result {
    if pos + data.len() >= 4096 {
      Err(Chip8Error::GenericError("Attempted to write data outside of emulator memory bounds".to_owned()))
    } else {
      for i in 0..data.len() {
        self.mem[pos + i] = data[i];
      }
      Ok(())
    }
  }

  pub fn load_rom(&mut self, path: &PathBuf) -> Chip8Result {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::<u8>::new();
    reader.read_to_end(&mut buffer)?;
    self.write(ROM_OFFSET, &buffer)?;
    Ok(())
  }
}
