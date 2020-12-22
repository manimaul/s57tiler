use serde_json::json;
use serde_json::Value;
use geojson::Geometry;
use crate::geojson_builder::JsonObject;
use crate::colors::Colour;
use crate::util::compare;

/// LIGHTS, Light
/// Geometric primitives: P
///
/// Set Attribute_A: 	(!?)CATLIT; (!?)COLOUR; DATEND; DATSTA; EXCLIT; (?)HEIGHT; (!?)LITCHR; LITVIS; MARSYS; MLTYLT; NOBJNM; OBJNAM; (!?)ORIENT; PEREND; PERSTA; (!?)SECTR1; (!?)SECTR2; (!?)SIGGRP; (!?)SIGPER; (?)SIGSEQ; STATUS; VALNMR; VERACC; (?)VERDAT;
/// Set Attribute_B: 	INFORM; NINFOM; NTXTDS; SCAMAX; SCAMIN; TXTDSC;
/// Set Attribute_C: 	RECDAT; RECIND; SORDAT; SORIND;
///
/// Definition:
///     A luminous or lighted aid to navigation. (adapted from IHO Dictionary, S-32, 5th Edition, 2766)
/// References
///     INT 1:	IP 1-30.3, 40-65;
///     S-4:	470-473.5; 475-475.7; 476-478,5;
/// Remarks:
///     A light may be fixed on a buoy, beacon, tower etc. These are separate objects.
/// Distinction:
///     beacon, cardinal; beacon, isolated danger; beacon, lateral; beacon, safe water; beacon special purpose/general; buoy, cardinal; buoy, installation; buoy, isolated danger; buoy, lateral; buoy, safe water; buoy, special purpose/general; light vessel; light float;
pub fn layers() -> Vec<Value> {
    vec![
        json!({
            "id": "lights",
            "type": "symbol",
            "source": "src_senc",
            "source-layer": "LIGHTS",
            "filter": [ "any", [ "==", "$type", "Point" ] ],
            "layout": {
                "icon-image": ["get", "SY"],
                "icon-keep-upright": false,
                "icon-anchor": "top-left",
                "icon-allow-overlap": true,
                "symbol-placement": "point"
            }
        })
    ]
}

///Attribute type: L 	Used in: 	LIGHTS
///
/// Expected input:
///     ID	Meaning	INT 1	S-4
///     1	directional function	IP 30.1-3;	475.7;
///     2	rear/upper light
///     3	front/lower light
///     4	leading light	IP 20.1-3;	475.6;
///     5	aero light	IP 60;	476.1;
///     6	air obstruction light	IP 61.1-2	476.2;
///     7	fog detector light	IP 62;	477;
///     8	flood light	IP 63;	478.2;
///     9	strip light	IP 64;	478.5;
///     10	subsidiary light	IP 42;	471.8;
///     11	spotlight
///     12	front
///     13	rear
///     14	lower
///     15	upper
///     16	moire effect	IP 31;	475.8;
///     17	emergency
///     18	bearing light	 	478.1;
///     19	horizontally disposed
///     20	vertically disposed
/// Remarks:
///     Marine light (a light intended primarily for marine navigation) is not included in the above list. All lights are considered to be marine lights unless the attribute 'category of light' indicates otherwise.
pub enum Catlit {
    DirectionalFunction,
    RearUpperLight,
    FrontLowerLight,
    LeadingLight,
    AeroLight,
    AirObstructionLight,
    FogDetectorLight,
    FloodLight,
    StripLight,
    SubsidiaryLight,
    Spotlight,
    Front,
    Rear,
    Lower,
    Upper,
    MoireEffect,
    Emergency,
    BearingLight,
    HorizontallyDisposed,
    VerticallyDisposed,
}

impl Catlit {
    pub fn from_value(properties: &JsonObject) -> Vec<Catlit> {
        properties.get("CATLIT").and_then(|value| {
            value.as_array()
        }).map(|item| {
            item.iter().map(|ea| {
                match ea {
                    Value::String(n) => {
                        n.clone()
                    }
                    _ => {
                        panic!("unexpected value type for CATLIT");
                    }
                }
            }).map(|n| {
                match n.as_str() {
                    "1" => Some(Catlit::DirectionalFunction),
                    "2" => Some(Catlit::RearUpperLight),
                    "3" => Some(Catlit::FrontLowerLight),
                    "4" => Some(Catlit::LeadingLight),
                    "5" => Some(Catlit::AeroLight),
                    "6" => Some(Catlit::AirObstructionLight),
                    "7" => Some(Catlit::FogDetectorLight),
                    "8" => Some(Catlit::FloodLight),
                    "9" => Some(Catlit::StripLight),
                    "10" => Some(Catlit::SubsidiaryLight),
                    "11" => Some(Catlit::Spotlight),
                    "12" => Some(Catlit::Front),
                    "13" => Some(Catlit::Rear),
                    "14" => Some(Catlit::Lower),
                    "15" => Some(Catlit::Upper),
                    "16" => Some(Catlit::MoireEffect),
                    "17" => Some(Catlit::Emergency),
                    "18" => Some(Catlit::BearingLight),
                    "19" => Some(Catlit::HorizontallyDisposed),
                    "20" => Some(Catlit::VerticallyDisposed),
                    _ => None,
                }
            }).filter(|ea| ea.is_some()).map(|ea| ea.unwrap()).collect::<Vec<Catlit>>()
        }).unwrap_or(vec![])
    }
}

pub fn process_lights(geojson_geom: Geometry, properties: &mut JsonObject) -> Geometry {
    let catlit = Catlit::from_value(properties);
    let colors = Colour::from_value(properties);

    //todo: (WK) WIP
    let symbol = if compare(&vec![Colour::Red], &colors) {
        "LIGHTS11"
    } else if compare(&vec![Colour::Green], &colors) {
        "LIGHTS12"
    } else if compare(&vec![Colour::Yellow], &colors) {
        "LIGHTS13"
    } else {
        "LIGHTDEF"
    };

    properties.insert(String::from("SY"), Value::String(String::from(symbol)));
    geojson_geom
}