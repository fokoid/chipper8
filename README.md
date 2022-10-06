# Chipper 8

Chipper 8 is a Rust implementation of a CHIP-8 interpreter.
CHIP-8 is widely recommended as a first emulator target due to its simplicity.
(Since CHIP-8 is a virtual machine and was never implemented in hardware we are really writing an _interpreter_ rather than an emulator.)

## Goals

* [ ] compliant implementation of the CHIP-8 virtual machine
* [ ] emulator/interpreter to run CHIP-8 ROMs
* [ ] REPL with live visualisation of VM state

## References

I primarily followed this [guide][GuideNoCode] which covers CHIP-8 in detail but leaves the actual code implementation to the reader.
For a detailed guide including a C++ implementation, see this [guide with code][GuideCode].
There is also a
great [CHIP-8 reference][Reference] with more technical details.

[GuideCode]: https://austinmorlan.com/posts/chip8_emulator/
[GuideNoCode]: https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
[Reference]: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap