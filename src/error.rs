use thiserror::Error;

#[derive(Error, Debug)]
pub enum Chip8Error {
  #[error("Engine Error: {0}")]
  EngineError(#[from] ggez::GameError),
  #[error("IO Error: {0}")]
  IOError(#[from] std::io::Error),
  #[error("Error: {0}")]
  GenericError(String),
}

pub type Chip8Result<T = ()> = Result<T, Chip8Error>;
