macro_rules! dword {
  ($hi:expr, $lo:expr) => {{
    (($hi as u16) << 0x8) | $lo as u16
  }};
}

macro_rules! load_dword_to_reg {
  ($cpu_dword_setter:ident, $sel:ident) => {{
    let dw = $sel.read_opcode_dword();
    $sel.cpu.$cpu_dword_setter(dw);
  }};
}

macro_rules! load_word_to_reg {
  ($reg:ident, $sel:ident) => {{
    $sel.cpu.$reg = $sel.read_opcode_word();
  }};
}

macro_rules! load_word_to_reg_from_reg {
  ($reg_to:ident, $reg_from:ident, $sel:ident) => {{
    $sel.cpu.$reg_to = $sel.cpu.$reg_from;
  }};
}

macro_rules! load_word_to_reg_from_reg_addr {
  ($reg_to:ident, $reg_from_hi:ident, $reg_from_lo:ident, $sel:ident) => {{
    let addr = dword!($sel.cpu.$reg_from_hi, $sel.cpu.$reg_from_lo);
    $sel.cpu.$reg_to = $sel.read_word(addr);
  }};
}

macro_rules! load_word_to_reg_addr_from_reg {
  ($addr_reg_hi:ident, $addr_reg_lo:ident, $reg_from:ident, $sel:ident) => {{
    let addr = dword!($sel.cpu.$addr_reg_hi, $sel.cpu.$addr_reg_lo);
    $sel.write_word(addr, $sel.cpu.$reg_from);
  }};
}

macro_rules! xor_reg {
  ($reg:ident, $sel:ident) => {{
    $sel.cpu.reg_a = $sel.cpu.reg_a ^ $sel.cpu.$reg;
  }};
}

macro_rules! hi {
  ($dw:expr) => {{
    ($dw >> 0x8) as u8
  }};
}

macro_rules! lo {
  ($dw:expr) => {{
    ($dw | 0xff) as u8
  }};
}

macro_rules! set_dword_register {
    ($fname:ident, $reg_hi:ident, $reg_lo:ident) => (
      pub fn $fname(&mut self, dw: u16) {
        self.$reg_hi = (dw >> 0x8) as u8;
        self.$reg_lo = (dw & 0xff) as u8;
      }
    )
}

macro_rules! dec_dword_reg {
  ($fname:ident, $reg_lo:ident, $reg_hi:ident) => (
    pub fn $fname(&mut self) {
      let mut dw = dword!(self.$reg_hi, self.$reg_lo);
      dw = dw.wrapping_sub(1);
      self.$reg_hi = hi!(dw);
      self.$reg_lo = lo!(dw);
    }
  )
}
