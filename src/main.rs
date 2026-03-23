mod colors;
mod config;
mod device;
mod protocol;
mod service;
mod variants;

use std::collections::HashMap;
use std::process;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "nzxt-lightctl",
    version,
    about = "Linux LED controller for NZXT Function keyboards",
    after_help = "COLORS:\n  \
        Use hex codes (ff0000) or names: red, green, blue, white, purple,\n  \
        cyan, yellow, orange, pink, magenta, teal, lime, coral, ice, gold, lavender"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set LED colors (hex or named: red, blue, purple, etc.)
    Set {
        /// Color for all zones (hex or name)
        #[arg(short, long)]
        color: Option<String>,

        /// Rainbow pattern across zones
        #[arg(long)]
        rainbow: bool,

        /// Set a specific zone: --zone <name> <color>
        #[arg(short, long, num_args = 2, action = clap::ArgAction::Append)]
        zone: Vec<String>,

        /// Save this setting to config for auto-apply
        #[arg(long)]
        save: bool,
    },
    /// Turn off all LEDs
    Off {
        /// Save this setting to config
        #[arg(long)]
        save: bool,
    },
    /// Apply saved config
    Apply,
    /// List zones for the detected keyboard
    Zones,
    /// Show device info and current config
    Status,
    /// List available named colors
    Colors,
    /// Install udev rule and systemd service (requires root)
    InstallService,
    /// Remove udev rule and systemd service (requires root)
    UninstallService,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Set { color, rainbow, zone, save } => {
            let dev = device::Device::detect()?;
            let variant = dev.variant;

            let (frame, cfg) = if rainbow {
                let user_colors = protocol::rainbow_colors(variant.zone_count);
                let internal = protocol::map_user_colors(&user_colors, variant);
                (internal, config::Config::rainbow())
            } else if !zone.is_empty() {
                let mut internal = [(0u8, 0u8, 0u8); 10];
                let mut zone_map = HashMap::new();
                let mapping = variant.user_to_internal();

                for pair in zone.chunks(2) {
                    let (zone_name, color_str) = (&pair[0], &pair[1]);
                    let user_idx = resolve_zone(zone_name, variant)?;
                    let rgb = colors::resolve_color(color_str)
                        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
                    if user_idx < mapping.len() {
                        internal[mapping[user_idx]] = rgb;
                    }
                    zone_map.insert(
                        variant.zone_names[user_idx].to_string(),
                        color_str.clone(),
                    );
                }
                (internal, config::Config::per_zone(zone_map))
            } else if let Some(ref color_str) = color {
                let rgb = colors::resolve_color(color_str)
                    .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
                let internal = [rgb; 10];
                (internal, config::Config::solid(color_str))
            } else {
                return Err("Specify --color, --rainbow, or --zone".into());
            };

            let mut open = dev.open()?;
            open.send_colors(&frame)?;
            eprintln!("LEDs updated on {}", variant.name);

            if save {
                config::save(&cfg)?;
            }
        }

        Commands::Off { save } => {
            let dev = device::Device::detect()?;
            let mut open = dev.open()?;
            open.send_colors(&[(0, 0, 0); 10])?;
            eprintln!("LEDs off on {}", dev.variant.name);

            if save {
                config::save(&config::Config::off())?;
            }
        }

        Commands::Apply => {
            let cfg = config::load()?;
            let dev = device::Device::detect()?;
            let colors = config::resolve_colors(&cfg, dev.variant)?;
            let mut open = dev.open()?;
            open.send_colors(&colors)?;
            eprintln!("Applied saved config to {}", dev.variant.name);
        }

        Commands::Zones => {
            let dev = device::Device::detect()?;
            println!("{} — {} zones:", dev.variant.name, dev.variant.zone_count);
            for label in dev.variant.zone_labels {
                println!("  {label}");
            }
        }

        Commands::Status => {
            let dev = device::Device::detect()?;
            println!("Device:    {}", dev.variant.name);
            println!("PID:       0x{:04X}", dev.variant.pid);
            println!("Path:      {}", dev.path().display());
            println!("Zones:     {}", dev.variant.zone_count);

            match config::load() {
                Ok(cfg) => {
                    println!("Config:    mode={}", cfg.mode);
                    if let Some(ref c) = cfg.color {
                        println!("           color={c}");
                    }
                    if let Some(ref zones) = cfg.zones {
                        for (name, color) in zones {
                            println!("           {name}={color}");
                        }
                    }
                }
                Err(_) => println!("Config:    (none saved)"),
            }
        }

        Commands::Colors => {
            println!("Available named colors:");
            for (name, (r, g, b)) in colors::NAMED_COLORS {
                println!("  {name:<12} #{r:02x}{g:02x}{b:02x}");
            }
        }

        Commands::InstallService => {
            service::install()?;
        }

        Commands::UninstallService => {
            service::uninstall()?;
        }
    }

    Ok(())
}

fn resolve_zone(
    input: &str,
    variant: &variants::Variant,
) -> Result<usize, Box<dyn std::error::Error>> {
    if let Some(idx) = variant.zone_name_to_user_index(input) {
        return Ok(idx);
    }
    if let Ok(idx) = input.parse::<usize>() {
        if idx < variant.zone_count {
            return Ok(idx);
        }
        return Err(format!(
            "Zone index {idx} out of range (0-{})",
            variant.zone_count - 1
        )
        .into());
    }
    Err(format!(
        "Unknown zone '{input}'. Run 'nzxt-lightctl zones' to list available zones."
    )
    .into())
}
