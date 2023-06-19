# browser

This example uses the built WASM module directly (without a bundler such as webpack).

## Installation

We will need the following software:

- `cargo` as part of the Rust toolchain
- `wasm-pack` to build the WASM module
- `python3` to spin up a quick web-server locally

With these installed, we can execute `./start.zsh` to run the example.

## Programs

You will need some Chip8 programs to make use of the example. Visit <https://github.com/kripod/chip8-roms> to download some.

## Keys

The common mapping is used:

**QWERTY Keyboard**
```
| 1 | 2 | 3 | 4 |  
| Q | W | E | R | 
| A | S | D | F |  
| Z | X | C | V | 
```
maps to 

**Chip8 Keypad**
```
| 1 | 2 | 3 | C |
| 4 | 5 | 6 | D |
| 7 | 8 | 9 | E |
| A | 0 | B | F |
```
