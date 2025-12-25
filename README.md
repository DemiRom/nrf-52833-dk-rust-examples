# nrf52833dk-rust-sample

This repo uses branches to keep separate example applications. Check out the branch you want, build, and flash with `cargo run`.

Examples by branch

- `develop/embassy`: Embassy async baseline (blinky + defmt logging).
- `develop/bluetooth`: Embassy + nrf-softdevice BLE peripheral (advertising + GATT example).
- `develop/uart`: Embassy UART echo using UARTE0.

Notes

- Default target is `thumbv7em-none-eabihf`.
- See `.cargo/config.toml` in each branch for runner and logging settings.
