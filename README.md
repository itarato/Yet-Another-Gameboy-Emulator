# Yet Another GameBoy Emulator

### Requirements

- [SDL2 (core and ttf)](https://www.libsdl.org/download-2.0.php)
- [Rust stable](https://rustup.rs/)
- a _simple_ DMG rom (tetris, mario, etc)

### Run

- `cargo run --release -- CARTIDGE_FILE [--debug]`

### Debugger

Available commands

- `next` / `n` [STEPS=1]: next instruction
- `continue` / `c`: continue (until next breakpoint)
- `breakpoint` / `b` INSTRUCTION_HEX_CODE: break at instruction, eg `b 5d`
- `-breakpoint` / `-b` INSTRUCTION_HEX_CODE: remove breakpoint
- `memory` / `m` START_HEX [LENGTH=1]: print memory, eg `m 8C00 256`
- `backgroundmap` / `bgm`: update background map debug display
- `cpu`: print CPU registers
- `exit` / `e`: exit program
