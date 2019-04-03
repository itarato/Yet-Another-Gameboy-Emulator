use super::util::*;

#[derive(Default, Debug)]
pub struct Sound {
  pub nr10: u8,
  pub nr11: u8,
  pub nr12: u8,
  pub nr13: u8,
  pub nr14: u8,

  pub nr50: u8,
  pub nr51: u8,
  pub nr52: u8,
}

impl Sound {
  pub fn reset(&mut self) {
    self.nr52 = 0x0;
  }

  pub fn write_word(&mut self, addr: u16, w: u8) {
    match addr {
      0xff10 => self.nr10 = w,
      0xff11 => self.nr11 = w,
      0xff12 => self.nr12 = w,
      0xff13 => self.nr13 = w,
      0xff14 => self.nr14 = w,

      0xff24 => self.nr50 = w,
      0xff25 => self.nr51 = w,
      0xff26 => self.nr52 = w,
      0xff10...0xff39 => panic!("Unsupported sound addr: 0x{:>04x}", addr),
      _ => unimplemented!("Unimplemented sound addr: 0x{:>04x}", addr),
    };
  }

  pub fn read_word(&self, addr: u16) -> u8 {
    unimplemented!("Unimplemented sound chip reg read at 0x{:>04x}", addr);
  }

  fn is_sound_reg_enabled(&self) -> bool {
    bitn!(self.nr52, 7) == 0x1
  }
}
