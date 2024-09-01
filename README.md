rusty_db is a Gameboy emulator written in Rust. It is a work in progress and is not yet functional.

Even though there are plenty of Gameboy emulators out there, I wanted to write my own to learn more about emulation, the Gameboy hardware, and to improve my Rust skills. Since Rust can be used in both web assembly and embedded systems, I hope to be able to run this emulator on Linux first, then on the web and finally on a microcontroller such as raspberry pi pico if possible.

I took the inspiration from the following projects:
* [Gameboy emulator in C] https://github.com/rockytriton/LLD_gbemu/

Right now i am working on the CPU instructions, the interrupts, and the memory management. Then i will move to the the Pixel Processing Unit (PPU), which is the GPU of the Gameboy. Then i will start trying to run few games like Tetris.

# Building and testing
You can build the emulator using the following command:
```bash
cargo build --release
```
and you can test it using the following command:
```bash
cargo test
```

# Running test roms
There are some test roms in the `roms` directory. To build the emulator and run the tests, use the following commands:
```bash
cargo build --release roms/01-special.gb # for example
```

## Test roms

* 01-special.gb PENDING
* 02-interrupts.gb PENDING
* 03-op sp,hl.gb PENDING
* 04-op r,imm.gb PENDING
* 05-op rp.gb PENDING
* 06-ld r,r.gb PENDING
* 07-jr,jp,call,ret,rst.gb PENDING
* 08-misc instrs.gb PENDING
* 09-op r,r.gb PENDING
* 10-bit ops.gb PENDING
* 11-op a,(hl).gb PENDING

# References
* [The Cycle-Accurate Game Boy docs] https://raw.githubusercontent.com/rockytriton/LLD_gbemu/main/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf
* [Gameboy opcodes] https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
* [Pandocs] https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
