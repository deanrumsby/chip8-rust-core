# chip8_core

A Chip8 interpreter library. 

## Motivation

I have several project ideas that involve the Chip8 interpreter, since it is such a simple system to build for, so I wanted
to create a library that I could use for several different targets. Thus, the aim for this library is to create a simple, accurate 
and very portable library that I can build different UIs to attach to.

## Environments

- Desktop
- Browser (via WASM)
- Embedded

## Installation

- via cargo: `cargo add chip8_core`
- via npm (WASM module): `npm install @deanrumsby/chip8_core`

## Examples

There is an example application for each runtime environment in the `examples` folder.
You will need to clone this repo and then look to the specific example's README for instructions on how to run it. 
