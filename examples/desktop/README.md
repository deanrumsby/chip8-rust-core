# desktop

An example demonstrating a basic chip8 emulator app in the desktop environment.

## Installation

You will need `cargo` to run this example. If you don't have the Rust toolchain installed, I recommend using `rustup`,
which can be downloaded from <https://rustup.rs>

## Usage

You will need some Chip8 programs to play, I suggest downloading some from <https://github.com/kripod/chip8-roms>

Once you have some programs downloaded, from this directory (the directory this README is located) run the following:
```
cargo run <PATH>
```
where `PATH` is the path to the program you wish to play.

## Keybindings

The traditional key bindings are used, that is:

### Keyboard 
```
| 1 | 2 | 3 | 4 |
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | V |
```

maps to

### Chip8 Keypad
```
| 1 | 2 | 3 | C |
| 4 | 5 | 6 | D |
| 7 | 8 | 9 | E |
| A | 0 | B | F |
```

## Limitations

This example will only play the programs with the default speed of 700 instructions per second, but some Chip8 programs
were not designed for this speed.
You can very easily extend the example to allow for custom speeds.
