#[derive(Default, Debug)]
pub struct Sound {
  pub nr52: u8,
}

impl Sound {
  pub fn reset(&mut self) {
    self.nr52 = 0x0;
  }
}
