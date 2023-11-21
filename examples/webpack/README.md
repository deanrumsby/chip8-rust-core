# webpack

This example uses the WASM module alongside the webpack bundler to run within the browser.

## Installation

You will need:

- `wasm-pack`
- `npm`

installed to run this example.

Then, execute (from `examples/browser-bundler`):

```
npm install
```

to install the package.

Now, to begin using the example program, run:

```
npm start
```

to create a local dev server and begin the application.

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
