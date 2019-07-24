use super::util::*;

#[derive(Default, Debug)]
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
    Default::default()
  }

  pub fn reset(&mut self) {
    self.sp = 0xfffe;
    self.pc = 0x0;
  }

  pub fn reg_af(&self) -> u16 {
    dword!(self.reg_a, self.reg_f)
  }
  pub fn reg_bc(&self) -> u16 {
    dword!(self.reg_b, self.reg_c)
  }
  pub fn reg_de(&self) -> u16 {
    dword!(self.reg_d, self.reg_e)
  }
  pub fn reg_hl(&self) -> u16 {
    dword!(self.reg_h, self.reg_l)
  }

  pub fn reg_sp(&self) -> u16 {
    self.sp
  }

  set_dword_register! { set_af, reg_a, reg_f }
  set_dword_register! { set_bc, reg_b, reg_c }
  set_dword_register! { set_de, reg_d, reg_e }
  set_dword_register! { set_hl, reg_h, reg_l }

  dec_dword_reg! { dec_af, reg_a, reg_f }
  dec_dword_reg! { dec_bc, reg_b, reg_c }
  dec_dword_reg! { dec_de, reg_d, reg_e }
  dec_dword_reg! { dec_hl, reg_h, reg_l }

  inc_dword_reg! { inc_af, reg_a, reg_f }
  inc_dword_reg! { inc_bc, reg_b, reg_c }
  inc_dword_reg! { inc_de, reg_d, reg_e }
  inc_dword_reg! { inc_hl, reg_h, reg_l }

  cpu_flag_fn! { flag_zero, set_flag_zero, reset_flag_zero, 7 }
  cpu_flag_fn! { flag_add_sub, set_flag_add_sub, reset_flag_add_sub, 6 }
  cpu_flag_fn! { flag_half_carry, set_flag_half_carry, reset_flag_half_carry, 5 }
  cpu_flag_fn! { flag_carry, set_flag_carry, reset_flag_carry, 4 }

  pub fn set_flag_zero_for(&mut self, w: u8) {
    self.set_flag_zero((w == 0).as_bit());
  }

  pub fn pc_inc(&mut self) -> u16 {
    let pc = self.pc;
    self.pc += 1;

    if self.pc >= 0x8000 && (self.pc < 0xc000 || self.pc > 0xfffe) {
      unimplemented!(
        "PC is out of standard rom bank space: 0x{:>04x}\n{:#x?}",
        self.pc,
        self
      );
    }

    pc
  }

  pub fn registers_debug_print(&self) {
    println!("-----------------");
    println!("[A: 0x{:>02x} F: 0x{:>02x}]", self.reg_a, self.reg_f);
    println!("[B: 0x{:>02x} C: 0x{:>02x}]", self.reg_b, self.reg_c);
    println!("[D: 0x{:>02x} E: 0x{:>02x}]", self.reg_d, self.reg_e);
    println!("[H: 0x{:>02x} L: 0x{:>02x}]", self.reg_h, self.reg_l);
    println!("[PC: 0x{:>04x}]", self.pc);
    println!("[SP: 0x{:>04x}]", self.sp);
    println!(
      "[Z:{:?} N:{:?} H:{:?} C:{:?}]",
      bitn!(self.reg_f, 7),
      bitn!(self.reg_f, 6),
      bitn!(self.reg_f, 5),
      bitn!(self.reg_f, 4)
    );
    println!("-----------------");
  }
}
