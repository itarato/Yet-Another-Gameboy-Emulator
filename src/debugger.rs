use std::collections::HashSet;
use std::io::{self, Write};

#[derive(Clone, Copy)]
pub enum DebuggerCommand {
  Next,
  Continue,
  Empty,
  Breakpoint,
  MemoryPrint(u16, usize),
  CpuPrint,
  Quit,
}

#[derive(Default)]
pub struct Debugger {
  breakpoints: HashSet<u16>,
  next_count: Option<usize>,
}

impl Debugger {
  pub fn new() -> Debugger {
    let mut debugger: Debugger = Default::default();
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
      "memory" | "mem" | "m" => {
        let addr = u16::from_str_radix(parts[1], 16).unwrap();
        let len = usize::from_str_radix(parts[2], 10).unwrap();
        DebuggerCommand::MemoryPrint(addr, len)
      }
      "cpu" => DebuggerCommand::CpuPrint,
      "exit" | "e" | "quit" | "q" => DebuggerCommand::Quit,
      _ => {
        debug!("Unknown debugger command.");
        DebuggerCommand::Empty
      }
    }
  }
}