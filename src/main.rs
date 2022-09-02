#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// The I/O on the GPIO pins
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins: rp_pico::Pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let input_pin = pins.gpio2.into_pull_up_input();
    let mut output_pin = pins.gpio0.into_push_pull_output();

    loop {
        let is_button_pressed = input_pin.is_low().unwrap();
        if is_button_pressed {
            output_pin.set_high().unwrap();
        } else {
            output_pin.set_low().unwrap();
        }
    }
}

// End of file
