#! /usr/bin/zsh

set -e

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "usage: $0 <path-to-binary.elf>" >&2
    exit 1
fi

if [ "$#" -lt 1 ]; then
    echo "$0: Expecting a .elf file" >&2
    exit 1
fi

cargo build
# avrdude -q -C/etc/avrdude.conf -patmega328p -carduino -P/dev/tty.usbmodem141201 -D "-Uflash:w:$1:e"
avrdude -q -patmega328p -carduino -P/dev/tty.usbmodem144201 -D -Uflash:w:"$1"



