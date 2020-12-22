use serde_json::{Value, json};

/// Sea Area, SEAARE
/// Geometric primitives: Point, Area
///
/// Set Attribute_A: 	(!?)CATSEA; NOBJNM; (!?)OBJNAM;
/// Set Attribute_B: 	INFORM; NINFOM; NTXTDS; SCAMAX; SCAMIN; TXTDSC;
/// Set Attribute_C: 	RECDAT; RECIND; SORDAT; SORIND;
///
/// Definition:
///     A geographically defined part of the sea or other navigable waters. It may be specified within its limits by its proper name.
/// References
///     INT 1:	not specified;
///     S-4:	not specified;
/// Remarks:
///     Each sea area is defined independent of any other. Smaller sea areas may be located within larger sea areas.
/// Distinction:
///     depth area; seabed area;
pub fn layers(colors: &Value) -> Vec<Value> {
    vec![
        json!({
          "id": "SEAARE_fill",
          "type": "fill",
          "source": "src_senc",
          "source-layer": "SEAARE",
          "filter": [
            "any", [
              "==",
              "$type",
              "Polygon"
            ]
          ],
          "paint": {
            "fill-color": colors["CHWHT"]
          }
        }),
        // json!({
        //   "id": "SEAARE_point",
        //   "type": "symbol",
        //   "source": "src_senc",
        //   "source-layer": "SEAARE",
        //   // "filter": [ "any", [ ">", "CATSEA", 0 ] ],
        //   "layout": {
        //         "text-font": [ "Roboto Bold" ],
        //         "text-anchor": "center",
        //         "text-justify": "center",
        //         "text-field": ["get", "OBJNAM"],
        //         "text-allow-overlap": false,
        //         "text-ignore-placement": false,
        //         "text-max-width": 9,
        //         "text-size": 12,
        //         "text-padding": 6,
        //         "symbol-placement": "point"
        //       },
        //       "paint": {
        //         "text-color": *colors::TXT_FG,
        //         "text-halo-color": *colors::TXT_BG,
        //         "text-halo-width": 1.5
        //       }
        // })
    ]
}