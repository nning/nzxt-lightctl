# nzxt-lightctl

Linux LED controller for NZXT Function keyboards.

> **Status**: Early development — works with NZXT Function TKL ANSI. Other variants (ISO, Full-size, MiniTKL) are defined but untested.

## Supported Devices

| Keyboard | USB PID | Zones | Status |
|---|---|---|---|
| Function TKL (ANSI) | `2104` | 8 | Tested |
| Function TKL (ISO) | `2107` | 8 | Untested |
| Function Full-size (ANSI) | `2103` | 10 | Untested |
| Function Full-size (ISO) | `2106` | 10 | Untested |
| Function MiniTKL (ANSI) | `2105` | 8 | Untested |
| Function MiniTKL (ISO) | `2108` | 8 | Untested |

All devices share USB VID `1E71` (NZXT).

## Install

```bash
cargo install nzxt-lightctl
```

Or build from source:

```bash
git clone https://github.com/dyllybot/nzxt-lightctl.git
cd nzxt-lightctl
cargo build --release
sudo cp target/release/nzxt-lightctl /usr/local/bin/
```

## Setup

The keyboard's HID interface needs to be accessible without root:

```bash
sudo nzxt-lightctl install-service
```

This installs a udev rule (for device permissions) and a systemd service (to auto-apply your saved color config on plug-in). Then unplug and replug the keyboard.

## Usage

```bash
# Set all zones to a color
nzxt-lightctl set --color ff0000

# Rainbow pattern
nzxt-lightctl set --rainbow

# Set specific zones
nzxt-lightctl set --zone esc-row ff0000 --zone tab-row 00ff00

# Turn off LEDs
nzxt-lightctl off

# Save current setting to auto-apply on plug-in
nzxt-lightctl set --color 00ffcc --save

# Apply saved config
nzxt-lightctl apply

# List zones for your keyboard
nzxt-lightctl zones

# Show device info
nzxt-lightctl status
```

## How It Works

NZXT Function keyboards expose a vendor-specific HID interface for LED control. This tool sends HID output reports (report ID `0x43`, 64-byte packets) to the `/dev/hidraw` device on interface 1.

The keyboard has zone-based LED control (not per-key). Each zone covers a row or region of the keyboard.

Protocol was reverse-engineered from [SignalRGB plugins](https://gitlab.com/signalrgb/signal-plugins/-/tree/master/Plugins/Nzxt/Peripheral%20Protocol).

## License

MIT
