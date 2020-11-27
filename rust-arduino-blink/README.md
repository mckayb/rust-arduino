### Getting Started

### On a Mac
1) Plug in the arduino. (Only tested on an old arduino uno)

2) Build the program and flash it to the arduino
```
cargo build
sh flash.sh target/avr-atmega328p/debug/rust-arduino-blink.elf
```