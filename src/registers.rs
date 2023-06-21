pub struct Registers {
  pub pc: u16,
  pub i: u16,
  pub stack: Vec<u16>,
  pub v: [u8; 16],
  pub delay_timer: u8,
  pub sound_timer: u8,
}

impl Default for Registers {
  fn default() -> Self {
      Self {
        pc: 512,
        i: 0,
        stack: Vec::new(),
        v: [0; 16],
        delay_timer: 0,
        sound_timer: 0,
      }
  }
}
