// 4'194'304 Hz (4194304)
// Div 4 = 1048576 (machine cycles per sec)

// DIV: 16384

use super::util::*;

enum InputClockSpeed {
  Hz4096,
  Hz262144,
  Hz65536,
  Hz16384,
}

#[derive(Debug, Default)]
pub struct TimerResult {
  pub interrupt_generated: bool,
}

impl TimerResult {
  fn new(interrupt_generated: bool) -> TimerResult {
    TimerResult {
      interrupt_generated,
    }
  }
}

#[derive(Debug, Default)]
pub struct Timer {
  div: u8,
  tima: u8,
  tma: u8,
  tac: u8,
}

impl Timer {
  pub fn reset(&mut self) {}

  pub fn update(&mut self, cycles_prev: u64, cycles: u64) -> TimerResult {
    let mut result: TimerResult = TimerResult::default();

    if Util::did_tick_happened(cycles_prev, cycles, 16384) {
      self.div.wrapping_add(1);
    }

    if self.timer_enabled() {
      if match self.input_clock_select() {
        InputClockSpeed::Hz4096 => Util::did_tick_happened(cycles_prev, cycles, 4096),
        InputClockSpeed::Hz262144 => Util::did_tick_happened(cycles_prev, cycles, 262144),
        InputClockSpeed::Hz65536 => Util::did_tick_happened(cycles_prev, cycles, 65536),
        InputClockSpeed::Hz16384 => Util::did_tick_happened(cycles_prev, cycles, 16384),
      } {
        if self.tima == 0xff {
          self.tima = self.tma;
          result.interrupt_generated = true;
        } else {
          self.tima.wrapping_add(0x1);
        }
      }
    }

    result
  }

  pub fn write_word(&mut self, addr: u16, w: u8) {
    match addr {
      0xff04 => self.div = 0x0,
      0xff06 => self.tma = w,
      0xff07 => self.tac = w,
      _ => unimplemented!("Timer reg write is not yet implemented on 0x{:>04x}", addr),
    }
  }

  fn timer_enabled(&self) -> bool {
    bitn!(self.tac, 2) == 0x1
  }

  fn input_clock_select(&self) -> InputClockSpeed {
    match self.tac & 0x11 {
      0x00 => InputClockSpeed::Hz4096,
      0x01 => InputClockSpeed::Hz262144,
      0x10 => InputClockSpeed::Hz65536,
      0x11 => InputClockSpeed::Hz16384,
      bits @ _ => panic!("Invalid input clock selector bits: 0b{:b}", bits),
    }
  }
}
