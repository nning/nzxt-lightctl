use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::protocol::{self, Color};
use crate::variants::{self, Variant};

const PACKET_DELAY: Duration = Duration::from_millis(1);
const INIT_DELAY: Duration = Duration::from_millis(10);

pub struct Device {
    path: PathBuf,
    pub variant: &'static Variant,
}

pub struct OpenDevice {
    file: fs::File,
    pub variant: &'static Variant,
}

impl Device {
    /// Scan sysfs to find an NZXT Function keyboard on hidraw interface 1.
    pub fn detect() -> io::Result<Self> {
        let hidraw_dir = PathBuf::from("/sys/class/hidraw");
        let entries = fs::read_dir(&hidraw_dir).map_err(|e| {
            io::Error::new(e.kind(), format!("Cannot read /sys/class/hidraw: {e}"))
        })?;

        for entry in entries {
            let entry = entry?;
            let uevent_path = entry.path().join("device/uevent");
            let uevent = match fs::read_to_string(&uevent_path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            // Must be on input1 (the LED control interface)
            if !uevent.contains("input1") {
                continue;
            }

            // Check for NZXT VID
            if !uevent.contains(&format!("{:04X}", variants::VID))
                && !uevent.contains(&format!("{:04x}", variants::VID))
            {
                continue;
            }

            // Try to match a known PID
            for variant in variants::VARIANTS {
                let pid_upper = format!("{:04X}", variant.pid);
                let pid_lower = format!("{:04x}", variant.pid);
                if uevent.contains(&pid_upper) || uevent.contains(&pid_lower) {
                    let dev_path =
                        PathBuf::from("/dev").join(entry.file_name());
                    return Ok(Device {
                        path: dev_path,
                        variant,
                    });
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No NZXT Function keyboard found. Is it plugged in?",
        ))
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn open(&self) -> io::Result<OpenDevice> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .map_err(|e| {
                if e.kind() == io::ErrorKind::PermissionDenied {
                    io::Error::new(
                        e.kind(),
                        format!(
                            "Permission denied on {}. Run: sudo nzxt-lightctl install-service",
                            self.path.display()
                        ),
                    )
                } else {
                    e
                }
            })?;

        Ok(OpenDevice {
            file,
            variant: self.variant,
        })
    }
}

impl OpenDevice {
    fn write_packet(&mut self, pkt: &[u8]) -> io::Result<()> {
        self.file.write_all(pkt)?;
        thread::sleep(PACKET_DELAY);
        Ok(())
    }

    pub fn send_colors(&mut self, colors: &[Color; 10]) -> io::Result<()> {
        let init_pkts = protocol::build_init_packets();
        for pkt in &init_pkts {
            self.write_packet(pkt)?;
        }
        thread::sleep(INIT_DELAY);

        let color_pkts = protocol::build_color_packets(colors, self.variant);
        for pkt in &color_pkts {
            self.write_packet(pkt)?;
        }
        Ok(())
    }
}
