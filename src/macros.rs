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
    if $sel.cpu.reg_a == 0 {
      $sel.cpu.set_flag_zero(0x1);
    }
    $sel.cpu.reset_flag_add_sub();
    $sel.cpu.reset_flag_half_carry();
    $sel.cpu.reset_flag_carry();
  }};
}

macro_rules! hi {
  ($dw:expr) => {{
    ($dw >> 0x8) as u8
  }};
}

macro_rules! lo {
  ($dw:expr) => {{
    ($dw & 0xff) as u8
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
  ($fname:ident, $reg_hi:ident, $reg_lo:ident) => (
    pub fn $fname(&mut self) {
      let mut dw = dword!(self.$reg_hi, self.$reg_lo);
      dw = dw.wrapping_sub(1);
      self.$reg_hi = hi!(dw);
      self.$reg_lo = lo!(dw);
    }
  )
}

macro_rules! inc_dword_reg {
  ($fname:ident, $reg_hi:ident, $reg_lo:ident) => (
    pub fn $fname(&mut self) {
      let mut dw = dword!(self.$reg_hi, self.$reg_lo);
      dw = dw.wrapping_add(1);
      self.$reg_hi = hi!(dw);
      self.$reg_lo = lo!(dw);
    }
  )
}

macro_rules! bitn {
  ($val:expr, $n:expr) => {{
    ($val >> $n) & 0x1
  }};
}

macro_rules! cpu_flag_fn {
  ($getter_fn:ident, $setter_fn:ident, $reset_fn:ident, $bitnum:expr) => (
    pub fn $getter_fn(&self) -> bool { bitn!(self.reg_f, $bitnum) == 0x1 }
    pub fn $setter_fn(&mut self, val: u8) { self.reg_f = self.reg_f.wrapping_shr($bitnum + 1).wrapping_shl($bitnum + 1) | (self.reg_f & ((1 << $bitnum) - 1)) | (val << $bitnum) }
    pub fn $reset_fn(&mut self) { self.$setter_fn(0); }
  );
}

macro_rules! op_bit_test {
  ($sel:ident, $reg:ident, $bit:expr) => {{
    if bitn!($sel.cpu.$reg, $bit) == 0x0 {
      $sel.cpu.set_flag_zero(0x1);
    }
    $sel.cpu.reset_flag_add_sub();
    $sel.cpu.set_flag_half_carry(0x1);
  }};
}

macro_rules! op_dec_reg {
  ($sel:ident, $reg:ident) => {{
    if !Util::has_half_borrow($sel.cpu.$reg, 0x1) {
      $sel.cpu.set_flag_half_carry(0x1);
    }
    $sel.cpu.$reg = $sel.cpu.$reg.wrapping_sub(1);
    if $sel.cpu.$reg == 0x0 {
      $sel.cpu.set_flag_zero(0x1);
    }
    $sel.cpu.set_flag_add_sub(0x1);
  }};
}

macro_rules! op_inc_reg {
  ($sel:ident, $reg:ident) => {{
    if Util::has_half_carry($sel.cpu.$reg, 0x1) {
      $sel.cpu.set_flag_half_carry(0x1);
    }
    $sel.cpu.$reg = $sel.cpu.$reg.wrapping_add(1);
    if $sel.cpu.$reg == 0x0 {
      $sel.cpu.set_flag_zero(0x1);
    }
    $sel.cpu.reset_flag_add_sub();
  }};
}

macro_rules! rot_left_reg {
  ($sel:ident, $reg:ident) => {{
    let old_carry = if $sel.cpu.flag_carry() { 1 } else { 0 };
    $sel.cpu.set_flag_carry(bitn!($sel.cpu.$reg, 0x7));

    $sel.cpu.$reg = ($sel.cpu.$reg << 1) | old_carry;

    if $sel.cpu.$reg == 0x0 {
      $sel.cpu.set_flag_zero(0x1);
    }

    $sel.cpu.reset_flag_add_sub();
    $sel.cpu.reset_flag_half_carry();
  }};
}

macro_rules! op_sub_reg_from_a {
  ($sel:ident, $reg:ident) => {{
    if !Util::has_half_borrow($sel.cpu.reg_a, $sel.cpu.$reg) {
      $sel.cpu.set_flag_half_carry(0x1);
    }
    if !Util::has_borrow($sel.cpu.reg_a, $sel.cpu.$reg) {
      $sel.cpu.set_flag_carry(0x1);
    }

    $sel.cpu.reg_a = $sel.cpu.reg_a.wrapping_sub($sel.cpu.$reg);

    if $sel.cpu.reg_a == 0x0 {
      $sel.cpu.set_flag_zero(0x1);
    }
    $sel.cpu.set_flag_add_sub(0x1);
  }};
}

macro_rules! op_cp_with_a {
  ($sel:ident, $reg:ident) => {{
    if !Util::has_half_borrow($sel.cpu.reg_a, $sel.cpu.$reg) {
      $sel.cpu.set_flag_half_carry(0x1);
    }
    if !Util::has_borrow($sel.cpu.reg_a, $sel.cpu.$reg) {
      $sel.cpu.set_flag_carry(0x1);
    }

    if $sel.cpu.reg_a == $sel.cpu.$reg {
      $sel.cpu.set_flag_zero(0x1);
    }
    $sel.cpu.set_flag_add_sub(0x1);
  }};
}
