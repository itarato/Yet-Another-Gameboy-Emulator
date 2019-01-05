#[derive(Default)]
pub struct Cpu {
  reg_a: u8,
  reg_f: u8,
  reg_b: u8,
  reg_c: u8,
  reg_d: u8,
  reg_e: u8,
  reg_h: u8,
  reg_l: u8,
  sp: u16,
  pc: u16,
}

impl Cpu {
  pub fn new() -> Cpu {
    Default::default()
  }

  pub fn reset(&mut self) {}

  pub fn pc_inc(&mut self) -> u16 {
    let pc = self.pc;
    self.pc += 1;
    pc
  }
}
