use super::cpu::*;
use super::emu::*;
use super::graphics::*;
use super::util::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{ttf, Sdl};
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdin, stdout, Write};
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
  History,
  BackgroundOn,
  BackgroundOff,
  LogOn,
  LogOff,
  PrintCpuOn,
  PrintCpuOff,
}

pub struct Debugger {
  breakpoints: HashSet<u16>,
  next_count: Option<usize>,
  bg_debug_canvas: WindowCanvas,
  tile_debug_canvas: WindowCanvas,
  ttf_context: Sdl2TtfContext,
  pc_history: History<u16>,
  debug_displays_on: bool,
  log: File,
  log_on: bool,
  print_cpu: bool,
}

impl Debugger {
  pub fn new(sdl: Rc<Sdl>) -> Debugger {
    let video_subsystem = sdl.video().unwrap();
    let background_debug_window = video_subsystem
      .window(
        "Y.A.G.B.E. BACKGROUND DEBUG",
        256 * Debugger::scale() as u32,
        256 * Debugger::scale() as u32,
      )
      .position(16 + 160 * Debugger::scale() as i32 + 16, 64)
      .opengl()
      .build()
      .unwrap();


    let tile_debug_window = video_subsystem
      .window(
        "Y.A.G.B.E. TILE DEBUG",
        128 * Debugger::scale() as u32,
        192 * Debugger::scale() as u32,
      )
      .position(32 + 416 * Debugger::scale() as i32 + 16, 64)
      .opengl()
      .build()
      .unwrap();

    let mut debugger: Debugger = Debugger {
      breakpoints: HashSet::new(),
      next_count: None,
      bg_debug_canvas: background_debug_window.into_canvas().build().unwrap(),
      tile_debug_canvas: tile_debug_window.into_canvas().build().unwrap(),
      ttf_context: ttf::init().unwrap(),
      pc_history: History::with_capacity(4),
      debug_displays_on: true,
      log: File::create("./debug.log").unwrap(),
      log_on: false,
      print_cpu: false,
    };
    // Break at start.
    debugger.breakpoints.insert(0x0);
    debugger
  }

  pub fn should_break(&mut self, cpu: &Cpu) -> bool {
    self.pc_history.push(cpu.pc);

    if self.log_on {
      let _ = write!(self.log, "PC: 0x{:>04x} |> AF: {:>02x}{:>02x} BC: {:>02x}{:>02x} DE: {:>02x}{:>02x} HL: {:>02x}{:>02x} |> SP {:>04x}\n", cpu.pc, cpu.reg_a, cpu.reg_f, cpu.reg_b, cpu.reg_c, cpu.reg_d, cpu.reg_e, cpu.reg_h, cpu.reg_l, cpu.sp);
    }

    if self.print_cpu {
      println!("[YAGBE] -- PC: 0x{:>04x} |> AF: {:>02x}{:>02x} BC: {:>02x}{:>02x} DE: {:>02x}{:>02x} HL: {:>02x}{:>02x} |> SP {:>04x}\n", cpu.pc, cpu.reg_a, cpu.reg_f, cpu.reg_b, cpu.reg_c, cpu.reg_d, cpu.reg_e, cpu.reg_h, cpu.reg_l, cpu.sp)
    }

    if let Some(next_count) = self.next_count {
      if next_count == 1 {
        self.next_count = None;
        return true;
      } else {
        self.next_count = Some(next_count - 1);
      }
    }

    if self.breakpoints.contains(&(cpu.pc)) {
      println!("[YAGBE] -- Breakpoint at 0x{:x}", cpu.pc);
      return true;
    }

    false
  }

  pub fn read_command(&mut self) -> DebuggerCommand {
    let mut buffer = String::new();

    print!("[YAGBE]> ");
    let _ = stdout().flush();

    let _ = stdin().read_line(&mut buffer).unwrap();

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
      "background-on" | "bg-on" => {
        self.debug_displays_on = true;
        DebuggerCommand::BackgroundOn
      }
      "background-off" | "bg-off" => {
        self.debug_displays_on = false;
        DebuggerCommand::BackgroundOff
      }
      "cpu" => DebuggerCommand::CpuPrint,
      "history" | "h" => DebuggerCommand::History,
      "display" | "d" => DebuggerCommand::Display,
      "log-on" | "lon" => {
        self.log_on = true;
        DebuggerCommand::LogOn
      }
      "log-off" | "loff" => {
        self.log_on = false;
        DebuggerCommand::LogOff
      }
      "cpu-print-on" | "cp-on" => {
        self.print_cpu = true;
        DebuggerCommand::PrintCpuOn
      }
      "cpu-print-off" | "cp-off" => {
        self.print_cpu = false;
        DebuggerCommand::PrintCpuOff
      }
      "exit" | "e" | "quit" | "q" => DebuggerCommand::Quit,
      cmd @ _ => {
        debug!("Unknown debugger command.");
        println!("Unrecognized debugger command: {:#?}", cmd);
        DebuggerCommand::Invalid
      }
    }
  }

  pub fn update_debug_windows(&mut self, iteration_count: u64, cpu: &Cpu, graphics: &Graphics) {
    self.update_debug_background_window(iteration_count, cpu, graphics);
    self.update_debug_tile_window(graphics);
  }

  fn update_debug_background_window(
    &mut self,
    iteration_count: u64,
    cpu: &Cpu,
    graphics: &Graphics,
  ) {
    if !self.debug_displays_on {
      return;
    }

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
    let _ = self.bg_debug_canvas.draw_rect(Rect::new(
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

  fn update_debug_tile_window(&mut self, graphics: &Graphics) {
    if !self.debug_displays_on {
      return;
    }

    self.tile_debug_canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.tile_debug_canvas.clear();

    // Background.
    for row in 0..24 {
      let orig_y = row * 8;

      for col in 0..16 {
        let orig_x = col * 8;
        let tile_addr: usize = (row * 16 * 16) + (col * 16);

        for iy in 0..8 {
          for ix in 0..8 {
            let color_bit_hi = (graphics.vmem[tile_addr + (iy * 2)] >> (7 - ix)) & 1;
            let color_bit_lo = (graphics.vmem[tile_addr + (iy * 2) + 1] >> (7 - ix)) & 1;
            let color_code = (color_bit_hi << 1) | color_bit_lo;
            let color = graphics.color_bit_to_color(color_code);
            self.tile_debug_canvas.set_draw_color(color.as_sdl_color());
            let _ = self.tile_debug_canvas.fill_rect(Rect::new(
              ((orig_x + ix) * Debugger::scale()) as i32,
              ((orig_y + iy) * Debugger::scale()) as i32,
              Debugger::scale() as u32,
              Debugger::scale() as u32,
            ));
          }
        }
      }
    }

    self.tile_debug_canvas.present();
  }

  fn render_text(&mut self, text: String, offs_y: i32) {
    let mut font = self
      .ttf_context
      .load_font("asset/DroidSansMono.ttf", 12)
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

  pub fn print_history(&self) {
    println!("{:#x?}", self.pc_history.get());
  }

  fn scale() -> usize {
    2usize
  }

  pub fn dump(&self, emu: &Emu) -> String {
    format!(
      "--- CPU\n{:#x?}\n--- PC LOG\n{:#x?}",
      emu.cpu,
      self.pc_history.get()
    )
  }
}
