use serde_json::{Value, json};

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