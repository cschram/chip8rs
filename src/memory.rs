use std::io::{self, Read, BufReader};
use std::fs::File;
use std::path::PathBuf;

const ROM_OFFSET: usize = 512;

pub struct Memory {
  mem: [u8; 4096],
}

impl Memory {
  pub fn new() -> Self {
    Self {
      mem: [0; 4096],
    }
  }

  pub fn memcpy(&mut self, data: &Vec<u8>, offset: usize) {
    let end: usize = offset + data.len();
    for i in offset..end {
      if i < 4096 {
        self.mem[i] = data[i - offset];
      }
    }
  }

  pub fn read_rom(&mut self, path: &PathBuf) -> io::Result<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::<u8>::new();
    reader.read_to_end(&mut buffer)?;
    self.memcpy(&buffer, ROM_OFFSET);
    Ok(())
  }

  pub fn reset(&mut self) {
    self.mem = [0; 4096];
  }
}
