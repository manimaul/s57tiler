use serde_json::json;
use serde_json::Value;
use crate::geojson_builder::JsonObject;
use geojson::Geometry;
use crate::colors::{Colpat, Colour};
use crate::catspm::Catspm;

/// BOYSHP
///ID	Meaning	INT 1	S-4
/// 1	conical (nun, ogival)	IQ 20;	462.2
/// 2	can (cylindrical)	IQ 21;	462.3
/// 3	spherical	IQ 22;	462.4
/// 4	pillar	IQ 23;	462.5
/// 5	spar (spindle)	IQ 24;	462.6
/// 6	barrel (tun)	IQ 25;	462.7
/// 7	super-buoy	IQ 26;	462.9
/// 8	ice buoy
pub enum Boyshp {
    Conical,
    Can,
    Spherical,
    Pillar,
    Spar,
    Barrel,
    SuperBuoy,
    IceBuoy,
}

impl Boyshp {
    pub fn from_value(properties: &JsonObject) -> Option<Boyshp> {
        properties.get("BOYSHP").and_then(|value| {
            value.as_i64()
        }).and_then(|n| {
            match n {
                1_i64 => Some(Boyshp::Conical),
                2_i64 => Some(Boyshp::Can),
                3_i64 => Some(Boyshp::Spherical),
                4_i64 => Some(Boyshp::Pillar),
                5_i64 => Some(Boyshp::Spar),
                6_i64 => Some(Boyshp::Barrel),
                7_i64 => Some(Boyshp::SuperBuoy),
                8_i64 => Some(Boyshp::IceBuoy),
                _ => None,
            }
        })
    }
}

/// BOYSPP, Buoy Special Purpose / General
/// Geometric primitives: Point
///
/// Set Attribute_A: 	(!)BOYSHP; (!)CATSPM; (!)COLOUR; (!?)COLPAT; CONRAD; DATEND; DATSTA; (?)MARSYS; NATCON; NOBJNM; OBJNAM; PEREND; PERSTA; STATUS; VERACC; VERLEN;
/// Set Attribute_B: 	INFORM; NINFOM; NTXTDS; PICREP; SCAMAX; SCAMIN; TXTDSC;
/// Set Attribute_C: 	RECDAT; RECIND; SORDAT; SORIND;
///
/// Definition:
///     A buoy is a floating object moored to the bottom in a particular place, as an aid to navigation or for other specific purposes. (IHO Dictionary, S-32, 5th Edition, 565).
///     A special purpose buoy is primarily used to indicate an area or feature, the nature of which is apparent from reference to a chart, Sailing Directions or Notices to Mariners. (UKHO NP 735, 5th Edition)
///     Buoy in general: A buoy whose appearance or purpose is not adequately known.
/// References
///     INT 1:	IQ 130.6;
///     S-4:	461;
/// Remarks:
///     Topmark, light, fog signal, radar reflector and retro-reflector are separate objects.
/// Distinction:
///     buoy lateral; buoy safe water; buoy isolated danger; buoy cardinal; buoy installation; mooring/warping facility;
pub fn layers() -> Vec<Value> {
    vec![
        json!({
            "id": "BOYSPP_point",
            "type": "symbol",
            "source": "src_senc",
            "source-layer": "BOYSPP",
            "filter": [ "any", [ "==", "$type", "Point" ] ],
            "layout": {
                "icon-image": ["get", "SY"],
                "icon-anchor": "bottom",
                "icon-allow-overlap": true,
                "icon-keep-upright": true,
                "symbol-placement": "point"
            }
        })
    ]
}

fn compare<T : Eq>(lhs: &[T], rhs: &[T]) -> bool {
    (lhs.len() == rhs.len()) &&
        lhs.iter()
            .zip(rhs)
            .all(|(a,b)| a == b)
}

pub fn process_boyspp(geojson_geom: Geometry, properties: &mut JsonObject) -> Geometry {
    Boyshp::from_value(properties).map(|shape| {
        let pattern = Colpat::from_value(properties);
        let colors = Colour::from_value(properties);
        let symbol = match shape {
            Boyshp::Conical => {
                //todo: (WK) WIP
                "BOYCON01"
            }
            Boyshp::Can => {
                //todo: (WK) WIP
                "BOYCAN01"
            }
            Boyshp::Spherical => {
                //todo: (WK) WIP
                "BOYSPH01"
            }
            Boyshp::Pillar => {
                if compare(&vec![Colour::Red], &colors) {
                    "BOYPIL60"
                } else if compare(&vec![Colour::Green], &colors) {
                    "BOYPIL61"
                } else if compare(&vec![Colour::Yellow], &colors) {
                    "BOYPIL62"
                } else if compare(&vec![Colour::Black], &colors) {
                    "BOYPIL63"
                } else if compare(&vec![Colour::Orange], &colors) {
                    "BOYPIL64"
                } else if compare(&vec![Colour::Grey], &colors) {
                    "BOYPIL65"
                } else if compare(&vec![Colour::Red, Colour::Green, Colour::Red], &colors) {
                    "BOYPIL66"
                } else if compare(&vec![Colour::Green, Colour::Red, Colour::Green], &colors) {
                    "BOYPIL67"
                } else if compare(&vec![Colour::Black, Colour::Yellow], &colors) {
                    "BOYPIL68"
                } else if compare(&vec![Colour::Yellow, Colour::Black], &colors) {
                    "BOYPIL69"
                } else if compare(&vec![Colour::Yellow, Colour::Black, Colour::Yellow], &colors) {
                    "BOYPIL70"
                } else if compare(&vec![Colour::Black, Colour::Red, Colour::Black], &colors) {
                    "BOYPIL72"
                } else if compare(&vec![Colour::Red, Colour::White], &colors) {
                    if compare(&vec![Colpat::VerticalStripes], &pattern) {
                        "BOYPIL73"
                    } else {
                        "BOYPIL76"
                    }
                } else if compare(&vec![Colour::Red, Colour::Green], &colors) {
                    "BOYPIL74"
                } else if compare(&vec![Colour::Green, Colour::Red], &colors) {
                    "BOYPIL75"
                } else if compare(&vec![Colour::Green, Colour::White], &colors) {
                    "BOYPIL77"
                } else if compare(&vec![Colour::Red, Colour::White, Colour::Red, Colour::White], &colors) {
                    "BOYPIL78"
                } else if compare(&vec![Colour::Green, Colour::White, Colour::Green, Colour::White], &colors) {
                    "BOYPIL79"
                } else if compare(&vec![Colour::Red, Colour::Yellow], &colors) {
                    "BOYPIL80"
                } else if compare(&vec![Colour::White, Colour::Orange], &colors) {
                    "BOYPIL81"
                } else {
                    "BOYPIL01"
                }
            }
            Boyshp::Spar => {
                //todo: (WK) WIP
                "BOYSPR01"
            }
            Boyshp::Barrel => {
                //todo: (WK) WIP
                "BOYBAR01"
            }
            Boyshp::SuperBuoy => {
                if compare(&vec![Catspm::LargeAutomaticNavigationalBuoy], &Catspm::from_value(properties)) {
                    "BOYSUP03"
                } else {
                    "BOYSUP01"
                }
            }
            Boyshp::IceBuoy => {
                //todo: (WK) WIP
                "BOYSPR01"
            }
        };
        properties.insert(String::from("SY"), Value::String(String::from(symbol)));
    });
    geojson_geom
}