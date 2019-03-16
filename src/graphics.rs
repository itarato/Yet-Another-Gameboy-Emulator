use super::display_adapter::*;

enum WindowTileMapDisplayRegion {
  Region_0x9800_0x9BFF,
  Region_0x9C00_0x9FFF,
}

enum BackgroundAndWindowTileDataRegion {
  Region_0x8800_0x97FF,
  Region_0x8000_0x8FFF,
}

enum BackgroundTileMapDisplayRegion {
  Region_0x9800_0x9BFF,
  Region_0x9C00_0x9FFF,
}

enum ObjectSpriteSize {
  Size8x8,
  Size8x16,
}

pub struct Graphics {
  lcdc: u8,
  scy: u8,
  bgp: u8,
  ly_lcdc_y_coordinate: u8,
  display: ConsoleDisplay,
  mode_timer: u64,
  line: u8,
  stat: u8,
  vmem: [u8; 0x2000],
}

impl Default for Graphics {
  fn default() -> Graphics {
    Graphics {
      vmem: [0; 0x2000],
      lcdc: 0,
      scy: 0,
      bgp: 0,
      ly_lcdc_y_coordinate: 0,
      display: Default::default(),
      mode_timer: 0,
      line: 0,
      stat: 0,
    }
  }
}

impl Graphics {
  pub fn reset(&mut self) {}

  pub fn write_word(&mut self, addr: u16, w: u8) {
    match addr {
      0x8000...0x9fff => {
        if !self.is_vmem_used() {
          self.vmem[(addr - 0x8000) as usize] = w;
        }
      }
      0xff40 => self.set_lcdc(w),
      0xff42 => self.scy = w,
      0xff44 => self.ly_lcdc_y_coordinate = 0x0,
      0xff47 => self.bgp = w,
      _ => unimplemented!("Unknown graphics address: 0x{:>04x}", addr),
    };
  }

  fn is_vmem_used(&self) -> bool {
    // TODO Implement
    false
  }

  pub fn update(&mut self, cycles_prev: u64, cycles: u64) {
    assert!(cycles_prev < cycles);

    self.mode_timer += cycles - cycles_prev;

    match self.stat_mode() {
      0b10 => {
        // Accessing OAM.
        if self.mode_timer >= 320 {
          self.mode_timer = self.mode_timer % 320;
          self.set_stat_mode(0b11);
        }
      }
      0b11 => {
        // Accessing VRAM.
        // TODO  mode 3 about 170-240 tstates depending on where exactly the sprites, window, and fine scroll (SCX modulo 8) are positioned
        // !!! Draw should happen here !!!
        if self.mode_timer >= 688 {
          self.mode_timer = self.mode_timer % 688;
          self.set_stat_mode(0b00);
        }
      }
      0b00 => {
        // Horizontal blank.
        if self.mode_timer >= 816 {
          self.mode_timer = self.mode_timer % 816;

          self.line += 1;

          if self.line == 143 {
            self.set_stat_mode(0b01);
          // TODO Possibly do something on screen .. http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
          // Possibly not.
          } else {
            self.set_stat_mode(0b10);
          }
        }
      }
      0b01 => {
        if self.mode_timer >= 18240 {
          // Vertical blank.
          self.mode_timer = self.mode_timer % 18240;
          self.line = 0;
        }
      }
      mode @ _ => panic!("Illegal LCDC mode: 0b{:b}", mode),
    }
  }

  fn stat_mode(&self) -> u8 {
    self.stat & 0b11
  }

  fn set_stat_mode(&mut self, mode: u8) {
    assert!(mode <= 0b11);
    self.stat = (self.stat & 0b1111_1100) | (mode & 0b11);
  }

  fn set_lcdc(&mut self, w: u8) {
    let changed_bits = w ^ self.lcdc;
    self.lcdc = w;

    if bitn!(changed_bits, 7) == 0x1 {
      if bitn!(self.lcdc, 7) == 0x1 {
        // Turn display on.
        // Should draw immediately with LY = 0.
        self.display.on();
      } else {
        // Turn display off.
        self.display.off();
      }
    }

    if bitn!(changed_bits, 5) == 0x1 {
      unimplemented!("Window display enabled bit operation has not been implemented.");
    }
  }

  fn window_tile_map_display_select(&self) -> WindowTileMapDisplayRegion {
    if bitn!(self.lcdc, 6) == 0x0 {
      WindowTileMapDisplayRegion::Region_0x9800_0x9BFF
    } else {
      WindowTileMapDisplayRegion::Region_0x9C00_0x9FFF
    }
  }

  fn is_window_display_enabled(&self) -> bool {
    bitn!(self.lcdc, 5) == 0x1
  }

  fn background_and_window_tile_data_select(&self) -> BackgroundAndWindowTileDataRegion {
    if bitn!(self.lcdc, 4) == 0x0 {
      BackgroundAndWindowTileDataRegion::Region_0x8800_0x97FF
    } else {
      BackgroundAndWindowTileDataRegion::Region_0x8000_0x8FFF
    }
  }

  fn background_tile_map_display_select(&self) -> BackgroundTileMapDisplayRegion {
    if bitn!(self.lcdc, 3) == 0x0 {
      BackgroundTileMapDisplayRegion::Region_0x9800_0x9BFF
    } else {
      BackgroundTileMapDisplayRegion::Region_0x9C00_0x9FFF
    }
  }

  fn object_sprite_size(&self) -> ObjectSpriteSize {
    if bitn!(self.lcdc, 2) == 0x0 {
      ObjectSpriteSize::Size8x8
    } else {
      ObjectSpriteSize::Size8x16
    }
  }

  fn object_sprite_display_enable(&self) -> bool {
    bitn!(self.lcdc, 1) == 0x1
  }

  fn background_window_display_priority(&self) -> bool {
    bitn!(self.lcdc, 0) == 0x1
  }

  pub fn draw_display(&self) {
    self.display.draw();
  }
}
