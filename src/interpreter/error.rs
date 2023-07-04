use thiserror::Error;

// TODO: Implement PartialEq
#[derive(Error, Debug)]
pub enum InterpreterError {
  #[error("Invalid instruction {:#06x} at address {:#06x}", .1, .0)]
  InvalidInstructionError(usize, u16),
  #[error("Invalid address {:#06x}", .0)]
  InvalidAddressError(usize),
  #[error("Invalid register address {0}")]
  InvalidRegister(usize),
  #[error("Invalid frame buffer index {0}")]
  InvalidFrameBufferIndex(u16),
  #[error("Invalid key {0}")]
  InvalidKey(usize),
  #[error("Stack overflow")]
  StackOverflow,
  #[error("Stack underflow")]
  StackUnderflow,
}

pub type InterpretterResult<T = ()> = Result<T, InterpreterError>;
