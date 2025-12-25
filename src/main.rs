#![no_main]
#![no_std]

use embassy_executor::Spawner;
use embassy_nrf::bind_interrupts;
use embassy_nrf::uarte::{self, Uarte};
use {cortex_m as _, defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UARTE0_UART0 => uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_nrf::init(Default::default());
    defmt::info!("Embassy UART echo started");

    let mut config = uarte::Config::default();
    config.baudrate = uarte::Baudrate::BAUD115200;
    // nRF52833 DK UART: RXD P0.08, TXD P0.06. Adjust to match your wiring.
    let mut uart = Uarte::new(p.UARTE0, Irqs, p.P0_08, p.P0_06, config);

    let mut byte = [0u8; 1];
    loop {
        if uart.read(&mut byte).await.is_ok() {
            let _ = uart.write(&byte).await;
        }
    }
}
