use crate::interpreter::error::InterpreterError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Chip8Error {
  #[error("Pixels Error: {0}")]
  PixelsError(#[from] pixels::Error),
  #[error("Interpreter Error: {0}")]
  InterpreterError(#[from] InterpreterError),
}
