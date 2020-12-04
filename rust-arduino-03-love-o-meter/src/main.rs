#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use panic_halt as _;

// This example opens a serial connection to the host computer.  On most POSIX operating systems (like GNU/Linux or
// OSX), you can interface with the program by running (assuming the device appears as ttyACM0)
//
// $ sudo screen /dev/ttyACM0 9600

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );

    let mut adc = arduino_uno::adc::Adc::new(dp.ADC, Default::default());

    let mut d2 = pins.d2.into_output(&mut pins.ddr);
    let mut d3 = pins.d3.into_output(&mut pins.ddr);
    let mut d4 = pins.d4.into_output(&mut pins.ddr);
    let mut a0 = pins.a0.into_analog_input(&mut adc);

    let baseline_temp: u16 = 20;

    d2.set_low().void_unwrap();
    d3.set_low().void_unwrap();
    d4.set_low().void_unwrap();

    loop {
        // This isn't perfect because for some reason, I can't get floating point math working properly...
        let sensor_val: u16 = nb::block!(adc.read(&mut a0)).void_unwrap();
        let voltage = ((sensor_val * 100) / 1024) * 5;
        let temperature = voltage - 50;

        // let sensor_val_str = sensor_val as u16;
        ufmt::uwriteln!(&mut serial, "Sensor Value: {}\r", sensor_val as u16).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Volts (x100): {}\r", voltage as u16).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Degrees C: {}\r", temperature as u16).void_unwrap();

        if temperature < baseline_temp {
            d2.set_low().void_unwrap();
            d3.set_low().void_unwrap();
            d4.set_low().void_unwrap();
        } else if temperature >= (baseline_temp + 2) && temperature < (baseline_temp + 4) {
            d2.set_high().void_unwrap();
            d3.set_low().void_unwrap();
            d4.set_low().void_unwrap();
        } else if temperature >= (baseline_temp + 4) && temperature < (baseline_temp + 6) {
            d2.set_high().void_unwrap();
            d3.set_high().void_unwrap();
            d4.set_low().void_unwrap();
        } else {
            d2.set_high().void_unwrap();
            d3.set_high().void_unwrap();
            d4.set_high().void_unwrap();
        }

        arduino_uno::delay_ms(1000);
    }
}