use std::fs::File;
use std::io::Read;

use super::cpu::*;
use super::debugger::*;
use super::graphics::*;
use super::mem::*;
use super::sound::*;

use super::util::*;

#[rustfmt::skip]
const OPCODE_DUR: [u8; 256] = [
   4, 12,  8,  8,  4,  4,  8,  4, 20,  8,  8,  8,  4,  4,  8,  4, 
   4, 12,  8,  8,  4,  4,  8,  4, 12,  8,  8,  8,  4,  4,  8,  4, 
  12, 12,  8,  8,  4,  4,  8,  4, 12,  8,  8,  8,  4,  4,  8,  4, 
  12, 12,  8,  8, 12, 12, 12,  4, 12,  8,  8,  8,  4,  4,  8,  4,  
   4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  
   4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  
   4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  
   8,  8,  8,  8,  8,  8,  4,  8,  4,  4,  4,  4,  4,  4,  8,  4, 
   4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  
   4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  
   4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  
   4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4, 
  20, 12, 16, 16, 24, 16,  8, 16, 20, 16, 16,  4, 24, 24,  8, 16, 
  20, 12, 16,  0, 24, 16,  8, 16, 20, 16, 16,  0, 24,  0,  8, 16,
  12, 12,  8,  0,  0, 16,  8, 16, 16,  4, 16,  0,  0,  0,  8, 16, 
  12, 12,  8,  4,  0, 16,  8, 16, 12,  8, 16,  4,  0,  0,  8, 16, 
];

#[rustfmt::skip]
const OPCODE_DUR_ALTERNATIVE: [u8; 256] = [
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 
  8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  8, 0,12, 0,12, 0, 0, 0, 8, 0,12, 0,12, 0, 0, 0, 
  8, 0,12, 0,12, 0, 0, 0, 8, 0,12, 0,12, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[rustfmt::skip]
const OPCODE_DUR_PREFIX: [u8; 256] = [
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,  
  8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
];

#[derive(Default)]
pub struct Emu {
  pub cpu: Cpu,
  pub mem: Mem,
  pub sound: Sound,
  pub graphics: Graphics,
  dmg_rom: Vec<u8>,
  pub cycles: u64,
  debugger: Option<Debugger>,
  halted: bool,
  rom: Vec<u8>,
  interrupts_enabled: bool,
}

impl Emu {
  pub fn new() -> Emu {
    let mut emu: Emu = Default::default();
    emu.reset();
    emu.read_dmg_rom();
    emu.read_rom();
    emu
  }

  pub fn enable_debug_mode(&mut self) {
    self.debugger = Some(Debugger::new());
  }

  pub fn run(&mut self) {
    loop {
      if let Some(debugger) = self.debugger.as_mut() {
        let should_break = debugger.should_break(self.cpu.pc);
        if should_break {
          self.operate_debugger();
        }
      }

      if self.halted {
        return;
      }

      self.read_instruction();
      self.handle_interrupts();
    }
  }

  fn operate_debugger(&mut self) {
    match self.debugger.as_mut().unwrap().read_command() {
      DebuggerCommand::Quit => {
        self.halted = true;
        return;
      }
      DebuggerCommand::MemoryPrint(addr, len) => {
        self.mem_debug_print(addr, len);
      }
      DebuggerCommand::CpuPrint => self.cpu.registers_debug_print(),
      DebuggerCommand::Breakpoint => { /* keep it stopped */ }
      DebuggerCommand::Continue | DebuggerCommand::Next => return,
      _ => {}
    };

    self.operate_debugger();
  }

  fn handle_interrupts(&mut self) {
    if !self.interrupts_enabled {
      return;
    }

    info!("Interrupt check.");

    if self.interrupt_enabled_v_blank() && self.interrupt_flag_v_blank() {
      // Disable interrupts.
      self.interrupts_enabled = false;

      // Disable specific interrupt request.
      let flag_off = Util::setbit(self.read_word(0xff0f), 0, 0x0);
      self.write_word(0xff0f, flag_off);
      self.cycles += 2;

      // Save current PC.
      self.push_dword(self.cpu.pc);
      self.cycles += 2;

      // Jump to interrupt instruction.
      self.cpu.pc = 0x40;
      self.cycles += 1;
    } else if self.interrupt_enabled_lcd_stat() {
      unimplemented!("<lcd_stat> interrupt has not been implemented.");
    } else if self.interrupt_enabled_timer() {
      unimplemented!("<timer> interrupt has not been implemented.");
    } else if self.interrupt_enabled_serial() {
      unimplemented!("<serial> interrupt has not been implemented.");
    } else if self.interrupt_enabled_joypad() {
      unimplemented!("<joypad> interrupt has not been implemented.");
    }
  }

  pub fn read_instruction(&mut self) {
    let opcode = self.read_opcode_word();
    let mut is_cycle_alternative = false;

    info!(
      "Load opcode: 0x{:>02x} at PC: 0x{:>04x}",
      opcode,
      self.cpu.pc - 1
    );

    match opcode {
      // 0x00 | NOP | 1 | 4 | - - - -
      0x00 => {}
      // 0x01 | LD BC,d16 | 3 | 12 | - - - -
      0x01 => load_dword_to_reg!(set_bc, self),
      // 0x02 | LD (BC),A | 1 | 8 | - - - -
      0x02 => unimplemented!("Opcode 0x02 is not yet implemented"),
      // 0x03 | INC BC | 1 | 8 | - - - -
      0x03 => unimplemented!("Opcode 0x03 is not yet implemented"),
      // 0x04 | INC B | 1 | 4 | Z 0 H -
      0x04 => op_inc_reg!(self, reg_b),
      // 0x05 | DEC B | 1 | 4 | Z 1 H -
      0x05 => op_dec_reg!(self, reg_b),
      // 0x06 | LD B,d8 | 2 | 8 | - - - -
      0x06 => load_word_to_reg!(reg_b, self),
      // 0x07 | RLCA | 1 | 4 | 0 0 0 C
      0x07 => unimplemented!("Opcode 0x07 is not yet implemented"),
      // 0x08 | LD (a16),SP | 3 | 20 | - - - -
      0x08 => unimplemented!("Opcode 0x08 is not yet implemented"),
      // 0x09 | ADD HL,BC | 1 | 8 | - 0 H C
      0x09 => unimplemented!("Opcode 0x09 is not yet implemented"),
      // 0x0a | LD A,(BC) | 1 | 8 | - - - -
      0x0a => load_word_to_reg_from_reg_addr!(reg_a, reg_b, reg_c, self),
      // 0x0b | DEC BC | 1 | 8 | - - - -
      0x0b => unimplemented!("Opcode 0x0b is not yet implemented"),
      // 0x0c | INC C | 1 | 4 | Z 0 H -
      0x0c => op_inc_reg!(self, reg_c),
      // 0x0d | DEC C | 1 | 4 | Z 1 H -
      0x0d => op_dec_reg!(self, reg_c),
      // 0x0e | LD C,d8 | 2 | 8 | - - - -
      0x0e => load_word_to_reg!(reg_c, self),
      // 0x0f | RRCA | 1 | 4 | 0 0 0 C
      0x0f => unimplemented!("Opcode 0x0f is not yet implemented"),
      // 0x10 | STOP 0 | 2 | 4 | - - - -
      0x10 => unimplemented!("Opcode 0x10 is not yet implemented"),
      // 0x11 | LD DE,d16 | 3 | 12 | - - - -
      0x11 => load_dword_to_reg!(set_de, self),
      // 0x12 | LD (DE),A | 1 | 8 | - - - -
      0x12 => unimplemented!("Opcode 0x12 is not yet implemented"),
      // 0x13 | INC DE | 1 | 8 | - - - -
      0x13 => self.cpu.inc_de(),
      // 0x14 | INC D | 1 | 4 | Z 0 H -
      0x14 => op_inc_reg!(self, reg_d),
      // 0x15 | DEC D | 1 | 4 | Z 1 H -
      0x15 => op_dec_reg!(self, reg_d),
      // 0x16 | LD D,d8 | 2 | 8 | - - - -
      0x16 => load_word_to_reg!(reg_d, self),
      // 0x17 | RLA | 1 | 4 | 0 0 0 C
      0x17 => {
        let old_carry = if self.cpu.flag_carry() { 1 } else { 0 };
        self.cpu.set_flag_carry(bitn!(self.cpu.reg_a, 0x7));

        self.cpu.reg_a = (self.cpu.reg_a << 1) | old_carry;

        self.cpu.reset_flag_zero();
        self.cpu.reset_flag_add_sub();
        self.cpu.reset_flag_half_carry();
      }
      // 0x18 | JR r8 | 2 | 12 | - - - -
      0x18 => {
        let offs = self.read_opcode_word();
        let addr = Util::dword_signed_add(self.cpu.pc, offs as i8);
        self.cpu.pc = addr;
      }
      // 0x19 | ADD HL,DE | 1 | 8 | - 0 H C
      0x19 => unimplemented!("Opcode 0x19 is not yet implemented"),
      // 0x1a | LD A,(DE) | 1 | 8 | - - - -
      0x1a => load_word_to_reg_from_reg_addr!(reg_a, reg_d, reg_e, self),
      // 0x1b | DEC DE | 1 | 8 | - - - -
      0x1b => unimplemented!("Opcode 0x1b is not yet implemented"),
      // 0x1c | INC E | 1 | 4 | Z 0 H -
      0x1c => op_inc_reg!(self, reg_e),
      // 0x1d | DEC E | 1 | 4 | Z 1 H -
      0x1d => op_dec_reg!(self, reg_e),
      // 0x1e | LD E,d8 | 2 | 8 | - - - -
      0x1e => load_word_to_reg!(reg_e, self),
      // 0x1f | RRA | 1 | 4 | 0 0 0 C
      0x1f => unimplemented!("Opcode 0x1f is not yet implemented"),
      // 0x20 | JR NZ,r8 | 2 | 12/8 | - - - -
      0x20 => {
        let offs = self.read_opcode_word() as i8;
        if !self.cpu.flag_zero() {
          self.cpu.pc = (self.cpu.pc as i32 + offs as i32) as u16;
        } else {
          is_cycle_alternative = true;
        }
      }
      // 0x21 | LD HL,d16 | 3 | 12 | - - - -
      0x21 => load_dword_to_reg!(set_hl, self),
      // 0x22 | LD (HL+),A | 1 | 8 | - - - -
      0x22 => {
        load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_a, self);
        self.cpu.inc_hl();
      }
      // 0x23 | INC HL | 1 | 8 | - - - -
      0x23 => self.cpu.inc_hl(),
      // 0x24 | INC H | 1 | 4 | Z 0 H -
      0x24 => op_inc_reg!(self, reg_h),
      // 0x25 | DEC H | 1 | 4 | Z 1 H -
      0x25 => op_dec_reg!(self, reg_h),
      // 0x26 | LD H,d8 | 2 | 8 | - - - -
      0x26 => load_word_to_reg!(reg_h, self),
      // 0x27 | DAA | 1 | 4 | Z - 0 C
      0x27 => unimplemented!("Opcode 0x27 is not yet implemented"),
      // 0x28 | JR Z,r8 | 2 | 12/8 | - - - -
      0x28 => {
        let offs = self.read_opcode_word();
        if self.cpu.flag_zero() {
          self.cpu.pc += offs as u16;
        } else {
          is_cycle_alternative = true;
        }
      }
      // 0x29 | ADD HL,HL | 1 | 8 | - 0 H C
      0x29 => unimplemented!("Opcode 0x29 is not yet implemented"),
      // 0x2a | LD A,(HL+) | 1 | 8 | - - - -
      0x2a => unimplemented!("Opcode 0x2a is not yet implemented"),
      // 0x2b | DEC HL | 1 | 8 | - - - -
      0x2b => unimplemented!("Opcode 0x2b is not yet implemented"),
      // 0x2c | INC L | 1 | 4 | Z 0 H -
      0x2c => op_inc_reg!(self, reg_l),
      // 0x2d | DEC L | 1 | 4 | Z 1 H -
      0x2d => op_dec_reg!(self, reg_l),
      // 0x2e | LD L,d8 | 2 | 8 | - - - -
      0x2e => load_word_to_reg!(reg_l, self),
      // 0x2f | CPL | 1 | 4 | - 1 1 -
      0x2f => unimplemented!("Opcode 0x2f is not yet implemented"),
      // 0x30 | JR NC,r8 | 2 | 12/8 | - - - -
      0x30 => unimplemented!("Opcode 0x30 is not yet implemented"),
      // 0x31 | LD SP,d16 | 3 | 12 | - - - -
      0x31 => self.cpu.sp = self.read_opcode_dword(),
      // 0x32 | LD (HL-),A | 1 | 8 | - - - -
      0x32 => {
        load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_a, self);
        self.cpu.dec_hl();
      }
      // 0x33 | INC SP | 1 | 8 | - - - -
      0x33 => unimplemented!("Opcode 0x33 is not yet implemented"),
      // 0x34 | INC (HL) | 1 | 12 | Z 0 H -
      0x34 => unimplemented!("Opcode 0x34 is not yet implemented"),
      // 0x35 | DEC (HL) | 1 | 12 | Z 1 H -
      0x35 => unimplemented!("Opcode 0x35 is not yet implemented"),
      // 0x36 | LD (HL),d8 | 2 | 12 | - - - -
      0x36 => unimplemented!("Opcode 0x36 is not yet implemented"),
      // 0x37 | SCF | 1 | 4 | - 0 0 1
      0x37 => unimplemented!("Opcode 0x37 is not yet implemented"),
      // 0x38 | JR C,r8 | 2 | 12/8 | - - - -
      0x38 => unimplemented!("Opcode 0x38 is not yet implemented"),
      // 0x39 | ADD HL,SP | 1 | 8 | - 0 H C
      0x39 => unimplemented!("Opcode 0x39 is not yet implemented"),
      // 0x3a | LD A,(HL-) | 1 | 8 | - - - -
      0x3a => unimplemented!("Opcode 0x3a is not yet implemented"),
      // 0x3b | DEC SP | 1 | 8 | - - - -
      0x3b => unimplemented!("Opcode 0x3b is not yet implemented"),
      // 0x3c | INC A | 1 | 4 | Z 0 H -
      0x3c => op_inc_reg!(self, reg_a),
      // 0x3d | DEC A | 1 | 4 | Z 1 H -
      0x3d => op_dec_reg!(self, reg_a),
      // 0x3e | LD A,d8 | 2 | 8 | - - - -
      0x3e => load_word_to_reg!(reg_a, self),
      // 0x3f | CCF | 1 | 4 | - 0 0 C
      0x3f => unimplemented!("Opcode 0x3f is not yet implemented"),
      // 0x40 | LD B,B | 1 | 4 | - - - -
      0x40 => load_word_to_reg_from_reg!(reg_b, reg_b, self),
      // 0x41 | LD B,C | 1 | 4 | - - - -
      0x41 => load_word_to_reg_from_reg!(reg_b, reg_c, self),
      // 0x42 | LD B,D | 1 | 4 | - - - -
      0x42 => load_word_to_reg_from_reg!(reg_b, reg_d, self),
      // 0x43 | LD B,E | 1 | 4 | - - - -
      0x43 => load_word_to_reg_from_reg!(reg_b, reg_e, self),
      // 0x44 | LD B,H | 1 | 4 | - - - -
      0x44 => load_word_to_reg_from_reg!(reg_b, reg_h, self),
      // 0x45 | LD B,L | 1 | 4 | - - - -
      0x45 => load_word_to_reg_from_reg!(reg_b, reg_l, self),
      // 0x46 | LD B,(HL) | 1 | 8 | - - - -
      0x46 => load_word_to_reg_from_reg_addr!(reg_b, reg_h, reg_l, self),
      // 0x47 | LD B,A | 1 | 4 | - - - -
      0x47 => load_word_to_reg_from_reg!(reg_b, reg_a, self),
      // 0x48 | LD C,B | 1 | 4 | - - - -
      0x48 => load_word_to_reg_from_reg!(reg_c, reg_b, self),
      // 0x49 | LD C,C | 1 | 4 | - - - -
      0x49 => load_word_to_reg_from_reg!(reg_c, reg_c, self),
      // 0x4a | LD C,D | 1 | 4 | - - - -
      0x4a => load_word_to_reg_from_reg!(reg_c, reg_d, self),
      // 0x4b | LD C,E | 1 | 4 | - - - -
      0x4b => load_word_to_reg_from_reg!(reg_c, reg_e, self),
      // 0x4c | LD C,H | 1 | 4 | - - - -
      0x4c => load_word_to_reg_from_reg!(reg_c, reg_h, self),
      // 0x4d | LD C,L | 1 | 4 | - - - -
      0x4d => load_word_to_reg_from_reg!(reg_c, reg_l, self),
      // 0x4e | LD C,(HL) | 1 | 8 | - - - -
      0x4e => load_word_to_reg_from_reg_addr!(reg_c, reg_h, reg_l, self),
      // 0x4f | LD C,A | 1 | 4 | - - - -
      0x4f => load_word_to_reg_from_reg!(reg_c, reg_a, self),
      // 0x50 | LD D,B | 1 | 4 | - - - -
      0x50 => load_word_to_reg_from_reg!(reg_d, reg_b, self),
      // 0x51 | LD D,C | 1 | 4 | - - - -
      0x51 => load_word_to_reg_from_reg!(reg_d, reg_c, self),
      // 0x52 | LD D,D | 1 | 4 | - - - -
      0x52 => load_word_to_reg_from_reg!(reg_d, reg_d, self),
      // 0x53 | LD D,E | 1 | 4 | - - - -
      0x53 => load_word_to_reg_from_reg!(reg_d, reg_e, self),
      // 0x54 | LD D,H | 1 | 4 | - - - -
      0x54 => load_word_to_reg_from_reg!(reg_d, reg_h, self),
      // 0x55 | LD D,L | 1 | 4 | - - - -
      0x55 => load_word_to_reg_from_reg!(reg_d, reg_l, self),
      // 0x56 | LD D,(HL) | 1 | 8 | - - - -
      0x56 => load_word_to_reg_from_reg_addr!(reg_d, reg_h, reg_l, self),
      // 0x57 | LD D,A | 1 | 4 | - - - -
      0x57 => load_word_to_reg_from_reg!(reg_d, reg_a, self),
      // 0x58 | LD E,B | 1 | 4 | - - - -
      0x58 => load_word_to_reg_from_reg!(reg_e, reg_b, self),
      // 0x59 | LD E,C | 1 | 4 | - - - -
      0x59 => load_word_to_reg_from_reg!(reg_e, reg_c, self),
      // 0x5a | LD E,D | 1 | 4 | - - - -
      0x5a => load_word_to_reg_from_reg!(reg_e, reg_d, self),
      // 0x5b | LD E,E | 1 | 4 | - - - -
      0x5b => load_word_to_reg_from_reg!(reg_e, reg_e, self),
      // 0x5c | LD E,H | 1 | 4 | - - - -
      0x5c => load_word_to_reg_from_reg!(reg_e, reg_h, self),
      // 0x5d | LD E,L | 1 | 4 | - - - -
      0x5d => load_word_to_reg_from_reg!(reg_e, reg_l, self),
      // 0x5e | LD E,(HL) | 1 | 8 | - - - -
      0x5e => load_word_to_reg_from_reg_addr!(reg_e, reg_h, reg_l, self),
      // 0x5f | LD E,A | 1 | 4 | - - - -
      0x5f => load_word_to_reg_from_reg!(reg_e, reg_a, self),
      // 0x60 | LD H,B | 1 | 4 | - - - -
      0x60 => load_word_to_reg_from_reg!(reg_h, reg_b, self),
      // 0x61 | LD H,C | 1 | 4 | - - - -
      0x61 => load_word_to_reg_from_reg!(reg_h, reg_c, self),
      // 0x62 | LD H,D | 1 | 4 | - - - -
      0x62 => load_word_to_reg_from_reg!(reg_h, reg_d, self),
      // 0x63 | LD H,E | 1 | 4 | - - - -
      0x63 => load_word_to_reg_from_reg!(reg_h, reg_e, self),
      // 0x64 | LD H,H | 1 | 4 | - - - -
      0x64 => load_word_to_reg_from_reg!(reg_h, reg_h, self),
      // 0x65 | LD H,L | 1 | 4 | - - - -
      0x65 => load_word_to_reg_from_reg!(reg_h, reg_l, self),
      // 0x66 | LD H,(HL) | 1 | 8 | - - - -
      0x66 => load_word_to_reg_from_reg_addr!(reg_h, reg_h, reg_l, self),
      // 0x67 | LD H,A | 1 | 4 | - - - -
      0x67 => load_word_to_reg_from_reg!(reg_h, reg_a, self),
      // 0x68 | LD L,B | 1 | 4 | - - - -
      0x68 => load_word_to_reg_from_reg!(reg_l, reg_b, self),
      // 0x69 | LD L,C | 1 | 4 | - - - -
      0x69 => load_word_to_reg_from_reg!(reg_l, reg_c, self),
      // 0x6a | LD L,D | 1 | 4 | - - - -
      0x6a => load_word_to_reg_from_reg!(reg_l, reg_d, self),
      // 0x6b | LD L,E | 1 | 4 | - - - -
      0x6b => load_word_to_reg_from_reg!(reg_l, reg_e, self),
      // 0x6c | LD L,H | 1 | 4 | - - - -
      0x6c => load_word_to_reg_from_reg!(reg_l, reg_h, self),
      // 0x6d | LD L,L | 1 | 4 | - - - -
      0x6d => load_word_to_reg_from_reg!(reg_l, reg_l, self),
      // 0x6e | LD L,(HL) | 1 | 8 | - - - -
      0x6e => load_word_to_reg_from_reg_addr!(reg_l, reg_h, reg_l, self),
      // 0x6f | LD L,A | 1 | 4 | - - - -
      0x6f => load_word_to_reg_from_reg!(reg_l, reg_a, self),
      // 0x70 | LD (HL),B | 1 | 8 | - - - -
      0x70 => load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_b, self),
      // 0x71 | LD (HL),C | 1 | 8 | - - - -
      0x71 => load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_c, self),
      // 0x72 | LD (HL),D | 1 | 8 | - - - -
      0x72 => load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_d, self),
      // 0x73 | LD (HL),E | 1 | 8 | - - - -
      0x73 => load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_e, self),
      // 0x74 | LD (HL),H | 1 | 8 | - - - -
      0x74 => load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_h, self),
      // 0x75 | LD (HL),L | 1 | 8 | - - - -
      0x75 => load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_l, self),
      // 0x76 | HALT | 1 | 4 | - - - -
      0x76 => unimplemented!("Opcode 0x76 is not yet implemented"),
      // 0x77 | LD (HL),A | 1 | 8 | - - - -
      0x77 => load_word_to_reg_addr_from_reg!(reg_h, reg_l, reg_a, self),
      // 0x78 | LD A,B | 1 | 4 | - - - -
      0x78 => load_word_to_reg_from_reg!(reg_a, reg_b, self),
      // 0x79 | LD A,C | 1 | 4 | - - - -
      0x79 => load_word_to_reg_from_reg!(reg_a, reg_c, self),
      // 0x7a | LD A,D | 1 | 4 | - - - -
      0x7a => load_word_to_reg_from_reg!(reg_a, reg_d, self),
      // 0x7b | LD A,E | 1 | 4 | - - - -
      0x7b => load_word_to_reg_from_reg!(reg_a, reg_e, self),
      // 0x7c | LD A,H | 1 | 4 | - - - -
      0x7c => load_word_to_reg_from_reg!(reg_a, reg_h, self),
      // 0x7d | LD A,L | 1 | 4 | - - - -
      0x7d => load_word_to_reg_from_reg!(reg_a, reg_l, self),
      // 0x7e | LD A,(HL) | 1 | 8 | - - - -
      0x7e => load_word_to_reg_from_reg_addr!(reg_a, reg_h, reg_l, self),
      // 0x7f | LD A,A | 1 | 4 | - - - -
      0x7f => load_word_to_reg_from_reg!(reg_a, reg_a, self),
      // 0x80 | ADD A,B | 1 | 4 | Z 0 H C
      0x80 => unimplemented!("Opcode 0x80 is not yet implemented"),
      // 0x81 | ADD A,C | 1 | 4 | Z 0 H C
      0x81 => unimplemented!("Opcode 0x81 is not yet implemented"),
      // 0x82 | ADD A,D | 1 | 4 | Z 0 H C
      0x82 => unimplemented!("Opcode 0x82 is not yet implemented"),
      // 0x83 | ADD A,E | 1 | 4 | Z 0 H C
      0x83 => unimplemented!("Opcode 0x83 is not yet implemented"),
      // 0x84 | ADD A,H | 1 | 4 | Z 0 H C
      0x84 => unimplemented!("Opcode 0x84 is not yet implemented"),
      // 0x85 | ADD A,L | 1 | 4 | Z 0 H C
      0x85 => unimplemented!("Opcode 0x85 is not yet implemented"),
      // 0x86 | ADD A,(HL) | 1 | 8 | Z 0 H C
      0x86 => unimplemented!("Opcode 0x86 is not yet implemented"),
      // 0x87 | ADD A,A | 1 | 4 | Z 0 H C
      0x87 => unimplemented!("Opcode 0x87 is not yet implemented"),
      // 0x88 | ADC A,B | 1 | 4 | Z 0 H C
      0x88 => unimplemented!("Opcode 0x88 is not yet implemented"),
      // 0x89 | ADC A,C | 1 | 4 | Z 0 H C
      0x89 => unimplemented!("Opcode 0x89 is not yet implemented"),
      // 0x8a | ADC A,D | 1 | 4 | Z 0 H C
      0x8a => unimplemented!("Opcode 0x8a is not yet implemented"),
      // 0x8b | ADC A,E | 1 | 4 | Z 0 H C
      0x8b => unimplemented!("Opcode 0x8b is not yet implemented"),
      // 0x8c | ADC A,H | 1 | 4 | Z 0 H C
      0x8c => unimplemented!("Opcode 0x8c is not yet implemented"),
      // 0x8d | ADC A,L | 1 | 4 | Z 0 H C
      0x8d => unimplemented!("Opcode 0x8d is not yet implemented"),
      // 0x8e | ADC A,(HL) | 1 | 8 | Z 0 H C
      0x8e => unimplemented!("Opcode 0x8e is not yet implemented"),
      // 0x8f | ADC A,A | 1 | 4 | Z 0 H C
      0x8f => unimplemented!("Opcode 0x8f is not yet implemented"),
      // 0x90 | SUB B | 1 | 4 | Z 1 H C
      0x90 => op_sub_reg_from_a!(self, reg_b),
      // 0x91 | SUB C | 1 | 4 | Z 1 H C
      0x91 => op_sub_reg_from_a!(self, reg_c),
      // 0x92 | SUB D | 1 | 4 | Z 1 H C
      0x92 => op_sub_reg_from_a!(self, reg_d),
      // 0x93 | SUB E | 1 | 4 | Z 1 H C
      0x93 => op_sub_reg_from_a!(self, reg_e),
      // 0x94 | SUB H | 1 | 4 | Z 1 H C
      0x94 => op_sub_reg_from_a!(self, reg_h),
      // 0x95 | SUB L | 1 | 4 | Z 1 H C
      0x95 => op_sub_reg_from_a!(self, reg_l),
      // 0x96 | SUB (HL) | 1 | 8 | Z 1 H C
      0x96 => {
        let acc = self.read_word(self.cpu.reg_hl());
        if !Util::has_half_borrow(self.cpu.reg_a, acc) {
          self.cpu.set_flag_half_carry(0x1);
        }
        if !Util::has_borrow(self.cpu.reg_a, acc) {
          self.cpu.set_flag_carry(0x1);
        }

        self.cpu.reg_a = self.cpu.reg_a.wrapping_sub(acc);

        self.cpu.set_flag_zero_for(self.cpu.reg_a);
        self.cpu.set_flag_add_sub(0x1);
      }
      // 0x97 | SUB A | 1 | 4 | Z 1 H C
      0x97 => op_sub_reg_from_a!(self, reg_a),
      // 0x98 | SBC A,B | 1 | 4 | Z 1 H C
      0x98 => unimplemented!("Opcode 0x98 is not yet implemented"),
      // 0x99 | SBC A,C | 1 | 4 | Z 1 H C
      0x99 => unimplemented!("Opcode 0x99 is not yet implemented"),
      // 0x9a | SBC A,D | 1 | 4 | Z 1 H C
      0x9a => unimplemented!("Opcode 0x9a is not yet implemented"),
      // 0x9b | SBC A,E | 1 | 4 | Z 1 H C
      0x9b => unimplemented!("Opcode 0x9b is not yet implemented"),
      // 0x9c | SBC A,H | 1 | 4 | Z 1 H C
      0x9c => unimplemented!("Opcode 0x9c is not yet implemented"),
      // 0x9d | SBC A,L | 1 | 4 | Z 1 H C
      0x9d => unimplemented!("Opcode 0x9d is not yet implemented"),
      // 0x9e | SBC A,(HL) | 1 | 8 | Z 1 H C
      0x9e => unimplemented!("Opcode 0x9e is not yet implemented"),
      // 0x9f | SBC A,A | 1 | 4 | Z 1 H C
      0x9f => unimplemented!("Opcode 0x9f is not yet implemented"),
      // 0xa0 | AND B | 1 | 4 | Z 0 1 0
      0xa0 => unimplemented!("Opcode 0xa0 is not yet implemented"),
      // 0xa1 | AND C | 1 | 4 | Z 0 1 0
      0xa1 => unimplemented!("Opcode 0xa1 is not yet implemented"),
      // 0xa2 | AND D | 1 | 4 | Z 0 1 0
      0xa2 => unimplemented!("Opcode 0xa2 is not yet implemented"),
      // 0xa3 | AND E | 1 | 4 | Z 0 1 0
      0xa3 => unimplemented!("Opcode 0xa3 is not yet implemented"),
      // 0xa4 | AND H | 1 | 4 | Z 0 1 0
      0xa4 => unimplemented!("Opcode 0xa4 is not yet implemented"),
      // 0xa5 | AND L | 1 | 4 | Z 0 1 0
      0xa5 => unimplemented!("Opcode 0xa5 is not yet implemented"),
      // 0xa6 | AND (HL) | 1 | 8 | Z 0 1 0
      0xa6 => unimplemented!("Opcode 0xa6 is not yet implemented"),
      // 0xa7 | AND A | 1 | 4 | Z 0 1 0
      0xa7 => unimplemented!("Opcode 0xa7 is not yet implemented"),
      // 0xa8 | XOR B | 1 | 4 | Z 0 0 0
      0xa8 => xor_reg!(reg_b, self),
      // 0xa9 | XOR C | 1 | 4 | Z 0 0 0
      0xa9 => xor_reg!(reg_c, self),
      // 0xaa | XOR D | 1 | 4 | Z 0 0 0
      0xaa => xor_reg!(reg_d, self),
      // 0xab | XOR E | 1 | 4 | Z 0 0 0
      0xab => xor_reg!(reg_e, self),
      // 0xac | XOR H | 1 | 4 | Z 0 0 0
      0xac => xor_reg!(reg_h, self),
      // 0xad | XOR L | 1 | 4 | Z 0 0 0
      0xad => xor_reg!(reg_l, self),
      // 0xae | XOR (HL) | 1 | 8 | Z 0 0 0
      0xae => unimplemented!("Opcode 0xae is not yet implemented"),
      // 0xaf | XOR A | 1 | 4 | Z 0 0 0
      0xaf => xor_reg!(reg_a, self),
      // 0xb0 | OR B | 1 | 4 | Z 0 0 0
      0xb0 => unimplemented!("Opcode 0xb0 is not yet implemented"),
      // 0xb1 | OR C | 1 | 4 | Z 0 0 0
      0xb1 => unimplemented!("Opcode 0xb1 is not yet implemented"),
      // 0xb2 | OR D | 1 | 4 | Z 0 0 0
      0xb2 => unimplemented!("Opcode 0xb2 is not yet implemented"),
      // 0xb3 | OR E | 1 | 4 | Z 0 0 0
      0xb3 => unimplemented!("Opcode 0xb3 is not yet implemented"),
      // 0xb4 | OR H | 1 | 4 | Z 0 0 0
      0xb4 => unimplemented!("Opcode 0xb4 is not yet implemented"),
      // 0xb5 | OR L | 1 | 4 | Z 0 0 0
      0xb5 => unimplemented!("Opcode 0xb5 is not yet implemented"),
      // 0xb6 | OR (HL) | 1 | 8 | Z 0 0 0
      0xb6 => unimplemented!("Opcode 0xb6 is not yet implemented"),
      // 0xb7 | OR A | 1 | 4 | Z 0 0 0
      0xb7 => unimplemented!("Opcode 0xb7 is not yet implemented"),
      // 0xb8 | CP B | 1 | 4 | Z 1 H C
      0xb8 => unimplemented!("Opcode 0xb8 is not yet implemented"),
      // 0xb9 | CP C | 1 | 4 | Z 1 H C
      0xb9 => unimplemented!("Opcode 0xb9 is not yet implemented"),
      // 0xba | CP D | 1 | 4 | Z 1 H C
      0xba => unimplemented!("Opcode 0xba is not yet implemented"),
      // 0xbb | CP E | 1 | 4 | Z 1 H C
      0xbb => unimplemented!("Opcode 0xbb is not yet implemented"),
      // 0xbc | CP H | 1 | 4 | Z 1 H C
      0xbc => unimplemented!("Opcode 0xbc is not yet implemented"),
      // 0xbd | CP L | 1 | 4 | Z 1 H C
      0xbd => unimplemented!("Opcode 0xbd is not yet implemented"),
      // 0xbe | CP (HL) | 1 | 8 | Z 1 H C
      0xbe => unimplemented!("Opcode 0xbe is not yet implemented"),
      // 0xbf | CP A | 1 | 4 | Z 1 H C
      0xbf => unimplemented!("Opcode 0xbf is not yet implemented"),
      // 0xc0 | RET NZ | 1 | 20/8 | - - - -
      0xc0 => unimplemented!("Opcode 0xc0 is not yet implemented"),
      // 0xc1 | POP BC | 1 | 12 | - - - -
      0xc1 => {
        let dw = self.pop_dword();
        self.cpu.set_bc(dw);
      }
      // 0xc2 | JP NZ,a16 | 3 | 16/12 | - - - -
      0xc2 => unimplemented!("Opcode 0xc2 is not yet implemented"),
      // 0xc3 | JP a16 | 3 | 16 | - - - -
      0xc3 => unimplemented!("Opcode 0xc3 is not yet implemented"),
      // 0xc4 | CALL NZ,a16 | 3 | 24/12 | - - - -
      0xc4 => unimplemented!("Opcode 0xc4 is not yet implemented"),
      // 0xc5 | PUSH BC | 1 | 16 | - - - -
      0xc5 => self.push_dword(self.cpu.reg_bc()),
      // 0xc6 | ADD A,d8 | 2 | 8 | Z 0 H C
      0xc6 => unimplemented!("Opcode 0xc6 is not yet implemented"),
      // 0xc7 | RST 00H | 1 | 16 | - - - -
      0xc7 => unimplemented!("Opcode 0xc7 is not yet implemented"),
      // 0xc8 | RET Z | 1 | 20/8 | - - - -
      0xc8 => unimplemented!("Opcode 0xc8 is not yet implemented"),
      // 0xc9 | RET | 1 | 16 | - - - -
      0xc9 => self.cpu.pc = self.pop_dword(),
      // 0xca | JP Z,a16 | 3 | 16/12 | - - - -
      0xca => unimplemented!("Opcode 0xca is not yet implemented"),
      // 0xcb | PREFIX CB | 1 | 4 | - - - -
      0xcb => self.read_prefix_instruction(),
      // 0xcc | CALL Z,a16 | 3 | 24/12 | - - - -
      0xcc => unimplemented!("Opcode 0xcc is not yet implemented"),
      // 0xcd | CALL a16 | 3 | 24 | - - - -
      0xcd => {
        let addr = self.read_opcode_dword();
        self.push_dword(self.cpu.pc);
        self.cpu.pc = addr;
      }
      // 0xce | ADC A,d8 | 2 | 8 | Z 0 H C
      0xce => unimplemented!("Opcode 0xce is not yet implemented"),
      // 0xcf | RST 08H | 1 | 16 | - - - -
      0xcf => unimplemented!("Opcode 0xcf is not yet implemented"),
      // 0xd0 | RET NC | 1 | 20/8 | - - - -
      0xd0 => unimplemented!("Opcode 0xd0 is not yet implemented"),
      // 0xd1 | POP DE | 1 | 12 | - - - -
      0xd1 => unimplemented!("Opcode 0xd1 is not yet implemented"),
      // 0xd2 | JP NC,a16 | 3 | 16/12 | - - - -
      0xd2 => unimplemented!("Opcode 0xd2 is not yet implemented"),
      // 0xd4 | CALL NC,a16 | 3 | 24/12 | - - - -
      0xd4 => unimplemented!("Opcode 0xd4 is not yet implemented"),
      // 0xd5 | PUSH DE | 1 | 16 | - - - -
      0xd5 => self.push_dword(self.cpu.reg_de()),
      // 0xd6 | SUB d8 | 2 | 8 | Z 1 H C
      0xd6 => unimplemented!("Opcode 0xd6 is not yet implemented"),
      // 0xd7 | RST 10H | 1 | 16 | - - - -
      0xd7 => unimplemented!("Opcode 0xd7 is not yet implemented"),
      // 0xd8 | RET C | 1 | 20/8 | - - - -
      0xd8 => unimplemented!("Opcode 0xd8 is not yet implemented"),
      // 0xd9 | RETI | 1 | 16 | - - - -
      0xd9 => unimplemented!("Opcode 0xd9 is not yet implemented"),
      // 0xda | JP C,a16 | 3 | 16/12 | - - - -
      0xda => unimplemented!("Opcode 0xda is not yet implemented"),
      // 0xdc | CALL C,a16 | 3 | 24/12 | - - - -
      0xdc => unimplemented!("Opcode 0xdc is not yet implemented"),
      // 0xde | SBC A,d8 | 2 | 8 | Z 1 H C
      0xde => unimplemented!("Opcode 0xde is not yet implemented"),
      // 0xdf | RST 18H | 1 | 16 | - - - -
      0xdf => unimplemented!("Opcode 0xdf is not yet implemented"),
      // 0xe0 | LDH (a8),A | 2 | 12 | - - - -
      0xe0 => {
        let addr = 0xff00 | self.read_opcode_word() as u16;
        self.write_word(addr, self.cpu.reg_a);
      }
      // 0xe1 | POP HL | 1 | 12 | - - - -
      0xe1 => unimplemented!("Opcode 0xe1 is not yet implemented"),
      // 0xe2 | LD (C),A | 2 | 8 | - - - -
      0xe2 => {
        let addr = 0xff00 + self.cpu.reg_c as u16;
        self.write_word(addr, self.cpu.reg_a);
      }
      // 0xe5 | PUSH HL | 1 | 16 | - - - -
      0xe5 => self.push_dword(self.cpu.reg_hl()),
      // 0xe6 | AND d8 | 2 | 8 | Z 0 1 0
      0xe6 => unimplemented!("Opcode 0xe6 is not yet implemented"),
      // 0xe7 | RST 20H | 1 | 16 | - - - -
      0xe7 => unimplemented!("Opcode 0xe7 is not yet implemented"),
      // 0xe8 | ADD SP,r8 | 2 | 16 | 0 0 H C
      0xe8 => unimplemented!("Opcode 0xe8 is not yet implemented"),
      // 0xe9 | JP (HL) | 1 | 4 | - - - -
      0xe9 => unimplemented!("Opcode 0xe9 is not yet implemented"),
      // 0xea | LD (a16),A | 3 | 16 | - - - -
      0xea => {
        let addr = self.read_opcode_dword();
        self.write_word(addr, self.cpu.reg_a);
      }
      // 0xee | XOR d8 | 2 | 8 | Z 0 0 0
      0xee => unimplemented!("Opcode 0xee is not yet implemented"),
      // 0xef | RST 28H | 1 | 16 | - - - -
      0xef => unimplemented!("Opcode 0xef is not yet implemented"),
      // 0xf0 | LDH A,(a8) | 2 | 12 | - - - -
      0xf0 => {
        let addr = 0xff00 | self.read_opcode_word() as u16;
        self.write_word(addr, self.cpu.reg_a);
      }
      // 0xf1 | POP AF | 1 | 12 | Z N H C
      0xf1 => unimplemented!("Opcode 0xf1 is not yet implemented"),
      // 0xf2 | LD A,(C) | 2 | 8 | - - - -
      0xf2 => unimplemented!("Opcode 0xf2 is not yet implemented"),
      // 0xf3 | DI | 1 | 4 | - - - -
      0xf3 => unimplemented!("Opcode 0xf3 is not yet implemented"),
      // 0xf5 | PUSH AF | 1 | 16 | - - - -
      0xf5 => self.push_dword(self.cpu.reg_af()),
      // 0xf6 | OR d8 | 2 | 8 | Z 0 0 0
      0xf6 => unimplemented!("Opcode 0xf6 is not yet implemented"),
      // 0xf7 | RST 30H | 1 | 16 | - - - -
      0xf7 => unimplemented!("Opcode 0xf7 is not yet implemented"),
      // 0xf8 | LD HL,SP+r8 | 2 | 12 | 0 0 H C
      0xf8 => unimplemented!("Opcode 0xf8 is not yet implemented"),
      // 0xf9 | LD SP,HL | 1 | 8 | - - - -
      0xf9 => unimplemented!("Opcode 0xf9 is not yet implemented"),
      // 0xfa | LD A,(a16) | 3 | 16 | - - - -
      0xfa => unimplemented!("Opcode 0xfa is not yet implemented"),
      // 0xfb | EI | 1 | 4 | - - - -
      0xfb => unimplemented!("Opcode 0xfb is not yet implemented"),
      // 0xfe | CP d8 | 2 | 8 | Z 1 H C
      0xfe => {
        let acc = self.read_opcode_word();
        if !Util::has_half_borrow(self.cpu.reg_a, acc) {
          self.cpu.set_flag_half_carry(0x1);
        }
        if !Util::has_borrow(self.cpu.reg_a, acc) {
          self.cpu.set_flag_carry(0x1);
        }

        self
          .cpu
          .set_flag_zero(if self.cpu.reg_a == acc { 0x1 } else { 0x0 });
        self.cpu.set_flag_add_sub(0x1);
      }
      // 0xff | RST 38H | 1 | 16 | - - - -
      0xff => unimplemented!("Opcode 0xff is not yet implemented"),
      opcode @ _ => panic!("Unexpected opcode: {:?}", opcode),
    };

    if is_cycle_alternative {
      self.cycles += OPCODE_DUR_ALTERNATIVE[opcode as usize] as u64;
    } else {
      self.cycles += OPCODE_DUR[opcode as usize] as u64;
    }
  }

  fn read_prefix_instruction(&mut self) {
    let opcode = self.read_opcode_word();

    info!(
      "Load prefix opcode: 0x{:>02x} at PC: 0x{:>04x}",
      opcode,
      self.cpu.pc - 1
    );

    match opcode {
      // 0x00 | RLC B | 2 | 8 | Z 0 0 C
      0x00 => unimplemented!("Prefix opcode 0x00 is not yet implemented"),
      // 0x01 | RLC C | 2 | 8 | Z 0 0 C
      0x01 => unimplemented!("Prefix opcode 0x01 is not yet implemented"),
      // 0x02 | RLC D | 2 | 8 | Z 0 0 C
      0x02 => unimplemented!("Prefix opcode 0x02 is not yet implemented"),
      // 0x03 | RLC E | 2 | 8 | Z 0 0 C
      0x03 => unimplemented!("Prefix opcode 0x03 is not yet implemented"),
      // 0x04 | RLC H | 2 | 8 | Z 0 0 C
      0x04 => unimplemented!("Prefix opcode 0x04 is not yet implemented"),
      // 0x05 | RLC L | 2 | 8 | Z 0 0 C
      0x05 => unimplemented!("Prefix opcode 0x05 is not yet implemented"),
      // 0x06 | RLC (HL) | 2 | 16 | Z 0 0 C
      0x06 => unimplemented!("Prefix opcode 0x06 is not yet implemented"),
      // 0x07 | RLC A | 2 | 8 | Z 0 0 C
      0x07 => unimplemented!("Prefix opcode 0x07 is not yet implemented"),
      // 0x08 | RRC B | 2 | 8 | Z 0 0 C
      0x08 => unimplemented!("Prefix opcode 0x08 is not yet implemented"),
      // 0x09 | RRC C | 2 | 8 | Z 0 0 C
      0x09 => unimplemented!("Prefix opcode 0x09 is not yet implemented"),
      // 0x0a | RRC D | 2 | 8 | Z 0 0 C
      0x0a => unimplemented!("Prefix opcode 0x0a is not yet implemented"),
      // 0x0b | RRC E | 2 | 8 | Z 0 0 C
      0x0b => unimplemented!("Prefix opcode 0x0b is not yet implemented"),
      // 0x0c | RRC H | 2 | 8 | Z 0 0 C
      0x0c => unimplemented!("Prefix opcode 0x0c is not yet implemented"),
      // 0x0d | RRC L | 2 | 8 | Z 0 0 C
      0x0d => unimplemented!("Prefix opcode 0x0d is not yet implemented"),
      // 0x0e | RRC (HL) | 2 | 16 | Z 0 0 C
      0x0e => unimplemented!("Prefix opcode 0x0e is not yet implemented"),
      // 0x0f | RRC A | 2 | 8 | Z 0 0 C
      0x0f => unimplemented!("Prefix opcode 0x0f is not yet implemented"),
      // 0x10 | RL B | 2 | 8 | Z 0 0 C
      0x10 => rot_left_reg!(self, reg_b),
      // 0x11 | RL C | 2 | 8 | Z 0 0 C
      0x11 => rot_left_reg!(self, reg_c),
      // 0x12 | RL D | 2 | 8 | Z 0 0 C
      0x12 => rot_left_reg!(self, reg_d),
      // 0x13 | RL E | 2 | 8 | Z 0 0 C
      0x13 => rot_left_reg!(self, reg_e),
      // 0x14 | RL H | 2 | 8 | Z 0 0 C
      0x14 => rot_left_reg!(self, reg_h),
      // 0x15 | RL L | 2 | 8 | Z 0 0 C
      0x15 => rot_left_reg!(self, reg_l),
      // 0x16 | RL (HL) | 2 | 16 | Z 0 0 C
      0x16 => unimplemented!("Prefix opcode 0x16 is not yet implemented"),
      // 0x17 | RL A | 2 | 8 | Z 0 0 C
      0x17 => rot_left_reg!(self, reg_a),
      // 0x18 | RR B | 2 | 8 | Z 0 0 C
      0x18 => unimplemented!("Prefix opcode 0x18 is not yet implemented"),
      // 0x19 | RR C | 2 | 8 | Z 0 0 C
      0x19 => unimplemented!("Prefix opcode 0x19 is not yet implemented"),
      // 0x1a | RR D | 2 | 8 | Z 0 0 C
      0x1a => unimplemented!("Prefix opcode 0x1a is not yet implemented"),
      // 0x1b | RR E | 2 | 8 | Z 0 0 C
      0x1b => unimplemented!("Prefix opcode 0x1b is not yet implemented"),
      // 0x1c | RR H | 2 | 8 | Z 0 0 C
      0x1c => unimplemented!("Prefix opcode 0x1c is not yet implemented"),
      // 0x1d | RR L | 2 | 8 | Z 0 0 C
      0x1d => unimplemented!("Prefix opcode 0x1d is not yet implemented"),
      // 0x1e | RR (HL) | 2 | 16 | Z 0 0 C
      0x1e => unimplemented!("Prefix opcode 0x1e is not yet implemented"),
      // 0x1f | RR A | 2 | 8 | Z 0 0 C
      0x1f => unimplemented!("Prefix opcode 0x1f is not yet implemented"),
      // 0x20 | SLA B | 2 | 8 | Z 0 0 C
      0x20 => unimplemented!("Prefix opcode 0x20 is not yet implemented"),
      // 0x21 | SLA C | 2 | 8 | Z 0 0 C
      0x21 => unimplemented!("Prefix opcode 0x21 is not yet implemented"),
      // 0x22 | SLA D | 2 | 8 | Z 0 0 C
      0x22 => unimplemented!("Prefix opcode 0x22 is not yet implemented"),
      // 0x23 | SLA E | 2 | 8 | Z 0 0 C
      0x23 => unimplemented!("Prefix opcode 0x23 is not yet implemented"),
      // 0x24 | SLA H | 2 | 8 | Z 0 0 C
      0x24 => unimplemented!("Prefix opcode 0x24 is not yet implemented"),
      // 0x25 | SLA L | 2 | 8 | Z 0 0 C
      0x25 => unimplemented!("Prefix opcode 0x25 is not yet implemented"),
      // 0x26 | SLA (HL) | 2 | 16 | Z 0 0 C
      0x26 => unimplemented!("Prefix opcode 0x26 is not yet implemented"),
      // 0x27 | SLA A | 2 | 8 | Z 0 0 C
      0x27 => unimplemented!("Prefix opcode 0x27 is not yet implemented"),
      // 0x28 | SRA B | 2 | 8 | Z 0 0 0
      0x28 => unimplemented!("Prefix opcode 0x28 is not yet implemented"),
      // 0x29 | SRA C | 2 | 8 | Z 0 0 0
      0x29 => unimplemented!("Prefix opcode 0x29 is not yet implemented"),
      // 0x2a | SRA D | 2 | 8 | Z 0 0 0
      0x2a => unimplemented!("Prefix opcode 0x2a is not yet implemented"),
      // 0x2b | SRA E | 2 | 8 | Z 0 0 0
      0x2b => unimplemented!("Prefix opcode 0x2b is not yet implemented"),
      // 0x2c | SRA H | 2 | 8 | Z 0 0 0
      0x2c => unimplemented!("Prefix opcode 0x2c is not yet implemented"),
      // 0x2d | SRA L | 2 | 8 | Z 0 0 0
      0x2d => unimplemented!("Prefix opcode 0x2d is not yet implemented"),
      // 0x2e | SRA (HL) | 2 | 16 | Z 0 0 0
      0x2e => unimplemented!("Prefix opcode 0x2e is not yet implemented"),
      // 0x2f | SRA A | 2 | 8 | Z 0 0 0
      0x2f => unimplemented!("Prefix opcode 0x2f is not yet implemented"),
      // 0x30 | SWAP B | 2 | 8 | Z 0 0 0
      0x30 => unimplemented!("Prefix opcode 0x30 is not yet implemented"),
      // 0x31 | SWAP C | 2 | 8 | Z 0 0 0
      0x31 => unimplemented!("Prefix opcode 0x31 is not yet implemented"),
      // 0x32 | SWAP D | 2 | 8 | Z 0 0 0
      0x32 => unimplemented!("Prefix opcode 0x32 is not yet implemented"),
      // 0x33 | SWAP E | 2 | 8 | Z 0 0 0
      0x33 => unimplemented!("Prefix opcode 0x33 is not yet implemented"),
      // 0x34 | SWAP H | 2 | 8 | Z 0 0 0
      0x34 => unimplemented!("Prefix opcode 0x34 is not yet implemented"),
      // 0x35 | SWAP L | 2 | 8 | Z 0 0 0
      0x35 => unimplemented!("Prefix opcode 0x35 is not yet implemented"),
      // 0x36 | SWAP (HL) | 2 | 16 | Z 0 0 0
      0x36 => unimplemented!("Prefix opcode 0x36 is not yet implemented"),
      // 0x37 | SWAP A | 2 | 8 | Z 0 0 0
      0x37 => unimplemented!("Prefix opcode 0x37 is not yet implemented"),
      // 0x38 | SRL B | 2 | 8 | Z 0 0 C
      0x38 => unimplemented!("Prefix opcode 0x38 is not yet implemented"),
      // 0x39 | SRL C | 2 | 8 | Z 0 0 C
      0x39 => unimplemented!("Prefix opcode 0x39 is not yet implemented"),
      // 0x3a | SRL D | 2 | 8 | Z 0 0 C
      0x3a => unimplemented!("Prefix opcode 0x3a is not yet implemented"),
      // 0x3b | SRL E | 2 | 8 | Z 0 0 C
      0x3b => unimplemented!("Prefix opcode 0x3b is not yet implemented"),
      // 0x3c | SRL H | 2 | 8 | Z 0 0 C
      0x3c => unimplemented!("Prefix opcode 0x3c is not yet implemented"),
      // 0x3d | SRL L | 2 | 8 | Z 0 0 C
      0x3d => unimplemented!("Prefix opcode 0x3d is not yet implemented"),
      // 0x3e | SRL (HL) | 2 | 16 | Z 0 0 C
      0x3e => unimplemented!("Prefix opcode 0x3e is not yet implemented"),
      // 0x3f | SRL A | 2 | 8 | Z 0 0 C
      0x3f => unimplemented!("Prefix opcode 0x3f is not yet implemented"),
      // 0x40 | BIT 0,B | 2 | 8 | Z 0 1 -
      0x40 => op_bit_test!(self, reg_b, 0),
      // 0x41 | BIT 0,C | 2 | 8 | Z 0 1 -
      0x41 => op_bit_test!(self, reg_c, 0),
      // 0x42 | BIT 0,D | 2 | 8 | Z 0 1 -
      0x42 => op_bit_test!(self, reg_d, 0),
      // 0x43 | BIT 0,E | 2 | 8 | Z 0 1 -
      0x43 => op_bit_test!(self, reg_e, 0),
      // 0x44 | BIT 0,H | 2 | 8 | Z 0 1 -
      0x44 => op_bit_test!(self, reg_h, 0),
      // 0x45 | BIT 0,L | 2 | 8 | Z 0 1 -
      0x45 => op_bit_test!(self, reg_l, 0),
      // 0x46 | BIT 0,(HL) | 2 | 16 | Z 0 1 -
      0x46 => unimplemented!("Prefix opcode 0x46 is not yet implemented"),
      // 0x47 | BIT 0,A | 2 | 8 | Z 0 1 -
      0x47 => op_bit_test!(self, reg_a, 0),
      // 0x48 | BIT 1,B | 2 | 8 | Z 0 1 -
      0x48 => op_bit_test!(self, reg_b, 1),
      // 0x49 | BIT 1,C | 2 | 8 | Z 0 1 -
      0x49 => op_bit_test!(self, reg_c, 1),
      // 0x4a | BIT 1,D | 2 | 8 | Z 0 1 -
      0x4a => op_bit_test!(self, reg_d, 1),
      // 0x4b | BIT 1,E | 2 | 8 | Z 0 1 -
      0x4b => op_bit_test!(self, reg_e, 1),
      // 0x4c | BIT 1,H | 2 | 8 | Z 0 1 -
      0x4c => op_bit_test!(self, reg_h, 1),
      // 0x4d | BIT 1,L | 2 | 8 | Z 0 1 -
      0x4d => op_bit_test!(self, reg_l, 1),
      // 0x4e | BIT 1,(HL) | 2 | 16 | Z 0 1 -
      0x4e => unimplemented!("Prefix opcode 0x4e is not yet implemented"),
      // 0x4f | BIT 1,A | 2 | 8 | Z 0 1 -
      0x4f => op_bit_test!(self, reg_a, 1),
      // 0x50 | BIT 2,B | 2 | 8 | Z 0 1 -
      0x50 => op_bit_test!(self, reg_b, 2),
      // 0x51 | BIT 2,C | 2 | 8 | Z 0 1 -
      0x51 => op_bit_test!(self, reg_c, 2),
      // 0x52 | BIT 2,D | 2 | 8 | Z 0 1 -
      0x52 => op_bit_test!(self, reg_d, 2),
      // 0x53 | BIT 2,E | 2 | 8 | Z 0 1 -
      0x53 => op_bit_test!(self, reg_e, 2),
      // 0x54 | BIT 2,H | 2 | 8 | Z 0 1 -
      0x54 => op_bit_test!(self, reg_h, 2),
      // 0x55 | BIT 2,L | 2 | 8 | Z 0 1 -
      0x55 => op_bit_test!(self, reg_l, 2),
      // 0x56 | BIT 2,(HL) | 2 | 16 | Z 0 1 -
      0x56 => unimplemented!("Prefix opcode 0x56 is not yet implemented"),
      // 0x57 | BIT 2,A | 2 | 8 | Z 0 1 -
      0x57 => op_bit_test!(self, reg_a, 2),
      // 0x58 | BIT 3,B | 2 | 8 | Z 0 1 -
      0x58 => op_bit_test!(self, reg_b, 3),
      // 0x59 | BIT 3,C | 2 | 8 | Z 0 1 -
      0x59 => op_bit_test!(self, reg_c, 3),
      // 0x5a | BIT 3,D | 2 | 8 | Z 0 1 -
      0x5a => op_bit_test!(self, reg_d, 3),
      // 0x5b | BIT 3,E | 2 | 8 | Z 0 1 -
      0x5b => op_bit_test!(self, reg_e, 3),
      // 0x5c | BIT 3,H | 2 | 8 | Z 0 1 -
      0x5c => op_bit_test!(self, reg_h, 3),
      // 0x5d | BIT 3,L | 2 | 8 | Z 0 1 -
      0x5d => op_bit_test!(self, reg_l, 3),
      // 0x5e | BIT 3,(HL) | 2 | 16 | Z 0 1 -
      0x5e => unimplemented!("Prefix opcode 0x5e is not yet implemented"),
      // 0x5f | BIT 3,A | 2 | 8 | Z 0 1 -
      0x5f => op_bit_test!(self, reg_a, 3),
      // 0x60 | BIT 4,B | 2 | 8 | Z 0 1 -
      0x60 => op_bit_test!(self, reg_b, 4),
      // 0x61 | BIT 4,C | 2 | 8 | Z 0 1 -
      0x61 => op_bit_test!(self, reg_c, 4),
      // 0x62 | BIT 4,D | 2 | 8 | Z 0 1 -
      0x62 => op_bit_test!(self, reg_d, 4),
      // 0x63 | BIT 4,E | 2 | 8 | Z 0 1 -
      0x63 => op_bit_test!(self, reg_e, 4),
      // 0x64 | BIT 4,H | 2 | 8 | Z 0 1 -
      0x64 => op_bit_test!(self, reg_h, 4),
      // 0x65 | BIT 4,L | 2 | 8 | Z 0 1 -
      0x65 => op_bit_test!(self, reg_l, 4),
      // 0x66 | BIT 4,(HL) | 2 | 16 | Z 0 1 -
      0x66 => unimplemented!("Prefix opcode 0x66 is not yet implemented"),
      // 0x67 | BIT 4,A | 2 | 8 | Z 0 1 -
      0x67 => op_bit_test!(self, reg_a, 4),
      // 0x68 | BIT 5,B | 2 | 8 | Z 0 1 -
      0x68 => op_bit_test!(self, reg_b, 5),
      // 0x69 | BIT 5,C | 2 | 8 | Z 0 1 -
      0x69 => op_bit_test!(self, reg_c, 5),
      // 0x6a | BIT 5,D | 2 | 8 | Z 0 1 -
      0x6a => op_bit_test!(self, reg_d, 5),
      // 0x6b | BIT 5,E | 2 | 8 | Z 0 1 -
      0x6b => op_bit_test!(self, reg_e, 5),
      // 0x6c | BIT 5,H | 2 | 8 | Z 0 1 -
      0x6c => op_bit_test!(self, reg_h, 5),
      // 0x6d | BIT 5,L | 2 | 8 | Z 0 1 -
      0x6d => op_bit_test!(self, reg_l, 5),
      // 0x6e | BIT 5,(HL) | 2 | 16 | Z 0 1 -
      0x6e => unimplemented!("Prefix opcode 0x6e is not yet implemented"),
      // 0x6f | BIT 5,A | 2 | 8 | Z 0 1 -
      0x6f => op_bit_test!(self, reg_a, 5),
      // 0x70 | BIT 6,B | 2 | 8 | Z 0 1 -
      0x70 => op_bit_test!(self, reg_b, 6),
      // 0x71 | BIT 6,C | 2 | 8 | Z 0 1 -
      0x71 => op_bit_test!(self, reg_c, 6),
      // 0x72 | BIT 6,D | 2 | 8 | Z 0 1 -
      0x72 => op_bit_test!(self, reg_d, 6),
      // 0x73 | BIT 6,E | 2 | 8 | Z 0 1 -
      0x73 => op_bit_test!(self, reg_e, 6),
      // 0x74 | BIT 6,H | 2 | 8 | Z 0 1 -
      0x74 => op_bit_test!(self, reg_h, 6),
      // 0x75 | BIT 6,L | 2 | 8 | Z 0 1 -
      0x75 => op_bit_test!(self, reg_l, 6),
      // 0x76 | BIT 6,(HL) | 2 | 16 | Z 0 1 -
      0x76 => unimplemented!("Prefix opcode 0x76 is not yet implemented"),
      // 0x77 | BIT 6,A | 2 | 8 | Z 0 1 -
      0x77 => op_bit_test!(self, reg_a, 6),
      // 0x78 | BIT 7,B | 2 | 8 | Z 0 1 -
      0x78 => op_bit_test!(self, reg_b, 7),
      // 0x79 | BIT 7,C | 2 | 8 | Z 0 1 -
      0x79 => op_bit_test!(self, reg_c, 7),
      // 0x7a | BIT 7,D | 2 | 8 | Z 0 1 -
      0x7a => op_bit_test!(self, reg_d, 7),
      // 0x7b | BIT 7,E | 2 | 8 | Z 0 1 -
      0x7b => op_bit_test!(self, reg_e, 7),
      // 0x7c | BIT 7,H | 2 | 8 | Z 0 1 -
      0x7c => op_bit_test!(self, reg_h, 7),
      // 0x7d | BIT 7,L | 2 | 8 | Z 0 1 -
      0x7d => op_bit_test!(self, reg_l, 7),
      // 0x7e | BIT 7,(HL) | 2 | 16 | Z 0 1 -
      0x7e => unimplemented!("Prefix opcode 0x7e is not yet implemented"),
      // 0x7f | BIT 7,A | 2 | 8 | Z 0 1 -
      0x7f => op_bit_test!(self, reg_a, 7),
      // 0x80 | RES 0,B | 2 | 8 | - - - -
      0x80 => unimplemented!("Prefix opcode 0x80 is not yet implemented"),
      // 0x81 | RES 0,C | 2 | 8 | - - - -
      0x81 => unimplemented!("Prefix opcode 0x81 is not yet implemented"),
      // 0x82 | RES 0,D | 2 | 8 | - - - -
      0x82 => unimplemented!("Prefix opcode 0x82 is not yet implemented"),
      // 0x83 | RES 0,E | 2 | 8 | - - - -
      0x83 => unimplemented!("Prefix opcode 0x83 is not yet implemented"),
      // 0x84 | RES 0,H | 2 | 8 | - - - -
      0x84 => unimplemented!("Prefix opcode 0x84 is not yet implemented"),
      // 0x85 | RES 0,L | 2 | 8 | - - - -
      0x85 => unimplemented!("Prefix opcode 0x85 is not yet implemented"),
      // 0x86 | RES 0,(HL) | 2 | 16 | - - - -
      0x86 => unimplemented!("Prefix opcode 0x86 is not yet implemented"),
      // 0x87 | RES 0,A | 2 | 8 | - - - -
      0x87 => unimplemented!("Prefix opcode 0x87 is not yet implemented"),
      // 0x88 | RES 1,B | 2 | 8 | - - - -
      0x88 => unimplemented!("Prefix opcode 0x88 is not yet implemented"),
      // 0x89 | RES 1,C | 2 | 8 | - - - -
      0x89 => unimplemented!("Prefix opcode 0x89 is not yet implemented"),
      // 0x8a | RES 1,D | 2 | 8 | - - - -
      0x8a => unimplemented!("Prefix opcode 0x8a is not yet implemented"),
      // 0x8b | RES 1,E | 2 | 8 | - - - -
      0x8b => unimplemented!("Prefix opcode 0x8b is not yet implemented"),
      // 0x8c | RES 1,H | 2 | 8 | - - - -
      0x8c => unimplemented!("Prefix opcode 0x8c is not yet implemented"),
      // 0x8d | RES 1,L | 2 | 8 | - - - -
      0x8d => unimplemented!("Prefix opcode 0x8d is not yet implemented"),
      // 0x8e | RES 1,(HL) | 2 | 16 | - - - -
      0x8e => unimplemented!("Prefix opcode 0x8e is not yet implemented"),
      // 0x8f | RES 1,A | 2 | 8 | - - - -
      0x8f => unimplemented!("Prefix opcode 0x8f is not yet implemented"),
      // 0x90 | RES 2,B | 2 | 8 | - - - -
      0x90 => unimplemented!("Prefix opcode 0x90 is not yet implemented"),
      // 0x91 | RES 2,C | 2 | 8 | - - - -
      0x91 => unimplemented!("Prefix opcode 0x91 is not yet implemented"),
      // 0x92 | RES 2,D | 2 | 8 | - - - -
      0x92 => unimplemented!("Prefix opcode 0x92 is not yet implemented"),
      // 0x93 | RES 2,E | 2 | 8 | - - - -
      0x93 => unimplemented!("Prefix opcode 0x93 is not yet implemented"),
      // 0x94 | RES 2,H | 2 | 8 | - - - -
      0x94 => unimplemented!("Prefix opcode 0x94 is not yet implemented"),
      // 0x95 | RES 2,L | 2 | 8 | - - - -
      0x95 => unimplemented!("Prefix opcode 0x95 is not yet implemented"),
      // 0x96 | RES 2,(HL) | 2 | 16 | - - - -
      0x96 => unimplemented!("Prefix opcode 0x96 is not yet implemented"),
      // 0x97 | RES 2,A | 2 | 8 | - - - -
      0x97 => unimplemented!("Prefix opcode 0x97 is not yet implemented"),
      // 0x98 | RES 3,B | 2 | 8 | - - - -
      0x98 => unimplemented!("Prefix opcode 0x98 is not yet implemented"),
      // 0x99 | RES 3,C | 2 | 8 | - - - -
      0x99 => unimplemented!("Prefix opcode 0x99 is not yet implemented"),
      // 0x9a | RES 3,D | 2 | 8 | - - - -
      0x9a => unimplemented!("Prefix opcode 0x9a is not yet implemented"),
      // 0x9b | RES 3,E | 2 | 8 | - - - -
      0x9b => unimplemented!("Prefix opcode 0x9b is not yet implemented"),
      // 0x9c | RES 3,H | 2 | 8 | - - - -
      0x9c => unimplemented!("Prefix opcode 0x9c is not yet implemented"),
      // 0x9d | RES 3,L | 2 | 8 | - - - -
      0x9d => unimplemented!("Prefix opcode 0x9d is not yet implemented"),
      // 0x9e | RES 3,(HL) | 2 | 16 | - - - -
      0x9e => unimplemented!("Prefix opcode 0x9e is not yet implemented"),
      // 0x9f | RES 3,A | 2 | 8 | - - - -
      0x9f => unimplemented!("Prefix opcode 0x9f is not yet implemented"),
      // 0xa0 | RES 4,B | 2 | 8 | - - - -
      0xa0 => unimplemented!("Prefix opcode 0xa0 is not yet implemented"),
      // 0xa1 | RES 4,C | 2 | 8 | - - - -
      0xa1 => unimplemented!("Prefix opcode 0xa1 is not yet implemented"),
      // 0xa2 | RES 4,D | 2 | 8 | - - - -
      0xa2 => unimplemented!("Prefix opcode 0xa2 is not yet implemented"),
      // 0xa3 | RES 4,E | 2 | 8 | - - - -
      0xa3 => unimplemented!("Prefix opcode 0xa3 is not yet implemented"),
      // 0xa4 | RES 4,H | 2 | 8 | - - - -
      0xa4 => unimplemented!("Prefix opcode 0xa4 is not yet implemented"),
      // 0xa5 | RES 4,L | 2 | 8 | - - - -
      0xa5 => unimplemented!("Prefix opcode 0xa5 is not yet implemented"),
      // 0xa6 | RES 4,(HL) | 2 | 16 | - - - -
      0xa6 => unimplemented!("Prefix opcode 0xa6 is not yet implemented"),
      // 0xa7 | RES 4,A | 2 | 8 | - - - -
      0xa7 => unimplemented!("Prefix opcode 0xa7 is not yet implemented"),
      // 0xa8 | RES 5,B | 2 | 8 | - - - -
      0xa8 => unimplemented!("Prefix opcode 0xa8 is not yet implemented"),
      // 0xa9 | RES 5,C | 2 | 8 | - - - -
      0xa9 => unimplemented!("Prefix opcode 0xa9 is not yet implemented"),
      // 0xaa | RES 5,D | 2 | 8 | - - - -
      0xaa => unimplemented!("Prefix opcode 0xaa is not yet implemented"),
      // 0xab | RES 5,E | 2 | 8 | - - - -
      0xab => unimplemented!("Prefix opcode 0xab is not yet implemented"),
      // 0xac | RES 5,H | 2 | 8 | - - - -
      0xac => unimplemented!("Prefix opcode 0xac is not yet implemented"),
      // 0xad | RES 5,L | 2 | 8 | - - - -
      0xad => unimplemented!("Prefix opcode 0xad is not yet implemented"),
      // 0xae | RES 5,(HL) | 2 | 16 | - - - -
      0xae => unimplemented!("Prefix opcode 0xae is not yet implemented"),
      // 0xaf | RES 5,A | 2 | 8 | - - - -
      0xaf => unimplemented!("Prefix opcode 0xaf is not yet implemented"),
      // 0xb0 | RES 6,B | 2 | 8 | - - - -
      0xb0 => unimplemented!("Prefix opcode 0xb0 is not yet implemented"),
      // 0xb1 | RES 6,C | 2 | 8 | - - - -
      0xb1 => unimplemented!("Prefix opcode 0xb1 is not yet implemented"),
      // 0xb2 | RES 6,D | 2 | 8 | - - - -
      0xb2 => unimplemented!("Prefix opcode 0xb2 is not yet implemented"),
      // 0xb3 | RES 6,E | 2 | 8 | - - - -
      0xb3 => unimplemented!("Prefix opcode 0xb3 is not yet implemented"),
      // 0xb4 | RES 6,H | 2 | 8 | - - - -
      0xb4 => unimplemented!("Prefix opcode 0xb4 is not yet implemented"),
      // 0xb5 | RES 6,L | 2 | 8 | - - - -
      0xb5 => unimplemented!("Prefix opcode 0xb5 is not yet implemented"),
      // 0xb6 | RES 6,(HL) | 2 | 16 | - - - -
      0xb6 => unimplemented!("Prefix opcode 0xb6 is not yet implemented"),
      // 0xb7 | RES 6,A | 2 | 8 | - - - -
      0xb7 => unimplemented!("Prefix opcode 0xb7 is not yet implemented"),
      // 0xb8 | RES 7,B | 2 | 8 | - - - -
      0xb8 => unimplemented!("Prefix opcode 0xb8 is not yet implemented"),
      // 0xb9 | RES 7,C | 2 | 8 | - - - -
      0xb9 => unimplemented!("Prefix opcode 0xb9 is not yet implemented"),
      // 0xba | RES 7,D | 2 | 8 | - - - -
      0xba => unimplemented!("Prefix opcode 0xba is not yet implemented"),
      // 0xbb | RES 7,E | 2 | 8 | - - - -
      0xbb => unimplemented!("Prefix opcode 0xbb is not yet implemented"),
      // 0xbc | RES 7,H | 2 | 8 | - - - -
      0xbc => unimplemented!("Prefix opcode 0xbc is not yet implemented"),
      // 0xbd | RES 7,L | 2 | 8 | - - - -
      0xbd => unimplemented!("Prefix opcode 0xbd is not yet implemented"),
      // 0xbe | RES 7,(HL) | 2 | 16 | - - - -
      0xbe => unimplemented!("Prefix opcode 0xbe is not yet implemented"),
      // 0xbf | RES 7,A | 2 | 8 | - - - -
      0xbf => unimplemented!("Prefix opcode 0xbf is not yet implemented"),
      // 0xc0 | SET 0,B | 2 | 8 | - - - -
      0xc0 => unimplemented!("Prefix opcode 0xc0 is not yet implemented"),
      // 0xc1 | SET 0,C | 2 | 8 | - - - -
      0xc1 => unimplemented!("Prefix opcode 0xc1 is not yet implemented"),
      // 0xc2 | SET 0,D | 2 | 8 | - - - -
      0xc2 => unimplemented!("Prefix opcode 0xc2 is not yet implemented"),
      // 0xc3 | SET 0,E | 2 | 8 | - - - -
      0xc3 => unimplemented!("Prefix opcode 0xc3 is not yet implemented"),
      // 0xc4 | SET 0,H | 2 | 8 | - - - -
      0xc4 => unimplemented!("Prefix opcode 0xc4 is not yet implemented"),
      // 0xc5 | SET 0,L | 2 | 8 | - - - -
      0xc5 => unimplemented!("Prefix opcode 0xc5 is not yet implemented"),
      // 0xc6 | SET 0,(HL) | 2 | 16 | - - - -
      0xc6 => unimplemented!("Prefix opcode 0xc6 is not yet implemented"),
      // 0xc7 | SET 0,A | 2 | 8 | - - - -
      0xc7 => unimplemented!("Prefix opcode 0xc7 is not yet implemented"),
      // 0xc8 | SET 1,B | 2 | 8 | - - - -
      0xc8 => unimplemented!("Prefix opcode 0xc8 is not yet implemented"),
      // 0xc9 | SET 1,C | 2 | 8 | - - - -
      0xc9 => unimplemented!("Prefix opcode 0xc9 is not yet implemented"),
      // 0xca | SET 1,D | 2 | 8 | - - - -
      0xca => unimplemented!("Prefix opcode 0xca is not yet implemented"),
      // 0xcb | SET 1,E | 2 | 8 | - - - -
      0xcb => unimplemented!("Prefix opcode 0xcb is not yet implemented"),
      // 0xcc | SET 1,H | 2 | 8 | - - - -
      0xcc => unimplemented!("Prefix opcode 0xcc is not yet implemented"),
      // 0xcd | SET 1,L | 2 | 8 | - - - -
      0xcd => unimplemented!("Prefix opcode 0xcd is not yet implemented"),
      // 0xce | SET 1,(HL) | 2 | 16 | - - - -
      0xce => unimplemented!("Prefix opcode 0xce is not yet implemented"),
      // 0xcf | SET 1,A | 2 | 8 | - - - -
      0xcf => unimplemented!("Prefix opcode 0xcf is not yet implemented"),
      // 0xd0 | SET 2,B | 2 | 8 | - - - -
      0xd0 => unimplemented!("Prefix opcode 0xd0 is not yet implemented"),
      // 0xd1 | SET 2,C | 2 | 8 | - - - -
      0xd1 => unimplemented!("Prefix opcode 0xd1 is not yet implemented"),
      // 0xd2 | SET 2,D | 2 | 8 | - - - -
      0xd2 => unimplemented!("Prefix opcode 0xd2 is not yet implemented"),
      // 0xd3 | SET 2,E | 2 | 8 | - - - -
      0xd3 => unimplemented!("Prefix opcode 0xd3 is not yet implemented"),
      // 0xd4 | SET 2,H | 2 | 8 | - - - -
      0xd4 => unimplemented!("Prefix opcode 0xd4 is not yet implemented"),
      // 0xd5 | SET 2,L | 2 | 8 | - - - -
      0xd5 => unimplemented!("Prefix opcode 0xd5 is not yet implemented"),
      // 0xd6 | SET 2,(HL) | 2 | 16 | - - - -
      0xd6 => unimplemented!("Prefix opcode 0xd6 is not yet implemented"),
      // 0xd7 | SET 2,A | 2 | 8 | - - - -
      0xd7 => unimplemented!("Prefix opcode 0xd7 is not yet implemented"),
      // 0xd8 | SET 3,B | 2 | 8 | - - - -
      0xd8 => unimplemented!("Prefix opcode 0xd8 is not yet implemented"),
      // 0xd9 | SET 3,C | 2 | 8 | - - - -
      0xd9 => unimplemented!("Prefix opcode 0xd9 is not yet implemented"),
      // 0xda | SET 3,D | 2 | 8 | - - - -
      0xda => unimplemented!("Prefix opcode 0xda is not yet implemented"),
      // 0xdb | SET 3,E | 2 | 8 | - - - -
      0xdb => unimplemented!("Prefix opcode 0xdb is not yet implemented"),
      // 0xdc | SET 3,H | 2 | 8 | - - - -
      0xdc => unimplemented!("Prefix opcode 0xdc is not yet implemented"),
      // 0xdd | SET 3,L | 2 | 8 | - - - -
      0xdd => unimplemented!("Prefix opcode 0xdd is not yet implemented"),
      // 0xde | SET 3,(HL) | 2 | 16 | - - - -
      0xde => unimplemented!("Prefix opcode 0xde is not yet implemented"),
      // 0xdf | SET 3,A | 2 | 8 | - - - -
      0xdf => unimplemented!("Prefix opcode 0xdf is not yet implemented"),
      // 0xe0 | SET 4,B | 2 | 8 | - - - -
      0xe0 => unimplemented!("Prefix opcode 0xe0 is not yet implemented"),
      // 0xe1 | SET 4,C | 2 | 8 | - - - -
      0xe1 => unimplemented!("Prefix opcode 0xe1 is not yet implemented"),
      // 0xe2 | SET 4,D | 2 | 8 | - - - -
      0xe2 => unimplemented!("Prefix opcode 0xe2 is not yet implemented"),
      // 0xe3 | SET 4,E | 2 | 8 | - - - -
      0xe3 => unimplemented!("Prefix opcode 0xe3 is not yet implemented"),
      // 0xe4 | SET 4,H | 2 | 8 | - - - -
      0xe4 => unimplemented!("Prefix opcode 0xe4 is not yet implemented"),
      // 0xe5 | SET 4,L | 2 | 8 | - - - -
      0xe5 => unimplemented!("Prefix opcode 0xe5 is not yet implemented"),
      // 0xe6 | SET 4,(HL) | 2 | 16 | - - - -
      0xe6 => unimplemented!("Prefix opcode 0xe6 is not yet implemented"),
      // 0xe7 | SET 4,A | 2 | 8 | - - - -
      0xe7 => unimplemented!("Prefix opcode 0xe7 is not yet implemented"),
      // 0xe8 | SET 5,B | 2 | 8 | - - - -
      0xe8 => unimplemented!("Prefix opcode 0xe8 is not yet implemented"),
      // 0xe9 | SET 5,C | 2 | 8 | - - - -
      0xe9 => unimplemented!("Prefix opcode 0xe9 is not yet implemented"),
      // 0xea | SET 5,D | 2 | 8 | - - - -
      0xea => unimplemented!("Prefix opcode 0xea is not yet implemented"),
      // 0xeb | SET 5,E | 2 | 8 | - - - -
      0xeb => unimplemented!("Prefix opcode 0xeb is not yet implemented"),
      // 0xec | SET 5,H | 2 | 8 | - - - -
      0xec => unimplemented!("Prefix opcode 0xec is not yet implemented"),
      // 0xed | SET 5,L | 2 | 8 | - - - -
      0xed => unimplemented!("Prefix opcode 0xed is not yet implemented"),
      // 0xee | SET 5,(HL) | 2 | 16 | - - - -
      0xee => unimplemented!("Prefix opcode 0xee is not yet implemented"),
      // 0xef | SET 5,A | 2 | 8 | - - - -
      0xef => unimplemented!("Prefix opcode 0xef is not yet implemented"),
      // 0xf0 | SET 6,B | 2 | 8 | - - - -
      0xf0 => unimplemented!("Prefix opcode 0xf0 is not yet implemented"),
      // 0xf1 | SET 6,C | 2 | 8 | - - - -
      0xf1 => unimplemented!("Prefix opcode 0xf1 is not yet implemented"),
      // 0xf2 | SET 6,D | 2 | 8 | - - - -
      0xf2 => unimplemented!("Prefix opcode 0xf2 is not yet implemented"),
      // 0xf3 | SET 6,E | 2 | 8 | - - - -
      0xf3 => unimplemented!("Prefix opcode 0xf3 is not yet implemented"),
      // 0xf4 | SET 6,H | 2 | 8 | - - - -
      0xf4 => unimplemented!("Prefix opcode 0xf4 is not yet implemented"),
      // 0xf5 | SET 6,L | 2 | 8 | - - - -
      0xf5 => unimplemented!("Prefix opcode 0xf5 is not yet implemented"),
      // 0xf6 | SET 6,(HL) | 2 | 16 | - - - -
      0xf6 => unimplemented!("Prefix opcode 0xf6 is not yet implemented"),
      // 0xf7 | SET 6,A | 2 | 8 | - - - -
      0xf7 => unimplemented!("Prefix opcode 0xf7 is not yet implemented"),
      // 0xf8 | SET 7,B | 2 | 8 | - - - -
      0xf8 => unimplemented!("Prefix opcode 0xf8 is not yet implemented"),
      // 0xf9 | SET 7,C | 2 | 8 | - - - -
      0xf9 => unimplemented!("Prefix opcode 0xf9 is not yet implemented"),
      // 0xfa | SET 7,D | 2 | 8 | - - - -
      0xfa => unimplemented!("Prefix opcode 0xfa is not yet implemented"),
      // 0xfb | SET 7,E | 2 | 8 | - - - -
      0xfb => unimplemented!("Prefix opcode 0xfb is not yet implemented"),
      // 0xfc | SET 7,H | 2 | 8 | - - - -
      0xfc => unimplemented!("Prefix opcode 0xfc is not yet implemented"),
      // 0xfd | SET 7,L | 2 | 8 | - - - -
      0xfd => unimplemented!("Prefix opcode 0xfd is not yet implemented"),
      // 0xfe | SET 7,(HL) | 2 | 16 | - - - -
      0xfe => unimplemented!("Prefix opcode 0xfe is not yet implemented"),
      // 0xff | SET 7,A | 2 | 8 | - - - -
      0xff => unimplemented!("Prefix opcode 0xff is not yet implemented"),
      opcode @ _ => panic!("Invalid prefix opcode: {:?}", opcode),
    };

    self.cycles += OPCODE_DUR_PREFIX[opcode as usize] as u64;
  }

  fn read_word(&self, addr: u16) -> u8 {
    debug!("Read word from: 0x{:x}", addr);
    if Util::in_range(0x0000, 0x8000, addr) {
      if self.has_cartrige() {
        self.rom[addr as usize]
      } else {
        if Util::in_range(0x0000, 0x0100, addr) {
          self.dmg_rom[addr as usize]
        } else {
          // TODO This is only because we need the Nintendo logo from 0x0104 - be careful not sure if correct.
          self.rom[addr as usize]
        }
      }
    } else {
      self.mem.read_word(addr)
    }
  }

  fn write_word(&mut self, addr: u16, w: u8) {
    if Util::in_range(0x8000, 0xa000, addr) || // video ram
      Util::in_range(0xff80, 0xffff, addr)
    // internal ram
    {
      self.mem.write_word(addr, w);
    } else if Util::in_range(0xff00, 0xff80, addr) {
      // i/o ports ---> THIS NEEDS SPECIAL CARE
      match addr {
        0xff10...0xff3f => self.sound.write_word(addr, w),
        0xff42 => self.graphics.write_word(addr, w),
        0xff47 => self.graphics.write_word(addr, w),
        _ => unimplemented!("Unimplemented IO port: 0x{:>02x}", addr),
      };
    } else {
      unimplemented!(
        "Write on unknown address: 0x{:x} the value: 0x{:x}",
        addr,
        w
      );
    }
  }

  pub fn read_opcode_word(&mut self) -> u8 {
    let addr = self.cpu.pc_inc();
    self.read_word(addr)
  }

  fn read_opcode_dword(&mut self) -> u16 {
    let lo = self.read_opcode_word();
    let hi = self.read_opcode_word();
    ((hi as u16) << 0x8) | lo as u16
  }

  fn read_dmg_rom(&mut self) {
    let mut rom_file = File::open("asset/dmg_rom.bin").unwrap();
    let _ = rom_file.read_to_end(&mut self.dmg_rom).unwrap();
  }

  fn read_rom(&mut self) {
    let mut rom_file = File::open("asset/tetris.gb").unwrap();
    let _ = rom_file.read_to_end(&mut self.rom).unwrap();
  }

  pub fn push_word(&mut self, w: u8) {
    self.write_word(self.cpu.sp, w);
    self.cpu.sp -= 1;
  }

  pub fn push_dword(&mut self, dw: u16) {
    self.push_word(dw.lo());
    self.push_word(dw.hi());
  }

  pub fn pop_word(&mut self) -> u8 {
    self.cpu.sp += 1;
    self.read_word(self.cpu.sp)
  }

  pub fn pop_dword(&mut self) -> u16 {
    let hi = self.pop_word();
    let lo = self.pop_word();
    dword!(hi, lo)
  }

  fn reset(&mut self) {
    self.cpu.reset();
    self.mem.reset();
    self.sound.reset();
    self.graphics.reset();
    self.interrupts_enabled = false;
  }

  fn interrupt_enabled_v_blank(&self) -> bool {
    bitn!(self.read_word(0xffff), 0) == 0x1
  }

  fn interrupt_enabled_lcd_stat(&self) -> bool {
    bitn!(self.read_word(0xffff), 1) == 0x1
  }

  fn interrupt_enabled_timer(&self) -> bool {
    bitn!(self.read_word(0xffff), 2) == 0x1
  }

  fn interrupt_enabled_serial(&self) -> bool {
    bitn!(self.read_word(0xffff), 3) == 0x1
  }

  fn interrupt_enabled_joypad(&self) -> bool {
    bitn!(self.read_word(0xffff), 4) == 0x1
  }

  fn interrupt_flag_v_blank(&self) -> bool {
    bitn!(self.read_word(0xff0f), 0) == 0x1
  }

  fn interrupt_flag_lcd_stat(&self) -> bool {
    bitn!(self.read_word(0xff0f), 1) == 0x1
  }

  fn interrupt_flag_timer(&self) -> bool {
    bitn!(self.read_word(0xff0f), 2) == 0x1
  }

  fn interrupt_flag_serial(&self) -> bool {
    bitn!(self.read_word(0xff0f), 3) == 0x1
  }

  fn interrupt_flag_joypad(&self) -> bool {
    bitn!(self.read_word(0xff0f), 4) == 0x1
  }

  fn mem_debug_print(&self, addr: u16, len: usize) {
    for offs in 0..len {
      if offs % 8 == 0 {
        print!("\n[0x{:>04x}] ", (addr + offs as u16));
      }

      if offs % 4 == 0 {
        print!(" ");
      }

      print!("{:>02x} ", self.read_word(addr + offs as u16));
    }

    println!("");
  }

  fn has_cartrige(&self) -> bool {
    // TODO add mechanism to check.
    false
  }
}

#[test]
fn test_stack() {
  let mut emu = Emu::new();
  emu.push_dword(0xabcd);
  assert_eq!(0xabcd, emu.pop_dword());
}
