// main.rs

#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;


#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let pushbutton = pins.d2.into_floating_input(&mut pins.ddr);

    let mut led3 = pins.d3.into_output(&mut pins.ddr);
    let mut led4 = pins.d4.into_output(&mut pins.ddr);
    let mut led5 = pins.d5.into_output(&mut pins.ddr);

    loop {
        if pushbutton.is_low().void_unwrap() {
            led3.set_high().void_unwrap();
            led4.set_low().void_unwrap();
            led5.set_low().void_unwrap();
        } else {
            led3.set_low().void_unwrap();
            led4.set_low().void_unwrap();
            led5.set_high().void_unwrap();

            arduino_uno::delay_ms(250);

            led4.set_high().void_unwrap();
            led5.set_low().void_unwrap();

            arduino_uno::delay_ms(250);
        }

    }
}