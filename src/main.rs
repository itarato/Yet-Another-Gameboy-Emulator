pub mod cpu;
pub mod emu;
pub mod mem;
pub mod util;

use self::emu::*;

fn main() {
    Emu::new().run();
}
