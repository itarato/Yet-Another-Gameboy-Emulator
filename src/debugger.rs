use super::cpu::*;
use super::graphics::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{ttf, Sdl};
use std::collections::HashSet;
use std::io::{self, Write};
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum DebuggerCommand {
  Next,
  Continue,
  Invalid,
  Breakpoint,
  MemoryPrint(u16, usize),
  CpuPrint,
  Quit,
  Display,
  PrintBackgroundMap,
}

pub struct Debugger {
  breakpoints: HashSet<u16>,
  next_count: Option<usize>,
  sdl: Rc<Sdl>,
  bg_debug_canvas: WindowCanvas,
  ttf_context: Sdl2TtfContext,
}

impl Debugger {
  pub fn new(sdl: Rc<Sdl>) -> Debugger {
    let video_subsystem = sdl.video().unwrap();
    let debug_window = video_subsystem
      .window(
        "Y.A.G.B.E. BACKGROUND DEBUG",
        256 * Debugger::scale() as u32,
        256 * Debugger::scale() as u32,
      )
      .position(16 + 160 * Debugger::scale() as i32 + 16, 64)
      .opengl()
      .build()
      .unwrap();

    let mut debugger: Debugger = Debugger {
      breakpoints: HashSet::new(),
      next_count: None,
      sdl,
      bg_debug_canvas: debug_window.into_canvas().build().unwrap(),
      ttf_context: ttf::init().unwrap(),
    };
    // Break at start.
    debugger.breakpoints.insert(0x0);
    debugger
  }

  pub fn should_break(&mut self, pc: u16) -> bool {
    if let Some(next_count) = self.next_count {
      if next_count == 1 {
        self.next_count = None;
        return true;
      } else {
        self.next_count = Some(next_count - 1);
      }
    }

    if self.breakpoints.contains(&pc) {
      println!("[YAGBE] -- Breakpoint at 0x{:x}", pc);
      return true;
    }

    false
  }

  pub fn read_command(&mut self) -> DebuggerCommand {
    let mut buffer = String::new();

    print!("[YAGBE]> ");
    let _ = std::io::stdout().flush();

    let _ = io::stdin().read_line(&mut buffer).unwrap();

    let parts = buffer.trim().split(' ').collect::<Vec<&str>>();

    match parts[0] {
      "next" | "n" => {
        let n = if parts.len() > 1 {
          usize::from_str_radix(parts[1], 10).unwrap()
        } else {
          1
        };
        self.next_count = Some(n);

        DebuggerCommand::Next
      }
      "continue" | "c" | "run" => DebuggerCommand::Continue,
      "breakpoint" | "break" | "b" => {
        self
          .breakpoints
          .insert(u16::from_str_radix(parts[1], 16).unwrap());
        DebuggerCommand::Breakpoint
      }
      "-breakpoint" | "-break" | "-b" => {
        self
          .breakpoints
          .remove(&u16::from_str_radix(parts[1], 16).unwrap());
        DebuggerCommand::Breakpoint
      }
      "memory" | "mem" | "m" => {
        let addr = u16::from_str_radix(parts[1], 16).unwrap();
        let len = if parts.len() > 2 {
          usize::from_str_radix(parts[2], 10).unwrap()
        } else {
          1
        };
        DebuggerCommand::MemoryPrint(addr, len)
      }
      "backgroundmap" | "bgmap" | "bgm" => DebuggerCommand::PrintBackgroundMap,
      "cpu" => DebuggerCommand::CpuPrint,
      "display" | "d" => DebuggerCommand::Display,
      "exit" | "e" | "quit" | "q" => DebuggerCommand::Quit,
      cmd @ _ => {
        debug!("Unknown debugger command.");
        println!("Unrecognized debugger command: {:#?}", cmd);
        DebuggerCommand::Invalid
      }
    }
  }

  // @TODO Move this to debugger.
  pub fn update_debug_background_window(
    &mut self,
    iteration_count: u64,
    cpu: &Cpu,
    graphics: &Graphics,
  ) {
    self.bg_debug_canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.bg_debug_canvas.clear();

    // Background.
    for row in 0..32 {
      let orig_y = row * 8;

      for col in 0..32 {
        let orig_x = col * 8;
        let tile_idx = row * 32 + col;

        let tile_offs: usize = match graphics.background_tile_map_display_select() {
          BackgroundTileMapDisplayRegion::Region_0x9800_0x9BFF => 0x9800 + tile_idx - 0x8000,
          BackgroundTileMapDisplayRegion::Region_0x9C00_0x9FFF => 0x9c00 + tile_idx - 0x8000,
        };
        let map_region_start: usize = match graphics.background_and_window_tile_data_select() {
          BackgroundAndWindowTileDataRegion::Region_0x8000_0x8FFF => 0x8000 - 0x8000,
          BackgroundAndWindowTileDataRegion::Region_0x8800_0x97FF => 0x8800 - 0x8000,
        };
        let tile_block_size = 0x10;
        let tile_addr: usize =
          map_region_start + (graphics.vmem[tile_offs] as usize * tile_block_size);

        for iy in 0..8 {
          for ix in 0..8 {
            let color_bit_hi = (graphics.vmem[tile_addr + (iy * 2)] >> (7 - ix)) & 1;
            let color_bit_lo = (graphics.vmem[tile_addr + (iy * 2) + 1] >> (7 - ix)) & 1;
            let color_code = (color_bit_hi << 1) | color_bit_lo;
            let color = graphics.color_bit_to_color(color_code);
            self.bg_debug_canvas.set_draw_color(color.as_sdl_color());
            let _ = self.bg_debug_canvas.fill_rect(Rect::new(
              ((orig_x + ix) * Debugger::scale()) as i32,
              ((orig_y + iy) * Debugger::scale()) as i32,
              Debugger::scale() as u32,
              Debugger::scale() as u32,
            ));
          }
        }
      }
    }

    self
      .bg_debug_canvas
      .set_draw_color(Color::RGBA(0, 200, 0, 100));
    self.bg_debug_canvas.draw_rect(Rect::new(
      graphics.scx as i32 * Debugger::scale() as i32,
      graphics.scy as i32 * Debugger::scale() as i32,
      160 * Debugger::scale() as u32,
      144 * Debugger::scale() as u32,
    ));

    self.render_text(format!("#{:0>16?}", iteration_count), 0);
    self.render_text(
      format!(
        "A: 0x{:0>2x?} F: 0x{:0>2x?} Z{:?} N{:?} H{:?} C{:?}",
        cpu.reg_a,
        cpu.reg_f,
        bitn!(cpu.reg_f, 7),
        bitn!(cpu.reg_f, 6),
        bitn!(cpu.reg_f, 5),
        bitn!(cpu.reg_f, 4)
      ),
      16,
    );
    self.render_text(
      format!("B: 0x{:0>2x?} C: 0x{:0>2x?}", cpu.reg_b, cpu.reg_c),
      32,
    );
    self.render_text(
      format!("D: 0x{:0>2x?} E: 0x{:0>2x?}", cpu.reg_d, cpu.reg_e),
      48,
    );
    self.render_text(
      format!("H: 0x{:0>2x?} L: 0x{:0>2x?}", cpu.reg_h, cpu.reg_l),
      64,
    );
    self.render_text(format!("SP: 0x{:0>4x} PC: 0x{:0>4x}", cpu.sp, cpu.pc), 80);
    self.render_text(format!("LCDC 0b{:0>8b}", graphics.lcdc), 96);

    self.bg_debug_canvas.present();
  }

  fn render_text(&mut self, text: String, offs_y: i32) {
    let mut font = self
      .ttf_context
      .load_font(
        "C:\\Users\\itarato\\Downloads\\DroidFamily\\DroidFonts\\DroidSansMono.ttf",
        12,
      )
      .unwrap();
    font.set_style(ttf::FontStyle::BOLD);

    let surface = font
      .render(text.as_ref())
      .solid(Color::RGB(64, 96, 192))
      .unwrap();

    let texture_creator = self.bg_debug_canvas.texture_creator();
    let texture = texture_creator
      .create_texture_from_surface(surface)
      .unwrap();

    let sdl2::render::TextureQuery { width, height, .. } = texture.query();

    let target = sdl2::rect::Rect::new(4, offs_y, width, height);

    self
      .bg_debug_canvas
      .copy(&texture, None, Some(target))
      .unwrap();
  }

  fn scale() -> usize {
    4usize
  }
}
