#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;

#[macro_use]
pub mod macros;

pub mod cpu;
pub mod debugger;
pub mod display_adapter;
pub mod emu;
pub mod graphics;
pub mod mem;
pub mod sound;
pub mod timer;
pub mod util;

use self::emu::*;

fn main() {
  env_logger::init();

  info!("Emulator start");

  let mut emu = Emu::new();
  // emu.enable_debug_mode();
  emu.run();
}
