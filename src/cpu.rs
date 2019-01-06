macro_rules! set_dword_register {
    ($reg_hi:ident, $reg_lo:ident, $fname:ident) => (
      pub fn $fname(&mut self, dw: u16) {
        self.$reg_hi = (dw >> 0x8) as u8;
        self.$reg_lo = (dw & 0xff) as u8;
      }
    )
}

#[derive(Default)]
pub struct Cpu {
  pub reg_a: u8,
  pub reg_f: u8,
  pub reg_b: u8,
  pub reg_c: u8,
  pub reg_d: u8,
  pub reg_e: u8,
  pub reg_h: u8,
  pub reg_l: u8,
  pub sp: u16,
  pc: u16,
}

impl Cpu {
  pub fn new() -> Cpu {
    Default::default()
  }

  pub fn reset(&mut self) {}

  set_dword_register! { reg_a, reg_f, set_af }
  set_dword_register! { reg_b, reg_c, set_bc }
  set_dword_register! { reg_d, reg_e, set_de }
  set_dword_register! { reg_h, reg_l, set_hl }

  pub fn pc_inc(&mut self) -> u16 {
    let pc = self.pc;
    self.pc += 1;
    pc
  }
}
