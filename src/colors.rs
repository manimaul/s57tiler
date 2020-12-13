use std::include_bytes;
use serde_json::{Value, from_slice};
use crate::geojson_builder::JsonObject;

/// COLOUR Attribute
///     ID	Meaning	INT 1	S-4
///     1	white	IP 11.1;	450.2-3;
///     2	black
///     3	red	IP 11.2;	450.2-3;
///     4	green	IP 11.3;	450.2-3;
///     5	blue	IP 11.4;	450.2-3;
///     6	yellow	IP 11.6;	450.2-3;
///     7	grey
///     8	brown
///     9	amber	IP 11.8;	450.2-3;
///     10	violet	IP 11.5;	450.2-3;
///     11	orange	IP 11.7;	450.2-3;
///     12	magenta
///     13	pink
#[derive(Eq, PartialEq, Hash)]
pub enum Colour {
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Grey,
    Brown,
    Amber,
    Violet,
    Orange,
    Magenta,
    Pink,
}

impl Colour {
    pub fn from_value(properties: &JsonObject) -> Vec<Colour> {
        properties.get("COLOUR").and_then(|value| {
            value.as_array()
        }).map(|item| {
            item.iter().map(|ea| {
                match ea {
                    Value::String(n) => {
                        n.clone()
                    },
                    _ => {
                        panic!("unexpected value type for COLOUR");
                    }
                }
            }).map(|n| {
                match n.as_str() {
                    "1" => Some(Colour::White),
                    "2" => Some(Colour::Black),
                    "3" => Some(Colour::Red),
                    "4" => Some(Colour::Green),
                    "5" => Some(Colour::Blue),
                    "6" => Some(Colour::Yellow),
                    "7" => Some(Colour::Grey),
                    "8" => Some(Colour::Brown),
                    "9" => Some(Colour::Amber),
                    "10" => Some(Colour::Violet),
                    "11" => Some(Colour::Orange),
                    "12" => Some(Colour::Magenta),
                    "13" => Some(Colour::Pink),
                    _ => None,
                }
            }).filter(|ea| ea.is_some()).map(|ea| ea.unwrap()).collect::<Vec<Colour>>()
        }).unwrap_or(vec![])
    }
}

/// COLPAT Attribute
///     ID	Meaning	INT 1	S-4
///     1	horizontal stripes
///     2	vertical stripes
///     3	diagonal stripes
///     4	squared
///     5	stripes (direction unknown)
///     6	border stripe
#[derive(Eq, PartialEq, Hash)]
pub enum Colpat {
    HorizontalStripes,
    VerticalStripes,
    DiagonalStripes,
    Squared,
    Stripes,
    BorderStripe,
}

impl Colpat {
    pub fn from_value(properties: &JsonObject) -> Vec<Colpat> {
        properties.get("COLPAT").and_then(|value| {
            value.as_array()
        }).map(|item| {
            item.iter().map(|ea| {
                match ea {
                    Value::String(n) => {
                        n.clone()
                    },
                    _ => {
                        panic!("unexpected value type for COLPAT");
                    }
                }
            }).map(|n| {
                match n.as_str() {
                    "1" => Some(Colpat::HorizontalStripes),
                    "2" => Some(Colpat::VerticalStripes),
                    "3" => Some(Colpat::DiagonalStripes),
                    "4" => Some(Colpat::Squared),
                    "5" => Some(Colpat::Stripes),
                    "6" => Some(Colpat::BorderStripe),
                    _ => None,
                }
            }).filter(|ea| ea.is_some()).map(|ea| ea.unwrap()).collect::<Vec<Colpat>>()
        }).unwrap_or(vec![])
    }
}

const COLORS_JSON: &'static [u8] = include_bytes!("colors.json");

lazy_static! {
    pub static ref BG: &'static str = "#000";
    pub static ref TXT_BG: &'static str = "#000";
    pub static ref TXT_FG: &'static str = "#fff";
    pub static ref COLOR_KEYS: Vec<String> = ["day", "dusk", "dark"].iter().map(|&ea| ea.into()).collect();
    pub static ref COLORS: Value = from_slice(COLORS_JSON).unwrap();
}
