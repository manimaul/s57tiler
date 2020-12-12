use std::include_bytes;
use serde_json::{Value, from_slice};

// COLOUR
//Attribute type: L 	Used in:
//
// Expected input:
//     ID	Meaning	INT 1	S-4
//     1	white	IP 11.1;	450.2-3;
//     2	black
//     3	red	IP 11.2;	450.2-3;
//     4	green	IP 11.3;	450.2-3;
//     5	blue	IP 11.4;	450.2-3;
//     6	yellow	IP 11.6;	450.2-3;
//     7	grey
//     8	brown
//     9	amber	IP 11.8;	450.2-3;
//     10	violet	IP 11.5;	450.2-3;
//     11	orange	IP 11.7;	450.2-3;
//     12	magenta
//     13	pink
// Remarks:
//     No remarks.

// COLPAT
// Attribute type: L 	Used in:
//
// Expected input:
//     ID	Meaning	INT 1	S-4
//     1	horizontal stripes
//     2	vertical stripes
//     3	diagonal stripes
//     4	squared
//     5	stripes (direction unknown)
//     6	border stripe
// Remarks:
//     No remarks.

const COLORS_JSON: &'static [u8] = include_bytes!("colors.json");

lazy_static! {
    pub static ref BG: &'static str = "#000";
    pub static ref TXT_BG: &'static str = "#000";
    pub static ref TXT_FG: &'static str = "#fff";
    pub static ref COLOR_KEYS: Vec<String> = ["day", "dusk", "dark"].iter().map(|&ea| ea.into()).collect();
    pub static ref COLORS: Value = from_slice(COLORS_JSON).unwrap();
}
