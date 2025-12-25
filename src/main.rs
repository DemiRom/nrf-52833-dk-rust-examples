#![no_main]
#![no_std]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::{Duration, Timer};
use {cortex_m as _, defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_nrf::init(Default::default());
    // nRF52833 DK LED1 is P0.13; adjust if your board uses a different pin.
    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    defmt::info!("Embassy blinky started");

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
