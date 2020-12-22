use serde_json::json;
use serde_json::Value;

/// DEPARE, Depth Area
/// Geometric primitives: L,A
///
/// Set Attribute_A: 	(!)DRVAL1; (!)DRVAL2; QUASOU; SOUACC; VERDAT;
/// Set Attribute_B: 	INFORM; NINFOM; NTXTDS; SCAMAX; SCAMIN; TXTDSC;
/// Set Attribute_C: 	RECDAT; RECIND; SORDAT; SORIND;
///
/// Definition:
///     A depth area is a water area whose depth is within a defined range of values.
/// References
///     INT 1:	not specified;
///     S-4:	not specified;
/// Remarks:
///     Intertidal areas are encoded as depth areas. These do not have to include soundings.
///     The depth range within a depth area is defined by the attributes 'DRVAL1' and 'DRVAL2'.
/// Distinction:
///     depth contour; dredged area; sounding; obstruction; sea area/named water area; unsurveyed area; wreck;
///
/// The geometric primitive line is removed for the object class, S-57 Supplement No. 3 (Edition 3.1.3), 3.3
pub fn layers(colors: &Value) -> Vec<Value> {
    vec![
        json!({
            "id": "DEPARE_fill_2",
            "type": "fill",
            "source": "src_senc",
            "source-layer": "DEPARE",
            "filter": ["all", ["==", "$type", "Polygon"], ["<=", "DRVAL1", 9.0]],
            "paint": {
                "fill-color": colors["DEPMD"]
            }
        }),
        json!({
            "id": "DEPARE_fill_1",
            "type": "fill",
            "source": "src_senc",
            "source-layer": "DEPARE",
            "filter": ["all", ["==", "$type", "Polygon"], ["<=", "DRVAL1", 3.0]],
            "paint": {
                "fill-color": colors["DEPVS"]
            }
        }),
        json!({
            "id": "DEPARE_fill_0",
            "type": "fill",
            "source": "src_senc",
            "source-layer": "DEPARE",
            "filter": ["all", ["==", "$type", "Polygon"], ["<", "DRVAL1", 0.0], ["<=", "DRVAL2", 0.0]],
            "paint": {
                "fill-color": colors["DEPIT"]
            }
        }),
        json!({
            "id": "DEPARE_line",
            "type": "line",
            "source": "src_senc",
            "source-layer": "DEPARE",
            "filter": ["any", ["==", "$type", "Polygon"], ["==", "$type", "LineString"]],
            "paint": {
                "line-color": colors["CSTLN"],
                "line-width": 0.5
            }
        })
    ]
}
