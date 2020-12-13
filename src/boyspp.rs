use serde_json::json;
use serde_json::Value;

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
                "icon-image": "BOYPIL01",
                "icon-keep-upright": true,
                "symbol-placement": "point"
            }
        }),
        // json!({
        //     "id": "BOYSPP_point_orange",
        //     "type": "symbol",
        //     "source": "src_senc",
        //     "source-layer": "BOYSPP",
        //     "filter": [ "all",
        //     [ "==", "$type", "Point" ],
        //     // [ "==", "COLPAT", 1]
        //     ],
        //     "layout": {
        //         "icon-image": "BOYPIL81",
        //         "symbol-placement": "point"
        //     },
        //     "paint": {
        //         "text-color": *colors::TXT_FG,
        //         "text-halo-color": *colors::TXT_BG,
        //         "text-halo-width": 1.5
        //     }
        // })
    ]
}