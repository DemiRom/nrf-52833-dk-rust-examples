# Project Context

This project is an nRF52833 Rust firmware using Embassy + nrf-softdevice (S140) with probe-rs tooling. The current goal is to get BLE advertising and RTT logs working reliably.

## Current State

- Target: `thumbv7em-none-eabihf`
- Runner: `.cargo/config.toml` uses `probe-rs run --chip nRF52833_xxAA --rtt-scan-memory`
- SoftDevice: S140 v7.3.0 flashed as a HEX (`s140_nrf52_7.3.0_softdevice.hex`)
- Flash layout: `memory.x` uses `FLASH ORIGIN = 0x00027000` to avoid overlapping the S140 image.
- RAM layout: `memory.x` currently uses `RAM ORIGIN = 0x20010000, LENGTH = 0x00010000` (still suspect).
- Logging: switched from defmt to RTT (`rtt-target`) because defmt decode was failing. RTT is initialized in `src/main.rs` with `rtt_init_print!()` and `rprintln!()`.
- SoftDevice enable is still crashing immediately after `rprintln!("Enabling SoftDevice")`.

## Key Files

- `src/main.rs`: Embassy executor + nrf-softdevice BLE peripheral example; also sets VTOR to 0 before enabling the SoftDevice and calls `sd_softdevice_vector_table_base_set(APP_BASE)` after enable.
- `memory.x`: app flash/ram origin (must match S140 memory requirements).
- `build.rs`: copies `memory.x` to OUT_DIR for linking.
- `.cargo/config.toml`: runner and link flags.

## Suspected Issues

- SoftDevice enable likely failing due to:
  - Vector table/VTOR setup (SVC handler not found).
  - RAM base still conflicting with S140 requirements.
- The correct RAM origin must match the SoftDevice’s requested RAM base for the current config. We have not yet captured this because logging fails before SoftDevice completes enable.

## Known Fixes Applied

- Added `nrf-softdevice` with `s140`, `ble-peripheral`, `ble-gatt-server`, and `critical-section-impl` features.
- Corrected flash start for S140 v7.3.0 to 0x27000.
- Adjusted interrupt priorities for Embassy (`P2`) to be lower than SoftDevice.
- Set VTOR directly via SCB before enabling SoftDevice and call `sd_softdevice_vector_table_base_set`.

## Next Steps

- Confirm whether the VTOR write or SoftDevice vector table base set is correct for this setup.
- Determine the exact RAM base required by S140 (likely via a minimal log path or by enabling a compatible defmt stack).
- Once RAM base is correct, BLE advertising should start; scan for device name "nRF52833-DK".
