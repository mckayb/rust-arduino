#! /usr/bin/zsh

set -e

cargo build --release
# avrdude -q -C/etc/avrdude.conf -patmega328p -carduino -P/dev/tty.usbmodem141201 -D "-Uflash:w:$1:e"
avrdude -q -patmega328p -carduino -P/dev/tty.usbmodem144301 -D -Uflash:w:"target/avr-atmega328p/release/rust-arduino-03-love-o-meter.elf"


