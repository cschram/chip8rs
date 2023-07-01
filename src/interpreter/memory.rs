use super::error::*;

const MEMORY_SIZE: usize = 4096;
const ROM_OFFSET: usize = 512;
pub const FONT_OFFSET: usize = 80;
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
    let mut mem = [0; MEMORY_SIZE];
    for i in 0..CHIP8_FONT.len() {
      mem[i + FONT_OFFSET] = CHIP8_FONT[i];
    }
    Self {
      mem,
    }
  }
}

impl Memory {
  pub fn read(&self, addr: usize, len: usize) -> Result<&[u8], InterpreterError> {
    let end = addr + len;
    if end < MEMORY_SIZE {
      Ok(&self.mem[addr..end])
    } else {
      Err(InterpreterError::InvalidAddressError(addr))
    }
  }

  pub fn read_byte(&self, addr: usize) -> Result<u8, InterpreterError> {
    if addr < MEMORY_SIZE {
      Ok(self.mem[addr])
    } else {
      Err(InterpreterError::InvalidAddressError(addr))
    }
  }

  pub fn write(&mut self, addr: usize, data: &[u8]) -> Result<(), InterpreterError> {
    if addr + data.len() >= 4096 {
      Err(InterpreterError::InvalidAddressError(addr))
    } else {
      for i in 0..data.len() {
        self.mem[addr + i] = data[i];
      }
      Ok(())
    }
  }

  pub fn write_byte(&mut self, addr: usize, byte: u8) -> Result<(), InterpreterError> {
    if addr >= 4096 {
      Err(InterpreterError::InvalidAddressError(addr))
    } else {
      self.mem[addr] = byte;
      Ok(())
    }
  }

  pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), InterpreterError> {
    self.write(ROM_OFFSET, rom)?;
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_read_write() {
    let mut mem = Memory::default();
    assert!(mem.write(0xF00, &[1, 2, 3]).is_ok());
    assert!(mem.write(0xFFF + 2, &[1, 2, 3]).is_err());
    assert_eq!(mem.read(0xF00, 3).unwrap(), &[1, 2, 3]);
    assert!(mem.write_byte(0xF00, 10).is_ok());
    assert!(mem.write_byte(0xFFF + 2, 10).is_err());
    assert_eq!(mem.read_byte(0xF00).unwrap(), 10);
  }
}
