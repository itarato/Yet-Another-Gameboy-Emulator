use super::util::*;

#[derive(Default)]
pub struct Mem {
  mem: Vec<u8>,
}

impl Mem {
  pub fn new() -> Mem {
    Default::default()
  }

  pub fn reset(&mut self) {
    self.mem = vec![0; 0x10000];
  }

  pub fn write_word(&mut self, addr: u16, w: u8) {
    if Util::in_range(0xff00, 0xff4c, addr) || Util::in_range(0xff80, 0xffff, addr) {
      self.mem[addr as usize] = w;
    } else if Util::in_range(0xe000, 0xfe00, addr) {
      self.mem[addr as usize] = w;
      self.mem[(0xc000 + (addr - 0xe000)) as usize] = w;
    } else if Util::in_range(0xc000, 0xde00, addr) {
      self.mem[addr as usize] = w;
      self.mem[(0xe000 + (addr - 0xc000)) as usize] = w;
    } else {
      unimplemented!("Memory write to 0x{:x} is not implemented.", addr);
    }
  }

  pub fn read_word(&self, addr: u16) -> u8 {
    self.mem[addr as usize]
  }
}

#[test]
fn test_echo_mem() {
  let mut m = Mem::new();
  m.reset();
  m.write_word(0xc000, 12);
  assert!(m.read_word(0xc000) == 12);
  assert!(m.read_word(0xe000) == 12);
}
