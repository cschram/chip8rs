use crate::{
  screen::{Chip8Screen},
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
  pub execute: fn(u16, &mut Memory, &mut Registers, &mut dyn Chip8Screen) -> Chip8Result,
}

impl Instruction {
  pub fn execute(
    mem: &mut Memory,
    registers: &mut Registers,
    display: &mut impl Chip8Screen,
    instructions: &Vec<Instruction>
  ) -> Chip8Result {
    let pc = registers.pc as usize;

    let opbytes = mem.read(pc, 2)?;
    let opcode = (opbytes[0] as u16) << 8 | (opbytes[1] as u16);

    match Instruction::decode(opcode, instructions) {
      Some(instr) => {
        if instr.debug {
          println!("{:#06x} | {}", opcode, instr.name);
        }
        (instr.execute)(opcode, mem, registers, display)
      },
      None => {
        return Err(Chip8Error::InvalidInstructionError(pc, opcode))
      }
    }
  }

  fn decode(opcode: u16, instructions: &Vec<Instruction>) -> Option<&Instruction> {
    instructions.iter().find(|instr| {
      (opcode & instr.mask) == instr.id
    })
  }
}

fn sys_addr() -> Instruction {
  Instruction {
    name: "0nnn SYS addr".to_owned(),
    id: 0x0000,
    mask: 0xF000,
    debug: false,
    execute: |_opcode, _mem, registers, _screen| {
      registers.pc += 2;
      Ok(())
    }
  }
}

fn cls() -> Instruction  {
  Instruction {
    name: "00E0 CLS".to_owned(),
    id: 0x00E0,
    mask: 0x00F0,
    debug: false,
    execute: |_opcode, _mem, registers, screen| {
      screen.clear();
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ret() -> Instruction  {
  Instruction {
    name: "00EE RET".to_owned(),
    id: 0x00EE,
    mask: 0x00FF,
    debug: false,
    execute: |_opcode, _mem, registers, _screen| {
      registers.pc = registers.pop()?;
      Ok(())
    }
  }
}

fn jp_addr() -> Instruction  {
  Instruction {
    name: "1nnn JP addr".to_owned(),
    id: 0x1000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let nnn = opcode & 0x0FFF;
      registers.pc = nnn;
      Ok(())
    }
  }
}

fn call_addr() -> Instruction  {
  Instruction {
    name: "2nnn CALL addr".to_owned(),
    id: 0x2000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let nnn = opcode & 0x0FFF;
      registers.push(registers.pc);
      registers.pc = nnn;
      Ok(())
    }
  }
}

fn se_vx_byte() -> Instruction  {
  Instruction {
    name: "3xnn SE Vx, byte".to_owned(),
    id: 0x3000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
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

fn sne_vx_byte() -> Instruction  {
  Instruction {
    name: "4xnn SNE Vx, byte".to_owned(),
    id: 0x4000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
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

fn se_vx_vy() -> Instruction  {
  Instruction {
    name: "5xy0 SE Vx, Vy".to_owned(),
    id: 0x5000,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
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

fn ld_vx_byte() -> Instruction  {
  Instruction {
    name: "6xnn LD Vx, byte".to_owned(),
    id: 0x6000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let nn = (opcode & 0x00FF) as u8;
      registers.set_v(x, nn)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn add_vx_byte() -> Instruction  {
  Instruction {
    name: "7xnn ADD Vx, byte".to_owned(),
    id: 0x7000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let nn = (opcode & 0x00FF) as u16;
      let vx = registers.get_v(x)? as u16;
      registers.set_v(x, (vx + nn) as u8)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_vx_vy() -> Instruction  {
  Instruction {
    name: "8xy0 LD Vx, Vy".to_owned(),
    id: 0x8000,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      registers.set_v(x, registers.get_v(y)?)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn or_vx_vy() -> Instruction  {
  Instruction {
    name: "8xy1 OR Vx, Vy".to_owned(),
    id: 0x8001,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
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

fn and_vx_vy() -> Instruction  {
  Instruction {
    name: "8xy2 AND Vx, Vy".to_owned(),
    id: 0x8002,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
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

fn xor_vx_vy() -> Instruction {
  Instruction {
    name: "8xy3 XOR Vx, Vy".to_owned(),
    id: 0x8003,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
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

fn add_vx_vy() -> Instruction {
  Instruction {
    name: "8xy4 ADD Vx, Vy".to_owned(),
    id: 0x8004,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
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
  }
}

fn sub_vx_vy() -> Instruction {
  Instruction {
    name: "8xy5 SUB Vx, Vy".to_owned(),
    id: 0x8005,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      let vx = registers.get_v(x)?;
      let vy = registers.get_v(y)?;
      registers.set_v(x, vx - vy)?;
      registers.set_vf(if vx > vy { 1 } else { 0 });
      registers.pc += 2;
      Ok(())
    }
  }
}

fn shr_vx() -> Instruction {
  Instruction {
    name: "8x06 SHR Vx".to_owned(),
    id: 0x8006,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)?;
      registers.set_v(x, vx >> 1)?;
      registers.set_vf(if vx & 1 == 1 { 1 } else { 0 });
      registers.pc += 2;
      Ok(())
    }
  }
}

fn subn_vx_vy() -> Instruction {
  Instruction {
    name: "8xy7 SUBN Vx, Vy".to_owned(),
    id: 0x8007,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      let vx = registers.get_v(x)?;
      let vy = registers.get_v(y)?;
      registers.set_v(x, vy - vx)?;
      registers.set_vf(if vy > vx { 1 } else { 0 });
      registers.pc += 2;
      Ok(())
    }
  }
}

fn shl_vx() -> Instruction {
  Instruction {
    name: "8x0E SHL Vx".to_owned(),
    id: 0x800E,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)?;
      registers.set_v(x, vx << 1)?;
      registers.set_vf(if vx & 0x80 == 1 { 1 } else { 0 });
      registers.pc += 2;
      Ok(())
    }
  }
}

fn sne_vx_vy() -> Instruction {
  Instruction {
    name: "9xy0 SNE Vx, Vy".to_owned(),
    id: 0x9000,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
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
  }
}

fn ld_i_addr() -> Instruction {
  Instruction {
    name: "Annn LD I, addr".to_owned(),
    id: 0xA000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let nnn = opcode & 0x0FFF;
      registers.i = nnn;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn jp_v0_addr() -> Instruction {
  Instruction {
    name: "Bnnn JP V0, addr".to_owned(),
    id: 0xB000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let nnn = opcode & 0x0FFF;
      let v0 = registers.get_v(0)? as u16;
      registers.pc = nnn + v0;
      Ok(())
    }
  }
}

fn rnd_vx_vyte() -> Instruction {
  Instruction {
    name: "Cxnn RND Vx, byte".to_owned(),
    id: 0xC000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let nn = opcode & 0x00FF;
      let r = registers.rng.gen_range(0..256);
      registers.set_v(x, (r & nn) as u8)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn drw_vx_vy_nibble() -> Instruction {
  Instruction {
    name: "Dxyn DRW Vx, Vy, nibble".to_owned(),
    id: 0xD000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, mem, registers, screen| {
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
              // Flip pixel if sprite bit is set
              if screen.pixel(px, py) {
                screen.set_pixel(px, py, false);
                // Set the VF register if a pixel is unset
                registers.v[15] = 1;
              } else {
                screen.set_pixel(px, py, true);
              }
            }
          }
        }
      }

      registers.pc += 2;
      Ok(())
    }
  }
}

fn skp_vx() -> Instruction {
  Instruction {
    name: "Ex9E SKP Vx".to_owned(),
    id: 0xE09E,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)? as usize;
      if registers.keydown(vx)? {
        registers.pc += 4;
      } else {
        registers.pc += 2;
      }
      Ok(())
    }
  }
}

fn sknp_vx() -> Instruction {
  Instruction {
    name: "ExA1 SKNP Vx".to_owned(),
    id: 0xE0A1,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)? as usize;
      if registers.keydown(vx)? {
        registers.pc += 2;
      } else {
        registers.pc += 4;
      }
      Ok(())
    }
  }
}

fn ld_vx_dt() -> Instruction {
  Instruction {
    name: "Fx07 LD Vx, DT".to_owned(),
    id: 0xF007,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      registers.set_v(x, registers.delay_timer)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_vx_k() -> Instruction {
  Instruction {
    name: "Fx0A LD Vx, K".to_owned(),
    id: 0xF00A,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      if let Some(key) = registers.first_keydown() {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        registers.set_v(x, key as u8)?;
        registers.pc += 2;
      }
      Ok(())
    }
  }
}

fn ld_dt_vx() -> Instruction {
  Instruction {
    name: "Fx15 LD DT, Vx".to_owned(),
    id: 0xF015,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      registers.delay_timer = registers.get_v(x)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_st_vx() -> Instruction {
  Instruction {
    name: "Fx18 LD ST, Vx".to_owned(),
    id: 0xF018,
    mask: 0xF0FF,
    debug: true,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      registers.sound_timer = registers.get_v(x)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn add_i_vx() -> Instruction {
  Instruction {
    name: "Fx1E ADD I, Vx".to_owned(),
    id: 0xF01E,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      registers.i += registers.get_v(x)? as u16;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_f_vx() -> Instruction {
  Instruction {
    name: "Fx29 LD F, Vx".to_owned(),
    id: 0xF029,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)? as u16;
      registers.i = FONT_OFFSET as u16 + (vx * 5);
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_b_vx() -> Instruction {
  Instruction {
    name: "Fx33 LD B, Vx".to_owned(),
    id: 0xF033,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, mem, registers, _screen| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)?;
      mem.write_byte(registers.i as usize, vx / 100)?;
      mem.write_byte(registers.i as usize, (vx / 10) % 10)?;
      mem.write_byte(registers.i as usize, (vx % 100) % 10)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_arr_i_vx() -> Instruction {
  Instruction {
    name: "Fx55 LD [I], Vx".to_owned(),
    id: 0xF055,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, mem, registers, _screen| {
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
  }
}

fn ld_arr_vx_i() -> Instruction {
  Instruction {
    name: "Fx65 LD Vx, [I]".to_owned(),
    id: 0xF065,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, mem, registers, _screen| {
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
  }
}

pub fn get_instructions() -> Vec<Instruction> {
  vec![
    sys_addr(),
    cls(),
    ret(),
    jp_addr(),
    call_addr(),
    se_vx_byte(),
    sne_vx_byte(),
    se_vx_vy(),
    ld_vx_byte(),
    add_vx_byte(),
    ld_vx_vy(),
    or_vx_vy(),
    and_vx_vy(),
    xor_vx_vy(),
    add_vx_vy(),
    sub_vx_vy(),
    shr_vx(),
    subn_vx_vy(),
    shl_vx(),
    sne_vx_vy(),
    ld_i_addr(),
    jp_v0_addr(),
    rnd_vx_vyte(),
    drw_vx_vy_nibble(),
    skp_vx(),
    sknp_vx(),
    ld_vx_dt(),
    ld_vx_k(),
    ld_dt_vx(),
    ld_st_vx(),
    add_i_vx(),
    ld_f_vx(),
    ld_b_vx(),
    ld_arr_i_vx(),
    ld_arr_vx_i(),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::screen::MockChip8Screen;

  fn deps() -> (Memory, Registers, MockChip8Screen) {
    (Memory::default(), Registers::default(), MockChip8Screen::new())
  }

  #[test]
  fn test_cls() {
    let (mut mem, mut registers, mut screen) = deps();
    let i_cls = cls();
    screen.expect_clear()
      .times(1)
      .return_const(());
    assert!((i_cls.execute)(0x00E0, &mut mem, &mut registers, &mut screen).is_ok());
  }

  #[test]
  fn test_ret() {
    let (mut mem, mut registers, mut screen) = deps();
    let i_ret = ret();
    registers.push(0xF);
    assert!((i_ret.execute)(0x00EE, &mut mem, &mut registers, &mut screen).is_ok());
    assert_eq!(registers.pc, 0xF);
    // TODO: Assert on error type
    assert!((i_ret.execute)(0x00EE, &mut mem, &mut registers, &mut screen).is_err());
  }
}
