pub const VID: u16 = 0x1E71;

pub struct Variant {
    pub name: &'static str,
    pub pid: u16,
    pub zone_count: usize,
    pub zone_names: &'static [&'static str],
    /// User-friendly labels: index, name, description
    pub zone_labels: &'static [&'static str],
    /// Mapping byte at offset 0x0D in color burst 1 (ANSI=0x01, ISO=0x03)
    pub mapping_byte: u8,
}

// TKL: 8 usable zones (internal indices 0,1,2,3,6,7,8,9 — 4,5 are null)
const TKL_ZONE_NAMES: &[&str] = &[
    "esc-row",
    "number-row",
    "tab-row",
    "caps-row",
    "shift-row",
    "ctrl-row",
    "nav-cluster",
    "arrows",
];

const TKL_ZONE_LABELS: &[&str] = &[
    "0: esc-row      - Esc, F1-F6 area",
    "1: number-row   - F7-F12, number row",
    "2: tab-row      - Tab row",
    "3: caps-row     - Caps Lock row",
    "4: shift-row    - Shift row",
    "5: ctrl-row     - Ctrl, Alt, Space row",
    "6: nav-cluster  - Ins, Del, PgUp, PgDn",
    "7: arrows       - Arrow keys area",
];

// Full-size: 10 zones (all internal indices used)
const FULL_ZONE_NAMES: &[&str] = &[
    "esc-row",
    "number-row",
    "tab-row",
    "caps-row",
    "shift-row",
    "ctrl-row",
    "nav-cluster",
    "arrows",
    "numpad-upper",
    "numpad-lower",
];

const FULL_ZONE_LABELS: &[&str] = &[
    "0: esc-row       - Esc, F1-F6 area",
    "1: number-row    - F7-F12, number row",
    "2: tab-row       - Tab row",
    "3: caps-row      - Caps Lock row",
    "4: shift-row     - Shift row",
    "5: ctrl-row      - Ctrl, Alt, Space row",
    "6: nav-cluster   - Ins, Del, PgUp, PgDn",
    "7: arrows        - Arrow keys area",
    "8: numpad-upper  - Numpad upper half",
    "9: numpad-lower  - Numpad lower half",
];

pub const VARIANTS: &[Variant] = &[
    // ── NZXT Function (original, 2022) ────────────────────────────────────────
    Variant {
        name: "Function TKL (ANSI)",
        pid: 0x2104,
        zone_count: 8,
        zone_names: TKL_ZONE_NAMES,
        zone_labels: TKL_ZONE_LABELS,
        mapping_byte: 0x01,
    },
    Variant {
        name: "Function TKL (ISO)",
        pid: 0x2107,
        zone_count: 8,
        zone_names: TKL_ZONE_NAMES,
        zone_labels: TKL_ZONE_LABELS,
        mapping_byte: 0x03,
    },
    Variant {
        name: "Function Full-size (ANSI)",
        pid: 0x2103,
        zone_count: 10,
        zone_names: FULL_ZONE_NAMES,
        zone_labels: FULL_ZONE_LABELS,
        mapping_byte: 0x01,
    },
    Variant {
        name: "Function Full-size (ISO)",
        pid: 0x2106,
        zone_count: 10,
        zone_names: FULL_ZONE_NAMES,
        zone_labels: FULL_ZONE_LABELS,
        mapping_byte: 0x03,
    },
    Variant {
        name: "Function MiniTKL (ANSI)",
        pid: 0x2105,
        zone_count: 8,
        zone_names: TKL_ZONE_NAMES,
        zone_labels: TKL_ZONE_LABELS,
        mapping_byte: 0x01,
    },
    Variant {
        name: "Function MiniTKL (ISO)",
        pid: 0x2108,
        zone_count: 8,
        zone_names: TKL_ZONE_NAMES,
        zone_labels: TKL_ZONE_LABELS,
        mapping_byte: 0x03,
    },
    // ── NZXT Function 2 (2023–2024) ───────────────────────────────────────────
    // PID 0x2137: confirmed on Function 2 MiniTKL ISO (German layout, bcdDevice 1.03).
    // The Function 2 exposes 3 HID interfaces; the LED control interface is
    // still interface 1 with report ID 0x43 / 64-byte reports — same as Function 1.
    //
    // ANSI variant PID: unknown (add here once observed; likely 0x213x or similar).
    // Full-size variant PIDs: unknown (add here once observed).
    Variant {
        name: "Function 2 MiniTKL (ISO)",
        pid: 0x2137,
        zone_count: 8,
        zone_names: TKL_ZONE_NAMES,
        zone_labels: TKL_ZONE_LABELS,
        mapping_byte: 0x03,
    },
];

/// Map user-facing zone index (0-7 or 0-9) to internal protocol zone index.
/// Internal layout: [0,1,2,3,4(null),5(null),6,7,8,9]
/// User layout (TKL): esc-row=0, number-row=1, tab-row=9, caps-row=2,
///                     shift-row=3, ctrl-row=8, nav-cluster=7, arrows=6
pub const USER_TO_INTERNAL_TKL: &[usize] = &[0, 1, 9, 2, 3, 8, 7, 6];

/// Full-size adds numpad-upper=4, numpad-lower=5
pub const USER_TO_INTERNAL_FULL: &[usize] = &[0, 1, 9, 2, 3, 8, 7, 6, 4, 5];

impl Variant {
    pub fn user_to_internal(&self) -> &'static [usize] {
        if self.zone_count == 10 {
            USER_TO_INTERNAL_FULL
        } else {
            USER_TO_INTERNAL_TKL
        }
    }

    pub fn zone_name_to_user_index(&self, name: &str) -> Option<usize> {
        self.zone_names.iter().position(|&n| n == name)
    }
}