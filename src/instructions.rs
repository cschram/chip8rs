use crate::{
  display::Display,
  error::*,
  memory::{Memory, FONT_OFFSET},
  registers::Registers,
};

use rand::Rng;

pub struct Instruction {
  pub name: String,
  pub id: u16,
  pub mask: u16,
  pub debug: bool,
  pub op: fn(u16, &mut Memory, &mut Registers, &mut Display) -> Chip8Result,
}

impl Instruction {
  pub fn execute(
    &self, opcode: u16,
    mem: &mut Memory,
    registers: &mut Registers,
    display: &mut Display
  ) -> Chip8Result {
    (self.op)(opcode, mem, registers, display)
  }
}

fn i_sys_addr() -> Instruction {
  Instruction {
    name: "0nnn SYS addr".to_owned(),
    id: 0x0000,
    mask: 0xF000,
    debug: false,
    op: |_opcode, _mem, registers, _display| {
      registers.pc += 2;
      Ok(())
    }
  }
}

fn i_cls() -> Instruction {
  Instruction {
    name: "00E0 CLS".to_owned(),
    id: 0x00E0,
    mask: 0x00F0,
    debug: false,
    op: |_opcode, _mem, registers, display| {
      display.clear();
      registers.pc += 2;
      Ok(())
    }
  }
}

fn i_ret() -> Instruction {
  Instruction {
    name: "00EE RET".to_owned(),
    id: 0x00EE,
    mask: 0x00FF,
    debug: false,
    op: |_opcode, _mem, registers, _display| {
      registers.pc = registers.pop()?;
      Ok(())
    }
  }
}

fn i_jp_addr() -> Instruction {
  Instruction {
    name: "1nnn JP addr".to_owned(),
    id: 0x1000,
    mask: 0xF000,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let nnn = opcode & 0x0FFF;
      registers.pc = nnn;
      Ok(())
    }
  }
}

fn i_call_addr() -> Instruction {
  Instruction {
    name: "2nnn CALL addr".to_owned(),
    id: 0x2000,
    mask: 0xF000,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let nnn = opcode & 0x0FFF;
      registers.push(registers.pc);
      registers.pc = nnn;
      Ok(())
    }
  }
}

fn i_se_vx_byte() -> Instruction {
  Instruction {
    name: "3xnn SE Vx, byte".to_owned(),
    id: 0x3000,
    mask: 0xF000,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let nn = (opcode & 0x00FF) as u8;
      let vx = registers.get_v(x)?;
      if vx == nn {
        registers.pc += 4;
      } else {
        registers.pc += 2;
      }
      Ok(())
    }
  }
}

fn i_sne_vx_byte() -> Instruction {
  Instruction {
    name: "4xnn SNE Vx, byte".to_owned(),
    id: 0x4000,
    mask: 0xF000,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let nn = (opcode & 0x00FF) as u8;
      let vx = registers.get_v(x)?;
      if vx != nn {
        registers.pc += 4;
      } else {
        registers.pc += 2;
      }
      Ok(())
    }
  }
}

fn i_se_vx_vy() -> Instruction {
  Instruction {
    name: "5xy0 SE Vx, Vy".to_owned(),
    id: 0x5000,
    mask: 0xF00F,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = (opcode & 0x0F00) >> 8;
      let y = (opcode & 0x00F0) >> 4;
      if x == y {
        registers.pc += 4;
      } else {
        registers.pc += 2;
      }
      Ok(())
    }
  }
}

fn i_ld_vx_byte() -> Instruction {
  Instruction {
    name: "6xnn LD Vx, byte".to_owned(),
    id: 0x6000,
    mask: 0xF000,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let nn = (opcode & 0x00FF) as u8;
      registers.set_v(x, nn)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn i_add_vx_byte() -> Instruction {
  Instruction {
    name: "7xnn ADD Vx, byte".to_owned(),
    id: 0x7000,
    mask: 0xF000,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let nn = (opcode & 0x00FF) as u16;
      let vx = registers.get_v(x)? as u16;
      registers.set_v(x, (vx + nn) as u8)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn i_ld_vx_vy() -> Instruction {
  Instruction {
    name: "8xy0 LD Vx, Vy".to_owned(),
    id: 0x8000,
    mask: 0xF00F,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      registers.set_v(x, registers.get_v(y)?)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn i_or_vx_vy() -> Instruction {
  Instruction {
    name: "8xy1 OR Vx, Vy".to_owned(),
    id: 0x8001,
    mask: 0xF00F,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      let vx = registers.get_v(x)?;
      let vy = registers.get_v(y)?;
      registers.set_v(x, vx | vy)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn i_and_vx_vy() -> Instruction {
  Instruction {
    name: "8xy2 AND Vx, Vy".to_owned(),
    id: 0x8002,
    mask: 0xF00F,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      let vx = registers.get_v(x)?;
      let vy = registers.get_v(y)?;
      registers.set_v(x, vx & vy)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn i_xor_vx_vy() -> Instruction {
  Instruction {
    name: "8xy3 XOR Vx, Vy".to_owned(),
    id: 0x8003,
    mask: 0xF00F,
    debug: false,
    op: |opcode, _mem, registers, _display| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      let vx = registers.get_v(x)?;
      let vy = registers.get_v(y)?;
      registers.set_v(x, vx ^ vy)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

pub fn get_instructions() -> Vec<Instruction> {
  vec![
    i_sys_addr(),
    i_cls(),
    i_ret(),
    i_jp_addr(),
    i_call_addr(),
    i_se_vx_byte(),
    i_sne_vx_byte(),
    i_se_vx_vy(),
    i_ld_vx_byte(),
    i_add_vx_byte(),
    i_ld_vx_vy(),
    i_or_vx_vy(),
    i_and_vx_vy(),
    i_xor_vx_vy(),
    Instruction {
      name: "8xy4 ADD Vx, Vy".to_owned(),
      id: 0x8004,
      mask: 0xF00F,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = registers.get_v(x)?;
        let vy = registers.get_v(y)?;
        let sum = vx as u16 + vy as u16;
        registers.set_v(x, sum as u8)?;
        registers.set_vf(if sum > 255 { 1 } else { 0 });
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "8xy5 SUB Vx, Vy".to_owned(),
      id: 0x8005,
      mask: 0xF00F,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = registers.get_v(x)?;
        let vy = registers.get_v(y)?;
        registers.set_v(x, vx - vy)?;
        registers.set_vf(if vx > vy { 1 } else { 0 });
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "8x06 SHR Vx".to_owned(),
      id: 0x8006,
      mask: 0xF00F,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let vx = registers.get_v(x)?;
        registers.set_v(x, vx >> 1)?;
        registers.set_vf(if vx & 1 == 1 { 1 } else { 0 });
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "8xy7 SUBN Vx, Vy".to_owned(),
      id: 0x8007,
      mask: 0xF00F,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = registers.get_v(x)?;
        let vy = registers.get_v(y)?;
        registers.set_v(x, vy - vx)?;
        registers.set_vf(if vy > vx { 1 } else { 0 });
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "8x0E SHL Vx".to_owned(),
      id: 0x800E,
      mask: 0xF00F,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let vx = registers.get_v(x)?;
        registers.set_v(x, vx << 1)?;
        registers.set_vf(if vx & 0x80 == 1 { 1 } else { 0 });
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "9xy0 SNE Vx, Vy".to_owned(),
      id: 0x9000,
      mask: 0xF00F,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = registers.get_v(x)?;
        let vy = registers.get_v(y)?;
        if vx != vy {
          registers.pc += 4;
        } else {
          registers.pc += 2;
        }
        Ok(())
      }
    },
    Instruction {
      name: "Annn LD I, addr".to_owned(),
      id: 0xA000,
      mask: 0xF000,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let nnn = opcode & 0x0FFF;
        registers.i = nnn;
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Bnnn JP V0, addr".to_owned(),
      id: 0xB000,
      mask: 0xF000,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let nnn = opcode & 0x0FFF;
        let v0 = registers.get_v(0)? as u16;
        registers.pc = nnn + v0;
        Ok(())
      }
    },
    Instruction {
      name: "Cxnn RND Vx, byte".to_owned(),
      id: 0xC000,
      mask: 0xF000,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = opcode & 0x00FF;
        let r = registers.rng.gen_range(0..256);
        registers.set_v(x, (r & nn) as u8)?;
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Dxyn DRW Vx, Vy, nibble".to_owned(),
      id: 0xD000,
      mask: 0xF000,
      debug: false,
      op: |opcode, mem, registers, display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = opcode & 0x000F;
        let vx = registers.get_v(x)? % 64;
        let vy = registers.get_v(y)? % 32;

        registers.v[15] = 0;
        for yi in 0..n {
          let py = vy as u16 + yi;
          if py < 32 {
            let byte = mem.read_byte((registers.i + yi) as usize)?;
            for xi in 0..8 {
              let px = vx as u16 + xi;
              if px < 64 && (byte & (0x80 >> xi)) != 0 {
                // Flip display pixel if sprite bit is set
                if display.pixel(px, py) {
                  display.set_pixel(px, py, false);
                  // Set the VF register if a pixel is unset
                  registers.v[15] = 1;
                } else {
                  display.set_pixel(px, py, true);
                }
              }
            }
          }
        }

        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Ex9E SKP Vx".to_owned(),
      id: 0xE09E,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let vx = registers.get_v(x)? as usize;
        if registers.keydown(vx)? {
          registers.pc += 4;
        } else {
          registers.pc += 2;
        }
        Ok(())
      }
    },
    Instruction {
      name: "ExA1 SKNP Vx".to_owned(),
      id: 0xE0A1,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let vx = registers.get_v(x)? as usize;
        if registers.keydown(vx)? {
          registers.pc += 2;
        } else {
          registers.pc += 4;
        }
        Ok(())
      }
    },
    Instruction {
      name: "Fx07 LD Vx, DT".to_owned(),
      id: 0xF007,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        registers.set_v(x, registers.delay_timer)?;
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Fx0A LD Vx, K".to_owned(),
      id: 0xF00A,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        if let Some(key) = registers.first_keydown() {
          let x = ((opcode & 0x0F00) >> 8) as usize;
          registers.set_v(x, key as u8)?;
          registers.pc += 2;
        }
        Ok(())
      }
    },
    Instruction {
      name: "Fx15 LD DT, Vx".to_owned(),
      id: 0xF015,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        registers.delay_timer = registers.get_v(x)?;
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Fx18 LD ST, Vx".to_owned(),
      id: 0xF018,
      mask: 0xF0FF,
      debug: true,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        registers.sound_timer = registers.get_v(x)?;
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Fx1E ADD I, Vx".to_owned(),
      id: 0xF01E,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        registers.i += registers.get_v(x)? as u16;
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Fx29 LD F, Vx".to_owned(),
      id: 0xF029,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, _mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let vx = registers.get_v(x)? as u16;
        registers.i = FONT_OFFSET as u16 + (vx * 5);
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Fx33 LD B, Vx".to_owned(),
      id: 0xF033,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, mem, registers, _display| {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let vx = registers.get_v(x)?;
        mem.write_byte(registers.i as usize, vx / 100)?;
        mem.write_byte(registers.i as usize, (vx / 10) % 10)?;
        mem.write_byte(registers.i as usize, (vx % 100) % 10)?;
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Fx55 LD [I], Vx".to_owned(),
      id: 0xF055,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, mem, registers, _display| {
        let x = (opcode & 0x0F00) >> 8;
        for i in 0..x {
          mem.write_byte(
            (registers.i + i) as usize,
            registers.get_v(i as usize)?
          )?;
        }
        registers.pc += 2;
        Ok(())
      }
    },
    Instruction {
      name: "Fx65 LD Vx, [I]".to_owned(),
      id: 0xF065,
      mask: 0xF0FF,
      debug: false,
      op: |opcode, mem, registers, _display| {
        let x = (opcode & 0x0F00) >> 8;
        for i in 0..x {
          registers.set_v(
            i as usize,
            mem.read_byte((registers.i + i) as usize)?
          )?;
        }
        registers.pc += 2;
        Ok(())
      }
    },
  ]
}
