use crate::protocol::Color;

pub const NAMED_COLORS: &[(&str, Color)] = &[
    ("red", (255, 0, 0)),
    ("green", (0, 255, 0)),
    ("blue", (0, 0, 255)),
    ("white", (255, 255, 255)),
    ("purple", (128, 0, 255)),
    ("cyan", (0, 255, 255)),
    ("yellow", (255, 255, 0)),
    ("orange", (255, 128, 0)),
    ("pink", (255, 0, 128)),
    ("magenta", (255, 0, 255)),
    ("teal", (0, 128, 128)),
    ("lime", (128, 255, 0)),
    ("coral", (255, 100, 80)),
    ("ice", (100, 200, 255)),
    ("gold", (255, 200, 0)),
    ("lavender", (180, 130, 255)),
];

/// Resolve a color string: named color, or hex (with or without #).
pub fn resolve_color(input: &str) -> Result<Color, String> {
    let lower = input.to_lowercase();

    if let Some(&(_, color)) = NAMED_COLORS.iter().find(|(name, _)| *name == lower) {
        return Ok(color);
    }

    let hex = lower.strip_prefix('#').unwrap_or(&lower);
    if hex.len() != 6 {
        return Err(format!(
            "Unknown color '{input}'. Use a hex code (ff0000) or name: {}",
            list_names()
        ));
    }

    let r = u8::from_str_radix(&hex[0..2], 16);
    let g = u8::from_str_radix(&hex[2..4], 16);
    let b = u8::from_str_radix(&hex[4..6], 16);

    match (r, g, b) {
        (Ok(r), Ok(g), Ok(b)) => Ok((r, g, b)),
        _ => Err(format!("Invalid hex color '{input}'")),
    }
}

pub fn list_names() -> String {
    NAMED_COLORS
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(", ")
}
