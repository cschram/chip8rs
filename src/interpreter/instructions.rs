use super::{
  error::InterpreterError,
  frame_buffer::FrameBuffer,
  memory::{Memory, FONT_OFFSET},
  registers::Registers,
};
use rand::prelude::*;

pub struct Instruction {
  pub name: String,
  pub id: u16,
  pub mask: u16,
  pub debug: bool,
  pub execute: fn(
    u16,
    &mut Memory,
    &mut Registers,
    &mut FrameBuffer,
    &mut ThreadRng,
  ) -> Result<(), InterpreterError>,
}

pub struct InstructionSet(Vec<Instruction>);

impl Default for InstructionSet {
  fn default() -> Self {
    Self(vec![
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
    ])
  }
}

impl InstructionSet {
  pub fn execute(
    &self,
    mem: &mut Memory,
    registers: &mut Registers,
    frame_buffer: &mut FrameBuffer,
    rng: &mut ThreadRng,
  ) -> Result<(), InterpreterError> {
    let pc = registers.pc as usize;

    let opbytes = mem.read(pc, 2)?;
    let opcode = ((opbytes[0] as u16) << 8) | ((opbytes[1] as u16) << 0);

    match self.disassemble(opcode) {
      Some(instr) => {
        (instr.execute)(opcode, mem, registers, frame_buffer, rng)
      },
      None => {
        Err(InterpreterError::InvalidInstructionError(pc, opcode))
      }
    }
  }

  fn disassemble(&self, opcode: u16) -> Option<&Instruction> {
    self.0.iter().find(|instr| {
      (opcode & instr.mask) == instr.id
    })
  }
}

fn sys_addr() -> Instruction {
  Instruction {
    name: "SYS addr".to_owned(),
    id: 0x0000,
    mask: 0xFFFF,
    debug: false,
    execute: |_opcode, _mem, registers, _frame_buffer, _rng| {
      registers.pc += 2;
      Ok(())
    }
  }
}

fn cls() -> Instruction  {
  Instruction {
    name: "CLS".to_owned(),
    id: 0x00E0,
    mask: 0xFFFF,
    debug: false,
    execute: |_opcode, _mem, registers, frame_buffer, _rng| {
      frame_buffer.clear();
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ret() -> Instruction  {
  Instruction {
    name: "RET".to_owned(),
    id: 0x00EE,
    mask: 0xFFFF,
    debug: false,
    execute: |_opcode, _mem, registers, _frame_buffer, _rng| {
      registers.pc = registers.pop()? + 2;
      Ok(())
    }
  }
}

fn jp_addr() -> Instruction  {
  Instruction {
    name: "JP addr".to_owned(),
    id: 0x1000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let nnn = opcode & 0x0FFF;
      registers.pc = nnn;
      Ok(())
    }
  }
}

fn call_addr() -> Instruction  {
  Instruction {
    name: "CALL addr".to_owned(),
    id: 0x2000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let nnn = opcode & 0x0FFF;
      registers.push(registers.pc)?;
      registers.pc = nnn;
      Ok(())
    }
  }
}

fn se_vx_byte() -> Instruction  {
  Instruction {
    name: "SE Vx, byte".to_owned(),
    id: 0x3000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "SNE Vx, byte".to_owned(),
    id: 0x4000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "SE Vx, Vy".to_owned(),
    id: 0x5000,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let vx = registers.get_v(((opcode & 0x0F00) >> 8) as usize)?;
      let vy = registers.get_v(((opcode & 0x00F0) >> 4) as usize)?;
      if vx == vy {
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
    name: "LD Vx, byte".to_owned(),
    id: 0x6000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "ADD Vx, byte".to_owned(),
    id: 0x7000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "LD Vx, Vy".to_owned(),
    id: 0x8000,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "OR Vx, Vy".to_owned(),
    id: 0x8001,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "AND Vx, Vy".to_owned(),
    id: 0x8002,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "XOR Vx, Vy".to_owned(),
    id: 0x8003,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "ADD Vx, Vy".to_owned(),
    id: 0x8004,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "SUB Vx, Vy".to_owned(),
    id: 0x8005,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      let vx = registers.get_v(x)?;
      let vy = registers.get_v(y)?;
      if vx > vy {
        registers.set_v(x, vx - vy)?;
        registers.set_vf(1);
      } else {
        registers.set_v(x, 255 - (vy - vx - 1))?;
        registers.set_vf(0);
      }
      registers.pc += 2;
      Ok(())
    }
  }
}

fn shr_vx() -> Instruction {
  Instruction {
    name: "SHR Vx".to_owned(),
    id: 0x8006,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)?;
      registers.set_v(x, vx >> 1)?;
      registers.set_vf(vx & 1);
      registers.pc += 2;
      Ok(())
    }
  }
}

fn subn_vx_vy() -> Instruction {
  Instruction {
    name: "SUBN Vx, Vy".to_owned(),
    id: 0x8007,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let y = ((opcode & 0x00F0) >> 4) as usize;
      let vx = registers.get_v(x)?;
      let vy = registers.get_v(y)?;
      if vy > vx {
        registers.set_v(x, vy - vx)?;
        registers.set_vf(1);
      } else {
        registers.set_v(x, 255 - (vx - vy - 1))?;
        registers.set_vf(0);
      }
      registers.pc += 2;
      Ok(())
    }
  }
}

fn shl_vx() -> Instruction {
  Instruction {
    name: "SHL Vx".to_owned(),
    id: 0x800E,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)?;
      registers.set_v(x, vx << 1)?;
      registers.set_vf(vx >> 7);
      registers.pc += 2;
      Ok(())
    }
  }
}

fn sne_vx_vy() -> Instruction {
  Instruction {
    name: "SNE Vx, Vy".to_owned(),
    id: 0x9000,
    mask: 0xF00F,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "LD I, addr".to_owned(),
    id: 0xA000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let nnn = opcode & 0x0FFF;
      registers.i = nnn;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn jp_v0_addr() -> Instruction {
  Instruction {
    name: "JP V0, addr".to_owned(),
    id: 0xB000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let nnn = opcode & 0x0FFF;
      let v0 = registers.get_v(0)? as u16;
      registers.pc = nnn + v0;
      Ok(())
    }
  }
}

fn rnd_vx_vyte() -> Instruction {
  Instruction {
    name: "RND Vx, byte".to_owned(),
    id: 0xC000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let nn = opcode & 0x00FF;
      let r = rng.gen_range(0..256);
      registers.set_v(x, (r & nn) as u8)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn drw_vx_vy_nibble() -> Instruction {
  Instruction {
    name: "DRW Vx, Vy, nibble".to_owned(),
    id: 0xD000,
    mask: 0xF000,
    debug: false,
    execute: |opcode, mem, registers, frame_buffer, _rng| {
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
              if frame_buffer.get_xy(px, py)? {
                frame_buffer.set_xy(px, py, false)?;
                // Set the VF register if a pixel is unset
                registers.v[15] = 1;
              } else {
                frame_buffer.set_xy(px, py, true)?;
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
    name: "SKP Vx".to_owned(),
    id: 0xE09E,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "SKNP Vx".to_owned(),
    id: 0xE0A1,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "LD Vx, DT".to_owned(),
    id: 0xF007,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      registers.set_v(x, registers.delay_timer as u8)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_vx_k() -> Instruction {
  Instruction {
    name: "LD Vx, K".to_owned(),
    id: 0xF00A,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
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
    name: "LD DT, Vx".to_owned(),
    id: 0xF015,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      registers.delay_timer = registers.get_v(x)? as f32;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_st_vx() -> Instruction {
  Instruction {
    name: "LD ST, Vx".to_owned(),
    id: 0xF018,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      registers.sound_timer = registers.get_v(x)? as f32;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn add_i_vx() -> Instruction {
  Instruction {
    name: "ADD I, Vx".to_owned(),
    id: 0xF01E,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      registers.i += registers.get_v(x)? as u16;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_f_vx() -> Instruction {
  Instruction {
    name: "LD F, Vx".to_owned(),
    id: 0xF029,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, _mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)? as u16;
      if vx > 0xF {
        Err(InterpreterError::InvalidKey(vx as usize))
      } else {
        registers.i = FONT_OFFSET as u16 + (vx * 5);
        registers.pc += 2;
        Ok(())
      }
    }
  }
}

fn ld_b_vx() -> Instruction {
  Instruction {
    name: "LD B, Vx".to_owned(),
    id: 0xF033,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, mem, registers, _frame_buffer, _rng| {
      let x = ((opcode & 0x0F00) >> 8) as usize;
      let vx = registers.get_v(x)?;
      mem.write_byte(registers.i as usize, vx / 100)?;
      mem.write_byte(registers.i as usize + 1, (vx / 10) % 10)?;
      mem.write_byte(registers.i as usize + 2, (vx % 100) % 10)?;
      registers.pc += 2;
      Ok(())
    }
  }
}

fn ld_arr_i_vx() -> Instruction {
  Instruction {
    name: "LD [I], Vx".to_owned(),
    id: 0xF055,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, mem, registers, _frame_buffer, _rng| {
      let x = (opcode & 0x0F00) >> 8;
      for i in 0..(x + 1) {
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
    name: "LD Vx, [I]".to_owned(),
    id: 0xF065,
    mask: 0xF0FF,
    debug: false,
    execute: |opcode, mem, registers, _frame_buffer, _rng| {
      let x = (opcode & 0x0F00) >> 8;
      for i in 0..(x + 1) {
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

#[cfg(test)]
mod tests {
  use super::*;

  fn deps() -> (Memory, Registers, FrameBuffer, ThreadRng) {
    (Memory::default(), Registers::default(), FrameBuffer::default(), thread_rng())
  }

  fn exec(
    instr: Instruction,
    opcode: u16,
    mem: &mut Memory,
    registers: &mut Registers,
    frame_buffer: &mut FrameBuffer,
    rng: &mut ThreadRng,
  ) {
    assert!(
      (instr.execute)(
        opcode,
        mem,
        registers,
        frame_buffer,
        rng,
      ).is_ok()
    );
  }

  fn exec_err(
    instr: Instruction,
    opcode: u16,
    mem: &mut Memory,
    registers: &mut Registers,
    frame_buffer: &mut FrameBuffer,
    rng: &mut ThreadRng,
  ) {
    assert!(
      (instr.execute)(
        opcode,
        mem,
        registers,
        frame_buffer,
        rng,
      ).is_err()
    );
  }

  #[test]
  fn test_disassemble() {
    let instructions = InstructionSet::default();

    let cls = instructions.disassemble(0x00E0);
    assert!(cls.is_some());
    assert_eq!(cls.unwrap().name, "CLS");

    let ret = instructions.disassemble(0x00EE);
    assert!(ret.is_some());
    assert_eq!(ret.unwrap().name, "RET");

    let jp_addr = instructions.disassemble(0x1AAA);
    assert!(jp_addr.is_some());
    assert_eq!(jp_addr.unwrap().name, "JP addr");

    let call_addr = instructions.disassemble(0x2AAA);
    assert!(call_addr.is_some());
    assert_eq!(call_addr.unwrap().name, "CALL addr");

    let se_vx_byte = instructions.disassemble(0x3AAA);
    assert!(se_vx_byte.is_some());
    assert_eq!(se_vx_byte.unwrap().name, "SE Vx, byte");

    let sne_vx_byte = instructions.disassemble(0x4AAA);
    assert!(sne_vx_byte.is_some());
    assert_eq!(sne_vx_byte.unwrap().name, "SNE Vx, byte");

    let se_vx_vy = instructions.disassemble(0x5AB0);
    assert!(se_vx_vy.is_some());
    assert_eq!(se_vx_vy.unwrap().name, "SE Vx, Vy");

    let ld_vx_byte = instructions.disassemble(0x6AAA);
    assert!(ld_vx_byte.is_some());
    assert_eq!(ld_vx_byte.unwrap().name, "LD Vx, byte");

    let add_vx_byte = instructions.disassemble(0x7AAA);
    assert!(add_vx_byte.is_some());
    assert_eq!(add_vx_byte.unwrap().name, "ADD Vx, byte");

    let ld_vx_vy = instructions.disassemble(0x8AB0);
    assert!(ld_vx_vy.is_some());
    assert_eq!(ld_vx_vy.unwrap().name, "LD Vx, Vy");

    let or_vx_vy = instructions.disassemble(0x8AB1);
    assert!(or_vx_vy.is_some());
    assert_eq!(or_vx_vy.unwrap().name, "OR Vx, Vy");

    let and_vx_vy = instructions.disassemble(0x8AB2);
    assert!(and_vx_vy.is_some());
    assert_eq!(and_vx_vy.unwrap().name, "AND Vx, Vy");

    let xor_vx_vy = instructions.disassemble(0x8AB3);
    assert!(xor_vx_vy.is_some());
    assert_eq!(xor_vx_vy.unwrap().name, "XOR Vx, Vy");

    let add_vx_vy = instructions.disassemble(0x8AB4);
    assert!(add_vx_vy.is_some());
    assert_eq!(add_vx_vy.unwrap().name, "ADD Vx, Vy");

    let sub_vx_vy = instructions.disassemble(0x8AB5);
    assert!(sub_vx_vy.is_some());
    assert_eq!(sub_vx_vy.unwrap().name, "SUB Vx, Vy");

    let shr_vx = instructions.disassemble(0x8AB6);
    assert!(shr_vx.is_some());
    assert_eq!(shr_vx.unwrap().name, "SHR Vx");

    let subn_vx_vy = instructions.disassemble(0x8AB7);
    assert!(subn_vx_vy.is_some());
    assert_eq!(subn_vx_vy.unwrap().name, "SUBN Vx, Vy");

    let shl_vx = instructions.disassemble(0x8ABE);
    assert!(shl_vx.is_some());
    assert_eq!(shl_vx.unwrap().name, "SHL Vx");

    let sne_vx_vy = instructions.disassemble(0x9AB0);
    assert!(sne_vx_vy.is_some());
    assert_eq!(sne_vx_vy.unwrap().name, "SNE Vx, Vy");

    let ld_i_addr = instructions.disassemble(0xAEEE);
    assert!(ld_i_addr.is_some());
    assert_eq!(ld_i_addr.unwrap().name, "LD I, addr");

    let jp_v0_addr = instructions.disassemble(0xBEEE);
    assert!(jp_v0_addr.is_some());
    assert_eq!(jp_v0_addr.unwrap().name, "JP V0, addr");

    let rnd_vx_byte = instructions.disassemble(0xCAEE);
    assert!(rnd_vx_byte.is_some());
    assert_eq!(rnd_vx_byte.unwrap().name, "RND Vx, byte");

    let drw_vx_vy_n = instructions.disassemble(0xDABC);
    assert!(drw_vx_vy_n.is_some());
    assert_eq!(drw_vx_vy_n.unwrap().name, "DRW Vx, Vy, nibble");

    let skp_vx = instructions.disassemble(0xEA9E);
    assert!(skp_vx.is_some());
    assert_eq!(skp_vx.unwrap().name, "SKP Vx");

    let sknp_vx = instructions.disassemble(0xEAA1);
    assert!(sknp_vx.is_some());
    assert_eq!(sknp_vx.unwrap().name, "SKNP Vx");

    let ld_vx_dt = instructions.disassemble(0xFA07);
    assert!(ld_vx_dt.is_some());
    assert_eq!(ld_vx_dt.unwrap().name, "LD Vx, DT");

    let ld_vx_k = instructions.disassemble(0xFA0A);
    assert!(ld_vx_k.is_some());
    assert_eq!(ld_vx_k.unwrap().name, "LD Vx, K");

    let ld_dt_vx = instructions.disassemble(0xFA15);
    assert!(ld_dt_vx.is_some());
    assert_eq!(ld_dt_vx.unwrap().name, "LD DT, Vx");

    let ld_st_vx = instructions.disassemble(0xFA18);
    assert!(ld_st_vx.is_some());
    assert_eq!(ld_st_vx.unwrap().name, "LD ST, Vx");

    let add_i_vx = instructions.disassemble(0xFA1E);
    assert!(add_i_vx.is_some());
    assert_eq!(add_i_vx.unwrap().name, "ADD I, Vx");

    let ld_f_vx = instructions.disassemble(0xFA29);
    assert!(ld_f_vx.is_some());
    assert_eq!(ld_f_vx.unwrap().name, "LD F, Vx");

    let ld_b_vx = instructions.disassemble(0xFA33);
    assert!(ld_b_vx.is_some());
    assert_eq!(ld_b_vx.unwrap().name, "LD B, Vx");

    let ld_arr_i_vx = instructions.disassemble(0xFA55);
    assert!(ld_arr_i_vx.is_some());
    assert_eq!(ld_arr_i_vx.unwrap().name, "LD [I], Vx");

    let ld_arr_vx_i = instructions.disassemble(0xFA65);
    assert!(ld_arr_vx_i.is_some());
    assert_eq!(ld_arr_vx_i.unwrap().name, "LD Vx, [I]");
  }

  #[test]
  fn test_cls() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    exec(cls(), 0x00E0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(frame_buffer.frame(), &[0; 2048]);
  }

  #[test]
  fn test_ret() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.push(0xF0).unwrap();
    exec(ret(), 0x00EE, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xF2);
    // TODO: Assert on error type
    exec_err(ret(), 0x00EE, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
  }

  #[test]
  fn test_jp_addr() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    exec(jp_addr(), 0x10F0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0x0F0);
  }

  #[test]
  fn test_call_addr() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    let pc = registers.pc;
    exec(call_addr(), 0x20F0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.stack.len(), 1);
    assert_eq!(registers.stack[0], pc);
    assert_eq!(registers.pc, 0x0F0);
    registers.stack = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    exec_err(call_addr(), 0x20F0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
  }

  #[test]
  fn test_se_vx_byte() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.pc = 0xFF0;
    registers.set_v(0, 0xF0).unwrap();
    exec(se_vx_byte(), 0x30F0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xFF4);
    registers.pc = 0xFF0;
    registers.set_v(0, 0xFF).unwrap();
    exec(se_vx_byte(), 0x30F0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xFF2);
  }

  #[test]
  fn test_sne_vx_byte() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.pc = 0xFF0;
    registers.set_v(0, 0xF0).unwrap();
    exec(sne_vx_byte(), 0x40F0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xFF2);
    registers.pc = 0xFF0;
    registers.set_v(0, 0xFF).unwrap();
    exec(sne_vx_byte(), 0x40F0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xFF4);
  }

  #[test]
  fn test_se_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.pc = 0xFF0;
    registers.set_v(0, 0xF0).unwrap();
    registers.set_v(1, 0xF0).unwrap();
    exec(se_vx_vy(), 0x5010, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xFF4);
    registers.pc = 0xFF0;
    registers.set_v(0, 0xF0).unwrap();
    registers.set_v(1, 0xF1).unwrap();
    exec(se_vx_vy(), 0x5010, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xFF2);
  }

  #[test]
  fn test_ld_vx_byte() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    exec(ld_vx_byte(), 0x60F0, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0xF0);
  }

  #[test]
  fn test_add_vx_byte() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x02).unwrap();
    exec(add_vx_byte(), 0x7002, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0x04);
  }

  #[test]
  fn test_ld_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x01).unwrap();
    registers.set_v(1, 0x02).unwrap();
    exec(ld_vx_vy(), 0x8010, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0x02);
  }
  
  #[test]
  fn test_or_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x3).unwrap();
    registers.set_v(1, 0x4).unwrap();
    exec(or_vx_vy(), 0x8011, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0x7);
  }
  
  #[test]
  fn test_and_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x34).unwrap();
    registers.set_v(1, 0xF0).unwrap();
    exec(and_vx_vy(), 0x8012, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0x30);
  }
  
  #[test]
  fn test_xor_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x3).unwrap();
    registers.set_v(1, 0x3).unwrap();
    exec(xor_vx_vy(), 0x8013, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0);
  }
  
  #[test]
  fn test_add_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x3).unwrap();
    registers.set_v(1, 0x4).unwrap();
    exec(add_vx_vy(), 0x8014, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0x7);
    assert_eq!(registers.get_vf(), 0);
    registers.set_v(0, 0xFF).unwrap();
    registers.set_v(1, 0xFF).unwrap();
    exec(add_vx_vy(), 0x8014, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0xFE);
    assert_eq!(registers.get_vf(), 1);
  }
  
  #[test]
  fn test_sub_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x4).unwrap();
    registers.set_v(1, 0x2).unwrap();
    exec(sub_vx_vy(), 0x8015, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0x2);
    assert_eq!(registers.get_vf(), 1);
    registers.set_v(0, 0x2).unwrap();
    registers.set_v(1, 0x3).unwrap();
    exec(sub_vx_vy(), 0x8015, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0xFF);
    assert_eq!(registers.get_vf(), 0);
  }

  #[test]
  fn test_shr_vx() {  
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x3).unwrap();
    exec(shr_vx(), 0x8006, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 1);
    assert_eq!(registers.get_vf(), 1);
  }
  
  #[test]
  fn test_subn_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x3).unwrap();
    registers.set_v(1, 0x2).unwrap();
    exec(subn_vx_vy(), 0x8017, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 255);
    assert_eq!(registers.get_vf(), 0);
    registers.set_v(0, 0x2).unwrap();
    registers.set_v(1, 0x3).unwrap();
    exec(subn_vx_vy(), 0x8017, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 1);
    assert_eq!(registers.get_vf(), 1);
  }

  #[test]
  fn test_shl_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x3).unwrap();
    exec(shl_vx(), 0x800E, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 6);
    assert_eq!(registers.get_vf(), 0);
  }

  #[test]
  fn test_sne_vx_vy() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.pc = 0xF0;
    registers.set_v(0, 1).unwrap();
    registers.set_v(1, 1).unwrap();
    exec(sne_vx_vy(), 0x9010, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xF2);
    registers.pc = 0xF0;
    registers.set_v(0, 1).unwrap();
    registers.set_v(1, 2).unwrap();
    exec(sne_vx_vy(), 0x9010, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xF4);
  }

  #[test]
  fn test_ld_i_addr() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    exec(ld_i_addr(), 0xAF00, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.i, 0xF00);
  }

  #[test]
  fn test_jp_v0_addr() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 0x010).unwrap();
    exec(jp_v0_addr(), 0xB010, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 32);
  }

  // TODO: Test DRW Vx Vy N
  // #[test]
  // fn test_drw_vx_vy_n() {
  // }

  #[test]
  fn test_skp_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.pc = 0xF0;
    registers.keys[4] = true;
    registers.set_v(0, 4).unwrap();
    exec(skp_vx(), 0xE09E, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xF4);
    registers.pc = 0xF0;
    exec(skp_vx(), 0xE19E, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xF2);
  }

  #[test]
  fn test_sknp_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.pc = 0xF0;
    registers.keys[4] = true;
    registers.set_v(0, 4).unwrap();
    exec(sknp_vx(), 0xE0A1, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xF2);
    registers.pc = 0xF0;
    exec(sknp_vx(), 0xE1A1, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.pc, 0xF4);
  }

  #[test]
  fn test_ld_vx_dt() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.delay_timer = 10.0;
    exec(ld_vx_dt(), 0xF407, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(4).unwrap(), 10);
  }

  #[test]
  fn test_ld_vx_k() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.pc = 0xF0;
    exec(ld_vx_k(), 0xF00A, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 0);
    assert_eq!(registers.pc, 0xF0);
    registers.keys[4] = true;
    exec(ld_vx_k(), 0xF00A, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 4);
    assert_eq!(registers.pc, 0xF2);
  }

  #[test]
  fn test_ld_dt_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 4).unwrap();
    exec(ld_dt_vx(), 0xF015, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.delay_timer, 4.0);
  }

  #[test]
  fn test_ld_st_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 4).unwrap();
    exec(ld_st_vx(), 0xF018, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.sound_timer, 4.0);
  }

  #[test]
  fn test_add_i_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.i = 5;
    registers.set_v(2, 4).unwrap();
    exec(add_i_vx(), 0xF21E, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.i, 9);
  }

  #[test]
  fn test_ld_f_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.set_v(0, 5).unwrap();
    exec(ld_f_vx(), 0xF029, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.i, FONT_OFFSET as u16 + 25);
    registers.set_v(0, 200).unwrap();
    exec_err(ld_f_vx(), 0xF029, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
  }

  #[test]
  fn test_ld_b_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.i = 0x300;
    registers.set_v(0, 0x7B).unwrap();
    exec(ld_b_vx(), 0xF033, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(mem.read_byte(0x300).unwrap(), 1);
    assert_eq!(mem.read_byte(0x301).unwrap(), 2);
    assert_eq!(mem.read_byte(0x302).unwrap(), 3);
  }

  #[test]
  fn test_ld_arr_i_vx() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.i = 0x300;
    registers.set_v(0, 1).unwrap();
    registers.set_v(1, 2).unwrap();
    registers.set_v(2, 3).unwrap();
    exec(ld_arr_i_vx(), 0xF255, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(mem.read_byte(0x300).unwrap(), 1);
    assert_eq!(mem.read_byte(0x301).unwrap(), 2);
    assert_eq!(mem.read_byte(0x302).unwrap(), 3);
  }

  #[test]
  fn test_ld_arr_vx_i() {
    let (mut mem, mut registers, mut frame_buffer, mut rng) = deps();
    registers.i = 0x300;
    mem.write(0x300, &[1, 2, 3]).unwrap();
    exec(ld_arr_vx_i(), 0xF265, &mut mem, &mut registers, &mut frame_buffer, &mut rng);
    assert_eq!(registers.get_v(0).unwrap(), 1);
    assert_eq!(registers.get_v(1).unwrap(), 2);
    assert_eq!(registers.get_v(2).unwrap(), 3);
  }
}
