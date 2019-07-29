#[derive(Debug, Default)]
pub struct Serial;

impl Serial {
  pub fn write_word(&self, addr: u16, w: u8) {
    // @TODO Implement.
    // println!(
    //   "Serial data was received: #0x{:>04x} -> 0x{:>02x} ({:?})",
    //   addr, w, w as char
    // );

    if w != 0x81 {
      print!("{}", w as char);
    }
  }
}
