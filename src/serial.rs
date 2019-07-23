#[derive(Debug, Default)]
pub struct Serial;

impl Serial {
  pub fn write_word(&self, addr: u16, w: u8) {
    // @TODO Implement.
    dbg!("Serial data was received.");
  }
}
