# Emu8
A Chip-8 interpreter written in Rust. See [Chip8](src/chip8.rs) for more system documentation.

![demo gif](docs/demo.gif)

Design aspirations:
 - [ ] Reusable, independent components
 - [ ] Browser support via WebAssembly
 - [ ] Rewindable state
 - [x] Fast forward

Specifications are taken from [Columbia University's Chip8 Design Specification][columbia university] and [Cowgod's Chip-8 Technical Reference][cowgod] among many other sources.

[columbia university]: http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf
[cowgod]: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
