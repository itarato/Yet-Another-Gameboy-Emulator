pub struct Util;

impl Util {
  pub fn setbit(seq: u8, bitnum: u32, bitval: u8) -> u8 {
    (seq as u16)
      .wrapping_shr(bitnum + 1)
      .wrapping_shl(bitnum + 1) as u8
      | (seq & ((1 << bitnum) - 1))
      | (bitval << bitnum)
  }

  pub fn has_half_carry(w: u8, acc: u8) -> bool {
    (w & 0xf) + (acc & 0xf) > 0xf
  }

  pub fn has_half_borrow(w: u8, acc: u8) -> bool {
    (w & 0xf) < (acc & 0xf)
  }

  pub fn has_carry(w: u8, acc: u8) -> bool {
    (w as u16) + (acc as u16) > 0xff
  }

  pub fn has_borrow(w: u8, acc: u8) -> bool {
    w < acc
  }

  pub fn swap(w: u8) -> u8 {
    (w << 4) | (w >> 4)
  }

  pub fn dword_signed_add(dword: u16, sw: i8) -> u16 {
    if sw >= 0 {
      dword.wrapping_add(sw as u16)
    } else {
      dword.wrapping_sub(sw.abs() as u16)
    }
  }

  pub fn did_tick_happened(cycles_pre: u64, cycles_current: u64, hz: u64) -> bool {
    assert!(cycles_pre < cycles_current);

    let machine_cycles = hz / 4;
    let div_pre = cycles_pre / machine_cycles;
    let div_post = cycles_current / machine_cycles;

    div_pre != div_post
  }
}

pub struct History<T: Default + Copy> {
  history: Vec<T>,
  ptr: usize,
  capacity: usize,
}

impl<T: Default + Copy> History<T> {
  pub fn with_capacity(capacity: usize) -> History<T> {
    let mut history = Vec::with_capacity(capacity);
    for _ in 0..capacity {
      history.push(T::default());
    }

    History {
      history,
      ptr: 0,
      capacity,
    }
  }

  pub fn push(&mut self, e: T) {
    self.history[self.ptr] = e;
    self.ptr = (self.ptr + 1) % self.capacity;
  }

  pub fn get(&self) -> Vec<T> {
    let mut out: Vec<T> = Vec::new();
    for i in 0..self.capacity {
      out.push(self.history[(self.ptr + i) % self.capacity]);
    }
    out.reverse();
    out
  }
}

pub trait BoolNumerics {
  fn as_bit(&self) -> u8;
}

impl BoolNumerics for bool {
  fn as_bit(&self) -> u8 {
    match self {
      true => 0b1,
      false => 0b0,
    }
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_setbit() {
    assert_eq!(0b1001_1000, Util::setbit(0b1001_1001, 0, 0));
    assert_eq!(0b1001_1001, Util::setbit(0b1001_1001, 0, 1));
    assert_eq!(0b1001_1001, Util::setbit(0b1001_1001, 1, 0));
    assert_eq!(0b1001_1011, Util::setbit(0b1001_1001, 1, 1));
    assert_eq!(0b0001_1001, Util::setbit(0b1001_1001, 7, 0));
    assert_eq!(0b1001_1001, Util::setbit(0b1001_1001, 7, 1));
    assert_eq!(0b0000_0000, Util::setbit(0b0000_0001, 0, 0));
  }

  #[test]
  fn test_has_half_carry() {
    assert!(Util::has_half_carry(0b1001_1000, 0b1000));
    assert!(Util::has_half_carry(0b1001_1001, 0b1110));
    assert!(Util::has_half_carry(0b1000_1111, 0b1));
    assert!(Util::has_half_carry(0b1111, 0b1));

    assert!(!Util::has_half_carry(0b1001_1000, 0b0111));
    assert!(!Util::has_half_carry(0b1001_1001, 0b0001_0000));
  }

  #[test]
  fn test_has_half_borrow() {
    assert!(Util::has_half_borrow(0b0001_0000, 0b1));
    assert!(Util::has_half_borrow(0b0001_0000, 0b10));
    assert!(Util::has_half_borrow(0b0001_0001, 0b10));
    assert!(Util::has_half_borrow(0x3e, 0xf));

    assert!(!Util::has_half_borrow(0b0001_0000, 0b0));
    assert!(!Util::has_half_borrow(0b0001_0001, 0b1));
    assert!(!Util::has_half_borrow(0x3e, 0x40));
  }

  #[test]
  fn test_has_carry() {
    assert!(Util::has_carry(0b1111_1111, 0b1));
    assert!(Util::has_carry(0b1111_1111, 0b1111_1111));

    assert!(!Util::has_carry(0b1101_1111, 0b1));
  }

  #[test]
  fn test_has_borrow() {
    assert!(Util::has_borrow(0b0001_1000, 0b0001_1001));
    assert!(Util::has_borrow(0x3e, 0x40));

    assert!(!Util::has_borrow(0x3e, 0xf));
  }
}
