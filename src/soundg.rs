use serde_json::{Value, json};
use geojson::Geometry;
use crate::geojson_builder::JsonObject;

static FEET: &str = "FEET";
static FATHOMS: &str = "FATHOMS";
static FATHOMS_FT: &str = "FATHOMS_FT";
static METERS: &str = "METERS";

struct Sounding {
    feet_display: i64,
    fathoms_display: i64,
    fathoms_feet_display: i64,
    meters_display: i64,
}

impl Sounding {

    fn from(depth_meters: f64) -> Sounding {
        let meters_display = depth_meters as i64;
        let feet_display = (depth_meters * 3.28084_f64) as i64;
        let fathoms = depth_meters * 0.546807_f64;
        let fathoms_display = fathoms as i64;
        let fathoms_feet_display = ((fathoms - (fathoms_display as f64)) * 6_f64) as i64;
        Sounding {
            feet_display,
            fathoms_display,
            fathoms_feet_display,
            meters_display,
        }
    }

    fn insert_into(&self, properties: &mut JsonObject) {
        properties.insert(String::from(FEET), json!(self.feet_display));
        properties.insert(String::from(FATHOMS), json!(self.fathoms_display));
        properties.insert(String::from(FATHOMS_FT), json!(self.fathoms_feet_display));
        properties.insert(String::from(METERS), json!(self.meters_display));
    }
}

/// Sounding SOUNDG
/// Geometric primitives: Point
///
/// Set Attribute_A: 	EXPSOU; NOBJNM; OBJNAM; QUASOU; (?)SOUACC; STATUS; (?)TECSOU; VERDAT;
/// Set Attribute_B: 	INFORM; NINFOM; NTXTDS; SCAMAX; SCAMIN; TXTDSC;
/// Set Attribute_C: 	RECDAT; RECIND; SORDAT; SORIND;
///
/// Definition:
///     A measured water depth or spot which has been reduced to a vertical datum (may be a drying height).
/// References
///     INT 1:	II 10-15;
///     S-4:	403.1; 410; 412-412.4; 413.1; 417.3;
/// Remarks:
///     The value of the sounding is encoded in the 3-D Coordinate field of the Spatial Record Structure (see S-57 Part 3).
///     Drying heights (drying soundings) are indicated by a negative value.
/// Distinction:
///     depth area; wreck; underwater/awash rock; obstruction;
pub fn layers(colors: &Value, depth: &String) -> Vec<Value> {
    match depth.as_str() {
        "fathoms" => vec![
            json!({
              "id": "soundg_fathoms",
              "type": "symbol",
              "source": "src_senc",
              "source-layer": "SOUNDG",
              "filter": [ "any", [ "==", "$type", "Point" ], ],
              "layout": {
                "text-font": [ "Roboto Bold" ],
                "text-anchor": "bottom-right",
                "text-justify": "center",
                "text-field": ["get", "FATHOMS"],
                "text-allow-overlap": true,
                "text-ignore-placement": true,
                "text-size": 11,
                "symbol-placement": "point"
              },
              "paint": {
                "text-color": ["case", ["<=", ["get", "METERS"], 9.0], colors["SNDG2"], colors["SNDG1"] ],
                "text-halo-color": colors["CHWHT"],
                "text-halo-width": 1.5
              }
            }),
            json!({
              "id": "soundg_fathoms_feet",
              "type": "symbol",
              "source": "src_senc",
              "source-layer": "SOUNDG",
              "filter": [
                "all",
                [ "==", "$type", "Point" ],
                [ "!=", "FATHOMS_FT", 0]
              ],
              "layout": {
                "text-font": [
                  "Roboto Bold"
                ],
                "text-anchor": "top-left",
                "text-offset": [0.1, -0.7],
                "text-justify": "center",
                "text-field": ["get", "FATHOMS_FT"],
                "text-allow-overlap": true,
                "text-ignore-placement": true,
                "text-size": 9,
                "symbol-placement": "point"
              },
              "paint": {
                "text-color": ["case", ["<=", ["get", "METERS"], 9.0], colors["SNDG2"], colors["SNDG1"] ]
              }
            })
        ],
        "feet" => vec![
            json!({
              "id": "soundg_feet",
              "type": "symbol",
              "source": "src_senc",
              "source-layer": "SOUNDG",
              "filter": [ "any", [ "==", "$type", "Point" ], ],
              "layout": {
                "text-font": [ "Roboto Bold" ],
                "text-anchor": "center",
                "text-justify": "center",
                "text-field": ["get", "FEET"],
                "text-allow-overlap": true,
                "text-ignore-placement": true,
                "text-size": 11,
                "symbol-placement": "point"
              },
              "paint": {
                "text-color": ["case", ["<=", ["get", "METERS"], 9.0], colors["SNDG2"], colors["SNDG1"] ],
                "text-halo-color": colors["CHWHT"],
                "text-halo-width": 1.5
              }
            })
        ],
        "meters" | _ => vec![
            json!({
              "id": "soundg_meters",
              "type": "symbol",
              "source": "src_senc",
              "source-layer": "SOUNDG",
              "filter": [ "any", [ "==", "$type", "Point" ], ],
              "layout": {
                "text-font": [ "Roboto Bold" ],
                "text-anchor": "center",
                "text-justify": "center",
                "text-field": ["get", "METERS"],
                "text-allow-overlap": true,
                "text-ignore-placement": true,
                "text-size": 11,
                "symbol-placement": "point"
              },
              "paint": {
                "text-color": ["case", ["<=", ["get", "METERS"], 9.0], colors["SNDG2"], colors["SNDG1"] ],
                "text-halo-color": colors["CHWHT"],
                "text-halo-width": 1.5
              }
            })
        ]
    }
}

pub fn process_sounding(geojson_geom: Geometry, properties: &mut JsonObject) -> Geometry {
    if let geojson::Value::Point(position) = &geojson_geom.value {
        let mut points = position.clone();
        //https://iho.int/uploads/user/pubs/standards/s-57/20ApB1.pdf
        let depth_meters = points[2];
        Sounding::from(depth_meters).insert_into(properties);
        points.drain(2..3);
        geojson::Geometry::new(geojson::Value::Point(points))
    } else {
        geojson_geom
    }
}

#[test]
fn test_soundings() {
    let mut subject = Sounding::from(0.9);
    assert_eq!(0_i64, subject.meters_display);
    assert_eq!(2_i64, subject.feet_display); //2.95276 ft
    assert_eq!(0_i64, subject.fathoms_display); //0.492126 fathoms
    assert_eq!(2_i64, subject.fathoms_feet_display); //0.492126 fathoms

    subject = Sounding::from(297.7);
    assert_eq!(297_i64, subject.meters_display);
    assert_eq!(976_i64, subject.feet_display); //976.70604 ft
    assert_eq!(162_i64, subject.fathoms_display); //162.78434 fathoms
    assert_eq!(4_i64, subject.fathoms_feet_display); //4.70604 fathoms
}