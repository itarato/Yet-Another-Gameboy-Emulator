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
use std::env;

fn main() {
  env_logger::init();

  info!("Emulator start");

  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Missing argument. Call: cargo run -- CARTRIGE [--debug]");
  }

  let mut emu = Emu::new(args[1].clone());

  if args.iter().find(|&arg| arg == "--debug").is_some() {
    emu.enable_debug_mode();
  }
  emu.run();
}
