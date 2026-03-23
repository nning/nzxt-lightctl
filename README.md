# nzxt-lightctl

Linux LED controller for NZXT Function keyboards. Single binary, no runtime dependencies.

> **Status**: Works with NZXT Function TKL ANSI. Other variants (ISO, Full-size, MiniTKL) are defined but untested.

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

Install the udev rule (device permissions) and systemd service (auto-apply on plug-in):

```bash
sudo nzxt-lightctl install-service
```

Then unplug and replug the keyboard.

To remove:

```bash
sudo nzxt-lightctl uninstall-service
```

## Usage

```bash
# Set all zones to a color (hex or named)
nzxt-lightctl set --color red
nzxt-lightctl set --color ff4500

# Rainbow pattern across zones
nzxt-lightctl set --rainbow

# Set specific zones
nzxt-lightctl set --zone esc-row red --zone ctrl-row blue

# Turn off LEDs
nzxt-lightctl off

# Save setting to auto-apply on plug-in
nzxt-lightctl set --color purple --save

# Apply saved config
nzxt-lightctl apply

# List zones for your keyboard
nzxt-lightctl zones

# List available named colors
nzxt-lightctl colors

# Show device info and saved config
nzxt-lightctl status
```

### Named Colors

red, green, blue, white, purple, cyan, yellow, orange, pink, magenta, teal, lime, coral, ice, gold, lavender

### Zones (TKL)

| Zone | Region |
|---|---|
| esc-row | Esc, F1-F6 area |
| number-row | F7-F12, number row |
| tab-row | Tab row |
| caps-row | Caps Lock row |
| shift-row | Shift row |
| ctrl-row | Ctrl, Alt, Space row |
| nav-cluster | Ins, Del, PgUp, PgDn |
| arrows | Arrow keys area |

## How It Works

NZXT Function keyboards expose a vendor-specific HID interface for LED control. This tool sends HID output reports (report ID `0x43`, 64-byte packets) to the `/dev/hidraw` device on interface 1.

The keyboard has **zone-based** LED control (not per-key). Each zone covers a row or region of the keyboard. The protocol was reverse-engineered from [SignalRGB plugins](https://gitlab.com/signalrgb/signal-plugins/-/tree/master/Plugins/Nzxt/Peripheral%20Protocol).

## License

MIT
