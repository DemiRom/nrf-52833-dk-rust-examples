#![no_std]
#![no_main]

/* define a default panic handler */
use panic_halt as _;

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use cortex_m_rt::entry;
use nrf52833_hal as hal;
use nrf52833_hal::gpio::Level;

/*
 *
 * If using the nRF52833_dk board
 * Button 1	P0.11	P1.07	-
 * Button 2	P0.12	P1.08	-
 * Button 3	P0.24	 	-
 * Button 4	P0.25	 	-
 * LED 1	P0.13	 	SB5
 * LED 2	P0.14	 	SB6
 * LED 3	P0.15	 	SB7
 * LED 4	P0.16	 	SB8
 */

#[entry]
fn main() -> ! {

    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let button = port.p0_11.into_pullup_input(); /* button 1 */
    let mut led = port0.p0_13.into_push_pull_output(Level::Low); /* led 1 */

    loop {
        if button.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
