use std::fs;
use std::io;
use std::process::Command;

const UDEV_RULE_PATH: &str = "/etc/udev/rules.d/99-nzxt-lightctl.rules";
const SYSTEMD_SERVICE_PATH: &str = "/etc/systemd/system/nzxt-lightctl.service";

const UDEV_RULE: &str = r#"# NZXT Function keyboard — allow user access to hidraw + trigger LED service on plug-in
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1e71", ATTRS{idProduct}=="2104", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1e71", ATTRS{idProduct}=="2107", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1e71", ATTRS{idProduct}=="2103", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1e71", ATTRS{idProduct}=="2106", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1e71", ATTRS{idProduct}=="2105", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1e71", ATTRS{idProduct}=="2108", MODE="0666"
ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="1e71", ATTR{idProduct}=="2104", TAG+="systemd", ENV{SYSTEMD_WANTS}="nzxt-lightctl.service"
ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="1e71", ATTR{idProduct}=="2107", TAG+="systemd", ENV{SYSTEMD_WANTS}="nzxt-lightctl.service"
ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="1e71", ATTR{idProduct}=="2103", TAG+="systemd", ENV{SYSTEMD_WANTS}="nzxt-lightctl.service"
ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="1e71", ATTR{idProduct}=="2106", TAG+="systemd", ENV{SYSTEMD_WANTS}="nzxt-lightctl.service"
ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="1e71", ATTR{idProduct}=="2105", TAG+="systemd", ENV{SYSTEMD_WANTS}="nzxt-lightctl.service"
ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="1e71", ATTR{idProduct}=="2108", TAG+="systemd", ENV{SYSTEMD_WANTS}="nzxt-lightctl.service"
"#;

fn systemd_service(binary_path: &str) -> String {
    format!(
        r#"[Unit]
Description=NZXT Function Keyboard LED Controller
After=sys-subsystem-hidraw.target

[Service]
Type=oneshot
ExecStartPre=/bin/sleep 1
ExecStart={binary_path} apply
RemainAfterExit=no

[Install]
WantedBy=multi-user.target
"#
    )
}

fn check_root() -> io::Result<()> {
    if unsafe { libc::geteuid() } != 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "This command must be run as root. Try: sudo nzxt-lightctl install-service",
        ));
    }
    Ok(())
}

fn find_binary() -> io::Result<String> {
    std::env::current_exe()?
        .to_str()
        .map(String::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Cannot determine binary path"))
}

fn run_cmd(cmd: &str, args: &[&str]) -> io::Result<()> {
    let status = Command::new(cmd).args(args).status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{cmd} {} failed", args.join(" ")),
        ));
    }
    Ok(())
}

pub fn install() -> io::Result<()> {
    check_root()?;
    let binary = find_binary()?;

    fs::write(UDEV_RULE_PATH, UDEV_RULE)?;
    eprintln!("Installed {UDEV_RULE_PATH}");

    fs::write(SYSTEMD_SERVICE_PATH, systemd_service(&binary))?;
    eprintln!("Installed {SYSTEMD_SERVICE_PATH}");

    run_cmd("systemctl", &["daemon-reload"])?;
    run_cmd("udevadm", &["control", "--reload-rules"])?;
    run_cmd("udevadm", &["trigger"])?;
    run_cmd("systemctl", &["enable", "nzxt-lightctl.service"])?;
    eprintln!("Service enabled. Unplug and replug your keyboard to activate.");

    Ok(())
}

pub fn uninstall() -> io::Result<()> {
    check_root()?;

    let _ = run_cmd("systemctl", &["disable", "nzxt-lightctl.service"]);

    if fs::remove_file(UDEV_RULE_PATH).is_ok() {
        eprintln!("Removed {UDEV_RULE_PATH}");
    }
    if fs::remove_file(SYSTEMD_SERVICE_PATH).is_ok() {
        eprintln!("Removed {SYSTEMD_SERVICE_PATH}");
    }

    run_cmd("systemctl", &["daemon-reload"])?;
    run_cmd("udevadm", &["control", "--reload-rules"])?;
    run_cmd("udevadm", &["trigger"])?;
    eprintln!("Service uninstalled.");

    Ok(())
}
