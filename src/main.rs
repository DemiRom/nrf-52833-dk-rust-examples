#![no_main]
#![no_std]

use embassy_executor::Spawner;
use embassy_nrf::config::{Config, LfclkSource};
use embassy_nrf::interrupt::Priority;
use cortex_m::peripheral::SCB;
use nrf_softdevice::ble::{gatt_server, peripheral};
use nrf_softdevice::{raw, Softdevice};
use rtt_target::{rprintln, rtt_init_print};
use {cortex_m as _, panic_probe as _};

#[nrf_softdevice::gatt_server]
struct Server {
    bas: BatteryService,
}

#[nrf_softdevice::gatt_service(uuid = "180F")]
struct BatteryService {
    #[characteristic(uuid = "2A19", read, notify)]
    battery_level: u8,
}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    rtt_init_print!();
    const APP_BASE: u32 = 0x00027000;
    // Ensure VTOR points at the SoftDevice/MBR vector table before enabling.
    unsafe { (*SCB::ptr()).vtor.write(0x0000_0000) };
    let mut config = Config::default();
    config.lfclk_source = LfclkSource::ExternalXtal;
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let _p = embassy_nrf::init(config);

    let mut sd_config = nrf_softdevice::Config::default();
    sd_config.clock = Some(raw::nrf_clock_lf_cfg_t {
        source: raw::NRF_CLOCK_LF_SRC_XTAL as u8,
        rc_ctiv: 0,
        rc_temp_ctiv: 0,
        accuracy: raw::NRF_CLOCK_LF_ACCURACY_20_PPM as u8,
    });

    rprintln!("Enabling SoftDevice");
    let sd = Softdevice::enable(&sd_config);
    // Let SoftDevice forward interrupts to our vector table.
    unsafe {
        let _ = raw::sd_softdevice_vector_table_base_set(APP_BASE);
    }
    rprintln!("SoftDevice enabled");
    let server = Server::new(sd).unwrap();
    let sd_ref: &'static Softdevice = sd;
    spawner.spawn(softdevice_task(sd_ref)).unwrap();

    let adv_data = [
        0x02, 0x01, 0x06,
        0x03, 0x03, 0x0F, 0x18,
        0x0C, 0x09, b'n', b'R', b'F', b'5', b'2', b'8', b'3', b'3', b'-', b'D', b'K',
    ];
    let scan_data = [0x03, 0x03, 0x0F, 0x18];

    loop {
        rprintln!("Advertising");
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
            adv_data: &adv_data,
            scan_data: &scan_data,
        };
        let conn = match peripheral::advertise_connectable(sd_ref, adv, &peripheral::Config::default()).await {
            Ok(conn) => conn,
            Err(e) => {
                rprintln!("advertise error: {:?}", e);
                continue;
            }
        };
        rprintln!("Connected");
        let _ = gatt_server::run(&conn, &server, |event| match event {
            ServerEvent::Bas(_) => {}
        })
        .await;
        rprintln!("Disconnected");
    }
}
