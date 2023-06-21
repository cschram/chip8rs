use crate::{
  error::*,
  display::Display,
  instructions::*,
  memory::Memory,
  registers::Registers,
};

const INSTRUCTIONS_PER_SECOND: f32 = 700.0;


pub struct Cpu {
  instructions: Vec<Instruction>,
  registers: Registers,
}

impl Default for Cpu {
  fn default() -> Self {
    Self {
      registers: Registers::default(),
      instructions: get_instructions(),
    }
  }
}

impl Cpu {
  pub fn reset(&mut self) {
    self.registers = Registers::default();
  }

  pub fn tick(&mut self, mem: &mut Memory, display: &mut Display, delta: f32) -> Chip8Result {
    let num_instructions = (INSTRUCTIONS_PER_SECOND * delta) as u32;
    for _ in 0..num_instructions {
      self.execute(mem, display)?;
    }
    Ok(())
  }

  fn execute(&mut self, mem: &mut Memory, display: &mut Display) -> Chip8Result {
    let pc = self.registers.pc as usize;
    self.registers.pc += 2;

    let opbytes = mem.read(pc, 2)?;
    let opcode = (opbytes[0] as u16) << 8 | (opbytes[1] as u16);

    let instruction = self.instructions.iter().find(|instr| {
      (opcode & instr.mask) == instr.id
    });

    match instruction {
      Some(instr) => {
        instr.execute(opcode, mem, &mut self.registers, display)
      },
      None => {
        return Err(Chip8Error::InvalidInstructionError(pc, opcode))
      }
    }
  }
}
