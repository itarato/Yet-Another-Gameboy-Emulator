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
  pub pc: u16,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      sp: 0xfffe,
      ..Default::default()
    }
  }

  pub fn reset(&mut self) {}

  set_dword_register! { set_af, reg_a, reg_f }
  set_dword_register! { set_bc, reg_b, reg_c }
  set_dword_register! { set_de, reg_d, reg_e }
  set_dword_register! { set_hl, reg_h, reg_l }

  dec_dword_reg! { dec_af, reg_a, reg_f }
  dec_dword_reg! { dec_bc, reg_b, reg_c }
  dec_dword_reg! { dec_de, reg_d, reg_e }
  dec_dword_reg! { dec_hl, reg_h, reg_l }

  pub fn pc_inc(&mut self) -> u16 {
    let pc = self.pc;
    self.pc += 1;
    pc
  }

  cpu_flag_fn! { flag_zero, set_flag_zero, reset_flag_zero, 7 }
  cpu_flag_fn! { flag_add_sub, set_flag_add_sub, reset_flag_add_sub, 6 }
  cpu_flag_fn! { flag_half_carry, set_flag_half_carry, reset_flag_half_carry, 5 }
  cpu_flag_fn! { flag_carry, set_flag_carry, reset_flag_carry, 4 }
}
