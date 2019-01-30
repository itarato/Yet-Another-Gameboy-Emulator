use super::util::*;

#[derive(Default, Debug)]
pub struct Sound {
  pub nr52: u8,
}

impl Sound {
  pub fn reset(&mut self) {
    self.nr52 = 0x0;
  }

  pub fn write_word(&mut self, addr: u16, w: u8) {
    if !Util::in_range(0xff10, 0xff40, addr) {
      panic!("Unsupported sound addr: 0x{:>02x}", addr);
    }

    match addr {
      0xff26 => self.nr52 = w,
      _ => unimplemented!("Unimplemented sound addr: 0x{:>02x}", addr),
    };
  }

  fn is_sound_reg_enabled(&self) -> bool {
    bitn!(self.nr52, 7) == 0x1
  }
}
