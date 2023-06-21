use std::io::{Read, BufReader};
use std::fs::File;
use std::path::PathBuf;

use crate::error::*;

const MEMORY_SIZE: usize = 4096;
const ROM_OFFSET: usize = 512;
const FONT_OFFSET: usize = 80;

const CHIP8_FONT: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Memory {
  mem: [u8; MEMORY_SIZE],
}

impl Default for Memory {
  fn default() -> Self {
    let mut m = Self {
      mem: [0; MEMORY_SIZE],
    };
    m.load_font();
    m
  }
}

impl Memory {
  pub fn reset(&mut self) {
    self.mem = [0; 4096];
    self.load_font();
  }

  pub fn read(&self, pos: usize, len: usize) -> Option<&[u8]> {
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

  fn load_font(&mut self) {
    for i in 0..CHIP8_FONT.len() {
      self.mem[i + FONT_OFFSET] = CHIP8_FONT[i];
    }
  }
}
