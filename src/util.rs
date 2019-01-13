pub struct Util;

impl Util {
  pub fn in_range<T: PartialOrd>(min: T, max: T, val: T) -> bool {
    min <= val && val < max
  }

  pub fn has_half_carry(w: u8, acc: u8) -> bool {
    (w & 0xf) + (acc & 0xf) > 0xf
  }

  pub fn has_half_borrow(w: u8, acc: u8) -> bool {
    (w & 0xf) < (acc & 0xf)
  }
}

pub trait BitNumerics {
  fn hi(&self) -> u8;
  fn lo(&self) -> u8;
}

impl BitNumerics for u16 {
  fn hi(&self) -> u8 {
    (self >> 0x8) as u8
  }

  fn lo(&self) -> u8 {
    (self & 0xff) as u8
  }
}
