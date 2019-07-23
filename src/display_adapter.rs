use std::fmt;

pub trait PixelDrawer {
  fn clear(&mut self);
  fn pixel(&mut self, x: usize, y: usize);
  fn draw(&self);
  fn on(&mut self);
  fn off(&mut self);
}

pub struct ConsoleDisplay {
  buffer: [bool; 160 * 144],
}

impl Default for ConsoleDisplay {
  fn default() -> Self {
    ConsoleDisplay {
      buffer: [false; 160 * 144],
    }
  }
}

impl fmt::Debug for ConsoleDisplay {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[ConsoleDisplay]")
  }
}

impl PixelDrawer for ConsoleDisplay {
  fn clear(&mut self) {
    self.buffer = [false; 160 * 144];
  }

  fn pixel(&mut self, x: usize, y: usize) {
    if x >= 160 || y >= 144 {
      return;
    }

    self.buffer[y * 160 + x] = true;
  }

  fn draw(&self) {
    for y in 0..144 {
      println!(
        "{:?}",
        self.buffer[(y * 160)..(y * 160 + 144)]
          .iter()
          .map(|&v| if v { 'X' } else { '_' })
          .collect::<String>()
      );
    }
  }

  fn on(&mut self) {}
  fn off(&mut self) {}
}
