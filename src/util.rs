pub struct Util;

impl Util {
  pub fn setbit(seq: u8, bitnum: u32, bitval: u8) -> u8 {
    (seq as u16)
      .wrapping_shr(bitnum + 1)
      .wrapping_shl(bitnum + 1) as u8
      | (seq & ((1 << bitnum) - 1))
      | (bitval << bitnum)
  }

  pub fn in_range<T: PartialOrd>(min: T, max: T, val: T) -> bool {
    min <= val && val < max
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

#[test]
fn test_bitval() {
  assert_eq!(0b1001_1000, Util::setbit(0b1001_1001, 0, 0));
  assert_eq!(0b1001_1001, Util::setbit(0b1001_1001, 0, 1));
  assert_eq!(0b1001_1001, Util::setbit(0b1001_1001, 1, 0));
  assert_eq!(0b1001_1011, Util::setbit(0b1001_1001, 1, 1));
  assert_eq!(0b0001_1001, Util::setbit(0b1001_1001, 7, 0));
  assert_eq!(0b1001_1001, Util::setbit(0b1001_1001, 7, 1));
}
