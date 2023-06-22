use thiserror::Error;

// TODO: Implement PartialEq
#[derive(Error, Debug)]
pub enum Chip8Error {
  #[error("Engine Error: {0}")]
  EngineError(#[from] ggez::GameError),
  #[error("IO Error: {0}")]
  IOError(#[from] std::io::Error),
  #[error("Invalid address {:#06x}", .0)]
  InvalidAddressError(usize),
  #[error("Invalid instruction {:#06x} at address {:#06x}", .1, .0)]
  InvalidInstructionError(usize, u16),
  #[error("Invalid register address {0}")]
  InvalidRegister(usize),
  #[error("Invalid key {0}")]
  InvalidKey(usize),
  #[error("Attempted to pop from empty stack")]
  EmptyStack,
}

impl Into<ggez::GameError> for Chip8Error {
  fn into(self) -> ggez::GameError {
    match self {
      Chip8Error::EngineError(err) => err,
      _ => ggez::GameError::CustomError(self.to_string())
    }
  }
}

pub type Chip8Result<T = ()> = Result<T, Chip8Error>;
