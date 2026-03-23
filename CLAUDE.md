# nzxt-lightctl

## Overview

Linux CLI tool to control LEDs on NZXT Function keyboards via HID output reports over `/dev/hidraw`. Written in Rust. Single binary, no runtime dependencies.

## Architecture

- `src/main.rs` — Clap CLI entry point
- `src/variants.rs` — Keyboard variant definitions (PIDs, zones, names)
- `src/protocol.rs` — HID packet construction (pure functions)
- `src/device.rs` — hidraw device discovery and I/O via sysfs
- `src/colors.rs` — Named color definitions and hex/name resolution
- `src/config.rs` — Config file read/write (~/.config/nzxt-lightctl/config.toml)
- `src/service.rs` — udev/systemd install/uninstall

## Protocol

- USB VID: `0x1E71` (NZXT), PIDs vary by variant
- HID interface 1 is the LED control interface
- Report ID: `0x43`, 64 bytes total (1 byte report ID + 63 bytes data)
- Init: 2 packets, then 4 "color burst" packets to set all zones
- 1ms delay between packets
- Protocol derived from SignalRGB plugins (see /tmp/nzxt_tkl_signalrgb.js for reference)

## Commands

```
cargo build                    # build
cargo test                     # run tests
cargo run -- <subcommand>      # run locally
cargo clippy                   # lint
```

## Code Style

- Line length: 99
- Comments explain *why*, not *what*
