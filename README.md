# Chippy

Another CHIP-8 emulator, as a first step in emulator developpement, and Rust practice.  

All opcodes are implemented folowing the description from [wiki](https://en.wikipedia.org/wiki/CHIP-8).

The emulator pass all tests from [chip8-test-suite](https://github.com/Timendus/chip8-test-suite) excepted for sprite drawing frequency limitation because only one cycle is treated between frames.