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
  pub debug: bool,
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
      name: "0x00E0 CLS".to_owned(),
      id: 0x00E0,
      mask: 0x00F0,
      debug: false,
      op: |_opcode, _mem, _registers, display| {
        display.clear();
        Ok(())
      }
    },
    Instruction {
      name: "0x1nnn JP addr".to_owned(),
      id: 0x1000,
      mask: 0xF000,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let nnn = opcode & 0x0FFF;
        registers.pc = nnn;
        Ok(())
      }
    },
    Instruction {
      name: "0x6xnn LD Vx, byte".to_owned(),
      id: 0x6000,
      mask: 0xF000,
      debug: true,
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
      name: "0x7xnn ADD Vx, byte".to_owned(),
      id: 0x7000,
      mask: 0xF000,
      debug: true,
      op: |opcode, _mem, registers, _display| {
        let x = (opcode & 0x0F00) >> 8;
        let nn = opcode & 0x00FF;
        registers.v[x as usize] += nn as u8;
        Ok(())
      }
    },
    Instruction {
      name: "0xAnnn LD I, addr".to_owned(),
      id: 0xA000,
      mask: 0xF000,
      debug: true,
      op: |opcode, _mem, registers, _display| {
        let nnn = opcode & 0x0FFF;
        registers.i = nnn;
        Ok(())
      }
    },
    Instruction {
      name: "0xDxyn DRW Vx, Vy, nibble".to_owned(),
      id: 0xD000,
      mask: 0xF000,
      debug: true,
      op: |opcode, mem, registers, display| {
        // Read arguments
        let vx = registers.v[((opcode & 0x0F00) >> 8) as usize] % 64;
        let vy = registers.v[((opcode & 0x00F0) >> 4) as usize] % 32;
        let n = opcode & 0x000F;

        // Reset VF register
        registers.v[15] = 0;

        for y in 0..n {
          let py = vy as u16 + y;
          if py < 32 {
            let byte = mem.read_byte((registers.i + y) as usize)?;
            for x in 0..8 {
              let px = vx as u16 + x;
              if px < 64 && (byte & (1 << x)) > 0 {
                // Flip display pixel if sprite bit is set
                println!("({},{})", px, py);
                if display.pixel(px, py) {
                  display.set_pixel(px, py, false);
                  registers.v[15] = 1;
                } else {
                  display.set_pixel(px, py, true);
                }
              }
            }
          }
        }

        println!("VF: {}", registers.v[15]);
        Ok(())
      }
    },
  ]
}
