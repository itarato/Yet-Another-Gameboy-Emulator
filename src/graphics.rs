use super::display_adapter::*;
use super::util::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::rc::Rc;

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

enum GdbColor {
  C0,
  C1,
  C2,
  C3,
}

impl GdbColor {
  fn as_sdl_color(&self) -> Color {
    match self {
      GdbColor::C3 => Color::RGB(0, 0, 0),
      GdbColor::C2 => Color::RGB(85, 85, 85),
      GdbColor::C1 => Color::RGB(170, 170, 170),
      GdbColor::C0 => Color::RGB(255, 255, 255),
    }
  }
}

#[derive(Default, Debug)]
struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn new(x: usize, y: usize) -> Point {
    Point { x, y }
  }
}

#[derive(Debug, Default)]
pub struct GraphicsUpdateResult {
  pub vblank_interrupt_generated: bool,
}

pub struct Graphics {
  lcdc: u8,
  scx: u8,
  scy: u8,
  bgp: u8,
  ly_lcdc_y_coordinate: u8,
  display: ConsoleDisplay,
  mode_timer: u64,
  line: u8,
  stat: u8,
  vmem: [u8; 0x2000],
  oam: [u8; 0xa0],
  canvas: WindowCanvas,
  bg_debug_canvas: WindowCanvas,
}

impl Graphics {
  pub fn new(sdl: Rc<Sdl>) -> Graphics {
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
      .window(
        "Y.A.G.B.E.",
        160 * Graphics::scale() as u32,
        144 * Graphics::scale() as u32,
      )
      .opengl()
      .build()
      .unwrap();

    let debug_window = video_subsystem
      .window(
        "Y.A.G.B.E. BACKGROUND DEBUG",
        256 * Graphics::scale() as u32,
        256 * Graphics::scale() as u32,
      )
      .opengl()
      .build()
      .unwrap();

    Graphics {
      vmem: [0; 0x2000],
      oam: [0; 0xa0],
      lcdc: 0,
      scx: 0,
      scy: 0,
      bgp: 0,
      ly_lcdc_y_coordinate: 0,
      display: Default::default(),
      mode_timer: 0,
      line: 0,
      stat: 0,
      canvas: window.into_canvas().build().unwrap(),
      bg_debug_canvas: debug_window.into_canvas().build().unwrap(),
    }
  }

  pub fn reset(&mut self) {
    self.lcdc = 0x91;
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
    self.canvas.present();
  }

  pub fn write_word(&mut self, addr: u16, w: u8) {
    let stat_mode = self.stat_mode();
    assert!(stat_mode <= 0b11);

    match addr {
      0x8000...0x9fff => {
        if stat_mode != 0b11 {
          self.vmem[(addr - 0x8000) as usize] = w;
        } else {
          debug!("VMEM write is ignored.");
        }
      }
      0xfe00...0xfe9f => {
        // Sprite attribute table (OAM).
        if stat_mode == 0b00 || stat_mode == 0b01 {
          self.oam[addr as usize - 0xfe00] = w;
        } else {
          debug!("OAM write is ignored.");
        }
      }
      0xff40 => self.set_lcdc(w),
      0xff41 => self.stat = (self.stat & 0b111) | (w & 0b1111_1000),
      0xff42 => self.scy = w,
      0xff43 => self.scx = w,
      0xff44 => self.ly_lcdc_y_coordinate = 0x0,
      0xff47 => self.bgp = w,
      _ => unimplemented!("Unknown graphics address: 0x{:>04x}", addr),
    };
  }

  pub fn read_word(&self, addr: u16, force_read: bool) -> u8 {
    let stat_mode = self.stat_mode();
    assert!(stat_mode <= 0b11);

    match addr {
      0xfe00...0xfe9f => {
        // Sprite attribute table (OAM).
        if force_read || stat_mode == 0b00 || stat_mode == 0b01 {
          self.oam[addr as usize - 0xfe00]
        } else {
          debug!("OAM read is ignored.");
          0xff
        }
      }
      0x8000...0x9fff => {
        if !force_read && stat_mode == 0b11 {
          debug!("VMEM read is ignored.");
          0xff
        } else {
          self.vmem[addr as usize - 0x8000]
        }
      }
      0xff40 => self.lcdc,
      0xff41 => self.stat,
      0xff42 => self.scy,
      0xff43 => self.scx,
      0xff44 => self.ly_lcdc_y_coordinate,
      0xff47 => self.bgp,
      _ => unimplemented!("Unrecognized video address: 0x{:>04x}", addr),
    }
  }

  pub fn update(&mut self, cycles_prev: u64, cycles: u64) -> GraphicsUpdateResult {
    assert!(cycles_prev < cycles);
    let mut response = GraphicsUpdateResult::default();

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
          // HERE DRAW LINE <self.line> from backstage.
          self.draw_hline(self.line);

          if self.line == 0 {
            self.update_debug_background_window();
          }

          self.mode_timer = self.mode_timer % 688;
          self.set_stat_mode(0b00);
        }
      }
      0b00 => {
        // Horizontal blank.
        if self.mode_timer >= 816 {
          self.mode_timer = self.mode_timer % 816;

          self.line += 1;

          if self.line == 144 {
            // This was 143 but seems we need all 0-143 to be accessible in state 0b11.
            self.set_stat_mode(0b01);
            // TODO Possibly do something on screen .. http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
            // Possibly not.
            response.vblank_interrupt_generated = true;
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
          self.set_stat_mode(0b10);
        }
      }
      mode @ _ => panic!("Illegal LCDC mode: 0b{:b}", mode),
    }

    response
  }

  fn draw_hline(&mut self, line: u8) {
    for i in 0..160 {
      self.clear_pixel(Point::new(i, line as usize));
    }

    // Background.
    let row = ((line as i32 + self.scy as i32) % 256) / 8;
    let tile_line: i32 = (line as i32 + self.scy as i32) % 8;

    for col in 0..32 {
      let orig_x = col * 8;
      let virt_x: i32 = (orig_x as i32 + 256 - self.scx as i32) % 256;

      // The tile line would not be presented on the visible line scan.
      if virt_x >= 160 {
        continue;
      }

      let tile_idx = row as usize * 32 + col;
      let tile_offs: usize = match self.background_tile_map_display_select() {
        BackgroundTileMapDisplayRegion::Region_0x9800_0x9BFF => 0x9800 + tile_idx - 0x8000,
        BackgroundTileMapDisplayRegion::Region_0x9C00_0x9FFF => 0x9c00 + tile_idx - 0x8000,
      };
      let map_region_start: usize = match self.background_and_window_tile_data_select() {
        BackgroundAndWindowTileDataRegion::Region_0x8000_0x8FFF => 0x8000 - 0x8000,
        BackgroundAndWindowTileDataRegion::Region_0x8800_0x97FF => 0x8800 - 0x8000,
      };
      let tile_block_size = 0x10;
      let tile_addr: usize = map_region_start + (self.vmem[tile_offs] as usize * tile_block_size);

      for i in 0..8 {
        // Out of screen.
        if virt_x + i >= 160 {
          continue;
        }

        let color_bit_hi = (self.vmem[tile_addr + (tile_line as usize * 2)] >> (7 - i)) & 1;
        let color_bit_lo = (self.vmem[tile_addr + (tile_line as usize * 2) + 1] >> (7 - i)) & 1;
        let color_code = (color_bit_hi << 1) | color_bit_lo;
        let color = self.color_bit_to_color(color_code);
        self.set_pixel(color, Point::new((virt_x + i) as usize, line as usize));
      }
    }

    self.canvas.present();
  }

  fn update_debug_background_window(&mut self) {
    self.bg_debug_canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.bg_debug_canvas.clear();

    // Background.
    for row in 0..32 {
      let orig_y = row * 8;

      for col in 0..32 {
        let orig_x = col * 8;
        let tile_idx = row * 32 + col;

        let tile_offs: usize = match self.background_tile_map_display_select() {
          BackgroundTileMapDisplayRegion::Region_0x9800_0x9BFF => 0x9800 + tile_idx - 0x8000,
          BackgroundTileMapDisplayRegion::Region_0x9C00_0x9FFF => 0x9c00 + tile_idx - 0x8000,
        };
        let map_region_start: usize = match self.background_and_window_tile_data_select() {
          BackgroundAndWindowTileDataRegion::Region_0x8000_0x8FFF => 0x8000 - 0x8000,
          BackgroundAndWindowTileDataRegion::Region_0x8800_0x97FF => 0x8800 - 0x8000,
        };
        let tile_block_size = 0x10;
        let tile_addr: usize = map_region_start + (self.vmem[tile_offs] as usize * tile_block_size);

        for iy in 0..8 {
          for ix in 0..8 {
            let color_bit_hi = (self.vmem[tile_addr + (iy * 2)] >> (7 - ix)) & 1;
            let color_bit_lo = (self.vmem[tile_addr + (iy * 2) + 1] >> (7 - ix)) & 1;
            let color_code = (color_bit_hi << 1) | color_bit_lo;
            let color = self.color_bit_to_color(color_code);
            self.bg_debug_canvas.set_draw_color(color.as_sdl_color());
            let _ = self.bg_debug_canvas.fill_rect(Rect::new(
              ((orig_x + ix) * Graphics::scale()) as i32,
              ((orig_y + iy) * Graphics::scale()) as i32,
              Graphics::scale() as u32,
              Graphics::scale() as u32,
            ));
          }
        }
      }
    }

    self.bg_debug_canvas.present();
  }

  fn color_bit_to_color(&self, bitmask: u8) -> GdbColor {
    match bitmask {
      0b00 => GdbColor::C0,
      0b01 => GdbColor::C1,
      0b10 => GdbColor::C2,
      0b11 => GdbColor::C3,
      _ => panic!("Invalid color bitmask."),
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

  fn clear_pixel(&mut self, coord: Point) {
    self.set_pixel(GdbColor::C0, coord);
  }

  fn set_pixel(&mut self, color: GdbColor, coord: Point) {
    assert!(coord.x < 160 && coord.y < 144);

    self.canvas.set_draw_color(color.as_sdl_color());
    let _ = self.canvas.fill_rect(Rect::new(
      (coord.x * Graphics::scale()) as i32,
      (coord.y * Graphics::scale()) as i32,
      Graphics::scale() as u32,
      Graphics::scale() as u32,
    ));
  }

  fn scale() -> usize {
    2
  }
}
