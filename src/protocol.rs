use crate::variants::Variant;

const REPORT_ID: u8 = 0x43;
const PACKET_SIZE: usize = 64;

pub type Color = (u8, u8, u8);

fn new_packet() -> [u8; PACKET_SIZE] {
    let mut pkt = [0u8; PACKET_SIZE];
    pkt[0] = REPORT_ID;
    pkt
}

pub fn build_init_packets() -> [[u8; PACKET_SIZE]; 2] {
    let mut pkt1 = new_packet();
    pkt1[1] = 0x81;
    pkt1[3] = 0x84;

    let mut pkt2 = new_packet();
    pkt2[1] = 0x81;
    pkt2[3] = 0x86;

    [pkt1, pkt2]
}

/// Build the 4 color burst packets.
/// `colors` is indexed by internal zone index (10 slots, indices 4/5 null on TKL).
pub fn build_color_packets(colors: &[Color; 10], variant: &Variant) -> [[u8; PACKET_SIZE]; 4] {
    [
        build_burst_one(colors, variant),
        build_burst_two(colors),
        build_burst_three(colors),
        build_burst_four(colors),
    ]
}

/// Burst 1: zones 0-1 (esc-row, number-row)
fn build_burst_one(colors: &[Color; 10], variant: &Variant) -> [u8; PACKET_SIZE] {
    let mut pkt = new_packet();
    pkt[0x01] = 0xBD;
    pkt[0x02] = 0x03;
    pkt[0x03] = 0x10;
    pkt[0x04] = 0x01;
    pkt[0x05] = 0x01;
    pkt[0x07] = 0x03;
    pkt[0x09] = 0x03;
    pkt[0x0B] = 0x03;
    pkt[0x0D] = variant.mapping_byte;
    pkt[0x0F] = 0x03;
    pkt[0x1A] = 0x01;
    pkt[0x1B] = 0x06;
    pkt[29] = 0x0C;
    pkt[31] = 0x0C;
    pkt[33] = 0x0C;
    pkt[35] = 0x0C;
    pkt[37] = 0x04;
    pkt[48] = 0x01;
    pkt[49] = 0x60;
    pkt[51] = 0xC0;
    pkt[53] = 0xC0;
    pkt[55] = 0xC0;
    pkt[57] = 0xC0;
    pkt[59] = 0x20;

    for zone_idx in 0..2 {
        let (r, g, b) = colors[zone_idx];
        pkt[(zone_idx * 22) + 23] = r;
        pkt[(zone_idx * 22) + 24] = g;
        pkt[(zone_idx * 22) + 25] = b;
    }

    pkt
}

/// Burst 2: zones 2-4 (caps-row, shift-row, null/numpad-upper)
fn build_burst_two(colors: &[Color; 10]) -> [u8; PACKET_SIZE] {
    let mut pkt = new_packet();
    pkt[0x01] = 0x3D;
    pkt[0x02] = 0x02;
    pkt[0x09] = 0x01;
    pkt[0x0A] = 0x80;
    pkt[0x0B] = 0x01;
    pkt[0x0D] = 0x03;
    pkt[0x0F] = 0x03;
    pkt[0x11] = 0x03;
    pkt[0x13] = 0x03;
    pkt[0x1F] = 0x01;
    pkt[0x2C] = 0xC0;
    pkt[0x2E] = 0x46;
    pkt[0x2F] = 0x0E;
    pkt[0x35] = 0x01;

    for zone_idx in 0..3 {
        let offset = zone_idx + 2;
        let (r, g, b) = colors[offset];
        pkt[(zone_idx * 22) + 6] = r;
        pkt[(zone_idx * 22) + 7] = g;
        pkt[(zone_idx * 22) + 8] = b;
    }

    pkt
}

/// Burst 3: zones 5-7 (null/numpad-lower, arrows, nav-cluster)
fn build_burst_three(colors: &[Color; 10]) -> [u8; PACKET_SIZE] {
    let mut pkt = new_packet();
    pkt[0x01] = 0x3D;
    pkt[0x02] = 0x01;
    pkt[0x05] = 0x32;
    pkt[0x06] = 0x04;
    pkt[0x07] = 0xB0;
    pkt[0x08] = 0x11;
    pkt[0x0E] = 0x01;
    pkt[0x10] = 0x40;
    pkt[0x18] = 0x40;
    pkt[0x1A] = 0x40;
    pkt[0x1B] = 0x08;
    pkt[0x1C] = 0x43;
    pkt[0x1D] = 0x08;
    pkt[0x1E] = 0x40;
    pkt[0x24] = 0x01;
    pkt[0x26] = 0x30;
    pkt[0x28] = 0x40;
    pkt[0x2A] = 0x60;
    pkt[0x2C] = 0x60;
    pkt[0x2E] = 0x20;
    pkt[0x30] = 0x34;
    pkt[58] = 0x01;
    pkt[60] = 0x0E;
    pkt[62] = 0x1C;

    for zone_idx in 0..3 {
        let offset = zone_idx + 5;
        let (r, g, b) = colors[offset];
        pkt[(zone_idx * 22) + 11] = r;
        pkt[(zone_idx * 22) + 12] = g;
        pkt[(zone_idx * 22) + 13] = b;
    }

    pkt
}

/// Burst 4: zones 8-9 (ctrl-row, tab-row)
fn build_burst_four(colors: &[Color; 10]) -> [u8; PACKET_SIZE] {
    let mut pkt = new_packet();
    pkt[0x01] = 0x26;
    pkt[0x03] = 0x1C;
    pkt[0x05] = 0x0C;
    pkt[0x07] = 0x0C;
    pkt[0x09] = 0x0A;
    pkt[0x13] = 0x01;
    pkt[0x14] = 0x18;
    pkt[0x16] = 0x30;
    pkt[0x18] = 0x30;
    pkt[0x1A] = 0x30;
    pkt[0x1C] = 0x30;

    for zone_idx in 0..2 {
        let offset = zone_idx + 8;
        let (r, g, b) = colors[offset];
        pkt[(zone_idx * 22) + 16] = r;
        pkt[(zone_idx * 22) + 17] = g;
        pkt[(zone_idx * 22) + 18] = b;
    }

    pkt
}

/// Build a 10-slot color array from user-facing zone colors.
pub fn map_user_colors(
    user_colors: &[Color],
    variant: &Variant,
) -> [Color; 10] {
    let mut internal = [(0u8, 0u8, 0u8); 10];
    let mapping = variant.user_to_internal();
    for (user_idx, &color) in user_colors.iter().enumerate() {
        if user_idx < mapping.len() {
            internal[mapping[user_idx]] = color;
        }
    }
    internal
}

/// Generate rainbow colors for the given zone count.
pub fn rainbow_colors(zone_count: usize) -> Vec<Color> {
    (0..zone_count)
        .map(|i| {
            let hue = i as f64 / zone_count as f64;
            hsv_to_rgb(hue, 1.0, 1.0)
        })
        .collect()
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Color {
    let i = (h * 6.0).floor() as u32;
    let f = h * 6.0 - i as f64;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => unreachable!(),
    };

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variants::VARIANTS;

    #[test]
    fn init_packets_have_correct_headers() {
        let [p1, p2] = build_init_packets();
        assert_eq!(p1[0], 0x43);
        assert_eq!(p1[1], 0x81);
        assert_eq!(p1[3], 0x84);
        assert_eq!(p2[0], 0x43);
        assert_eq!(p2[1], 0x81);
        assert_eq!(p2[3], 0x86);
    }

    #[test]
    fn color_packets_place_rgb_correctly() {
        let tkl = &VARIANTS[0]; // TKL ANSI
        let mut colors = [(0u8, 0u8, 0u8); 10];
        colors[0] = (255, 0, 0); // zone 0: esc-row

        let pkts = build_color_packets(&colors, tkl);

        // Burst 1, zone 0: color at offsets 23,24,25
        assert_eq!(pkts[0][23], 255);
        assert_eq!(pkts[0][24], 0);
        assert_eq!(pkts[0][25], 0);
    }

    #[test]
    fn all_packets_start_with_report_id() {
        let tkl = &VARIANTS[0];
        let colors = [(128, 64, 32); 10];
        let pkts = build_color_packets(&colors, tkl);
        for pkt in &pkts {
            assert_eq!(pkt[0], 0x43);
        }
    }

    #[test]
    fn rainbow_produces_correct_count() {
        assert_eq!(rainbow_colors(8).len(), 8);
        assert_eq!(rainbow_colors(10).len(), 10);
    }

    #[test]
    fn mapping_byte_differs_for_iso() {
        let ansi = &VARIANTS[0]; // TKL ANSI
        let iso = &VARIANTS[1]; // TKL ISO
        let colors = [(255, 255, 255); 10];

        let ansi_pkts = build_color_packets(&colors, ansi);
        let iso_pkts = build_color_packets(&colors, iso);

        assert_eq!(ansi_pkts[0][0x0D], 0x01);
        assert_eq!(iso_pkts[0][0x0D], 0x03);
    }
}
