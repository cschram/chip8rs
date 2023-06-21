use crate::{
  display::Display,
  error::*,
  memory::Memory,
  registers::Registers,
};

pub struct Instruction {
  pub name: String,
  pub id: u16,
  pub mask: u16,
  pub op: fn(u16, &mut Memory, &mut Registers, &mut Display) -> Chip8Result,
}

impl Instruction {
  pub fn execute(&self, opcode: u16, mem: &mut Memory, registers: &mut Registers, display: &mut Display) -> Chip8Result {
    (self.op)(opcode, mem, registers, display)
  }
}

pub fn get_instructions() -> Vec<Instruction> {
  vec![
    Instruction {
      name: "CLS".to_owned(),
      id: 0x00E0,
      mask: 0x00F0,
      op: |_opcode, _mem, _registers, display| {
        display.clear();
        Ok(())
      }
    },
    Instruction {
      name: "JP addr".to_owned(),
      id: 0x1000,
      mask: 0xF000,
      op: |opcode, _mem, registers, _display| {
        let nnn = opcode & 0x0FFF;
        registers.pc = nnn;
        Ok(())
      }
    },
    Instruction {
      name: "LD Vx, byte".to_owned(),
      id: 0x6000,
      mask: 0xF000,
      op: |opcode, _mem, registers, _display| {
        let x = (opcode & 0x0F00) >> 8;
        let nn = opcode & 0x00FF;
        if x > 15 {
          Err(Chip8Error::InstructionExecutionError(
            opcode,
            format!("Invalid register address {}", x)
          ))
        } else {
          registers.v[x as usize] = nn as u8;
          Ok(())
        }
      }
    },
    Instruction {
      name: "ADD Vx, byte".to_owned(),
      id: 0x7000,
      mask: 0xF000,
      op: |opcode, _mem, registers, _display| {
        let x = (opcode & 0x0F00) >> 8;
        let nn = opcode & 0x00FF;
        registers.v[x as usize] += nn as u8;
        Ok(())
      }
    },
    Instruction {
      name: "LD I, addr".to_owned(),
      id: 0xA000,
      mask: 0xF000,
      op: |opcode, _mem, registers, _display| {
        let nnn = opcode & 0x0FFF;
        registers.i = nnn;
        Ok(())
      }
    },
    Instruction {
      name: "DRW Vx, Vy, nibble".to_owned(),
      id: 0xD000,
      mask: 0xF000,
      op: |opcode, mem, registers, display| {
        let n = opcode & 0x000F;
        let vx = registers.v[((opcode & 0x0F00) >> 8) as usize] & 63;
        let vy = registers.v[((opcode & 0x00F0) >> 4) as usize] & 31;
        registers.v[15] = 0;
        for y in 0..n {
          let byte = mem.read_byte((registers.i + y) as usize)?;
          let py = vy + y as u8;
          if py < 32 {
            for x in 0..8 {
              let px = vx + x;
              if px < 64 {
                let shift = 1 << x;
                let bit = if (byte & shift) == shift { 1 } else { 0 };
                if bit == 1 {
                  let di = Display::index(px as usize, py as usize);
                  if display.data[di] == 1 {
                    display.data[di] = 0;
                    registers.v[15] = 1;
                  } else {
                    display.data[di] = 1;
                  }
                }
              }
            }
          }
        }
        Ok(())
      }
    },
  ]
}
