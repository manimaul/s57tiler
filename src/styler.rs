use std::path::Path;
use crate::utils;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

fn depths() -> Vec<String> {
    ["fathoms", "meters", "feet"].iter().map(|&ea| ea.into()).collect()
}

fn colors() -> Vec<String> {
    ["day", "dusk", "dark"].iter().map(|&ea| ea.into()).collect()
}

fn color_map_day() -> HashMap<String, String> {
    [
        ("NODTA", "rgba(163,180,183,1.0)"),
        ("CURSR", "rgba(235,125,54,1.0)"),
        ("CHBLK", "rgba(7,7,7,1.0)"),
        ("CHGRD", "rgba(125,137,140,1.0)"),
        ("CHGRF", "rgba(163,180,183,1.0)"),
        ("CHRED", "rgba(241,84,105,1.0)"),
        ("CHGRN", "rgba(104,228,86,1.0)"),
        ("CHYLW", "rgba(244,218,72,1.0)"),
        ("CHMGD", "rgba(197,69,195,1.0)"),
        ("CHMGF", "rgba(211,166,233,1.0)"),
        ("CHBRN", "rgba(177,145,57,1.0)"),
        ("CHWHT", "rgba(212,234,238,1.0)"),
        ("SCLBR", "rgba(235,125,54,1.0)"),
        ("CHCOR", "rgba(235,125,54,1.0)"),
        ("LITRD", "rgba(241,84,105,1.0)"),
        ("LITGN", "rgba(104,228,86,1.0)"),
        ("LITYW", "rgba(244,218,72,1.0)"),
        ("ISDNG", "rgba(197,69,195,1.0)"),
        ("DNGHL", "rgba(241,84,105,1.0)"),
        ("TRFCD", "rgba(197,69,195,1.0)"),
        ("TRFCF", "rgba(211,166,233,1.0)"),
        ("LANDA", "rgba(201,185,122,1.0)"),
        ("LANDF", "rgba(139,102,31,1.0)"),
        ("CSTLN", "rgba(82,90,92,1.0)"),
        ("SNDG1", "rgba(125,137,140,1.0)"),
        ("SNDG2", "rgba(7,7,7,1.0)"),
        ("DEPSC", "rgba(82,90,92,1.0)"),
        ("DEPCN", "rgba(125,137,140,1.0)"),
        ("DEPDW", "rgba(212,234,238,1.0)"),
        ("DEPMD", "rgba(186,213,225,1.0)"),
        ("DEPMS", "rgba(152,197,242,1.0)"),
        ("DEPVS", "rgba(115,182,239,1.0)"),
        ("DEPIT", "rgba(131,178,149,1.0)"),
        ("RADHI", "rgba(104,228,86,1.0)"),
        ("RADLO", "rgba(63,138,52,1.0)"),
        ("ARPAT", "rgba(63,165,111,1.0)"),
        ("NINFO", "rgba(235,125,54,1.0)"),
        ("RESBL", "rgba(58,120,240,1.0)"),
        ("ADINF", "rgba(178,159,52,1.0)"),
        ("RESGR", "rgba(125,137,140,1.0)"),
        ("SHIPS", "rgba(7,7,7,1.0)"),
        ("PSTRK", "rgba(7,7,7,1.0)"),
        ("SYTRK", "rgba(125,137,140,1.0)"),
        ("PLRTE", "rgba(220,64,37,1.0)"),
        ("APLRT", "rgba(235,125,54,1.0)"),
        ("UINFD", "rgba(7,7,7,1.0)"),
        ("UINFF", "rgba(125,137,140,1.0)"),
        ("UIBCK", "rgba(212,234,238,1.0)"),
        ("UIAFD", "rgba(115,182,239,1.0)"),
        ("UINFR", "rgba(241,84,105,1.0)"),
        ("UINFG", "rgba(104,228,86,1.0)"),
        ("UINFO", "rgba(235,125,54,1.0)"),
        ("UINFB", "rgba(58,120,240,1.0)"),
        ("UINFM", "rgba(197,69,195,1.0)"),
        ("UIBDR", "rgba(125,137,140,1.0)"),
        ("UIAFF", "rgba(201,185,122,1.0)"),
        ("OUTLW", "rgba(7,7,7,1.0)"),
        ("OUTLL", "rgba(201,185,122,1.0)"),
        ("RES01", "rgba(163,180,183,1.0)"),
        ("RES02", "rgba(163,180,183,1.0)"),
        ("RES03", "rgba(163,180,183,1.0)"),
        ("BKAJ1", "rgba(7,7,7,1.0)"),
        ("BKAJ2", "rgba(35,39,40,1.0)"),
    ].iter().map(|&ea| (ea.0.into(), ea.1.into())).collect()
}


/// https://tileserver.readthedocs.io/en/latest/config.html
pub fn create_config(out_dir: &Path, domain_list: Vec<String>) {
    utils::check_out_dir(out_dir);
    let config_json = json!(
{
  "options": {
    "paths": {
      "root": "",
      "fonts": "fonts",
      "sprites": "sprites",
      "styles": "styles",
      "mbtiles": ""
    },
    "domains": domain_list,
    "formatQuality": {
      "jpeg": 80,
      "webp": 90
    },
    "maxScaleFactor": 3,
    "maxSize": 2048,
    "pbfAlias": "pbf",
    "serveAllFonts": true,
    "serveAllStyles": true,
    "serveStaticMaps": true,
    "tileMargin": 0
  },
  "data": {
    "marine-chart": {
      "mbtiles": "chart.mbtiles"
    }
  }
}
    );
    utils::write_json(out_dir, "config.json", &config_json.to_string());
}

/// https://docs.mapbox.com/mapbox-gl-js/style-spec/
pub fn create_style(
    out_dir: &Path,
    base_url: &String,
) {
    utils::check_out_dir(out_dir);
    for depth in depths() {
        for color in colors() {
            let style_json = create_substyle(&base_url, &depth, &color);
            utils::write_json(out_dir, format!("{}_{}_style.json", color, depth).as_str(), &style_json.to_string());
        }
    }
}

fn create_substyle(base_url: &String, depth: &String, color: &String) -> Value {
    let json_style = json!({
      "version": 8,
      "name": format!("{}-{}", color, depth),
      "sources": {
        "src_senc": {
          "type": "vector",
          "url": format!("{}/data/marine-chart.json", base_url)
        }
      },
      "sprite": format!("rastersymbols-{}", color),
      "glyphs": format!("{}/fonts/{{fontstack}}/{{range}}.pbf", base_url),
      "layers": style_layers(depth, color)} );
    return json_style;
}

fn depth_layers(depth: &String) -> Vec<Value> {
    match depth.as_str() {
        "fathoms" => vec![
            json!({
              "id": "fathoms",
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
                "text-max-width": 9,
                "text-size": 10,
                "text-padding": 6,
                "symbol-placement": "point"
              },
              "paint": {
                "text-color": "#fff",
                "text-halo-color": "#000",
                "text-halo-width": 1.5
              }
            }),
            json!({
              "id": "fathoms_feet",
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
                "text-max-width": 9,
                "text-size": 10,
                "text-padding": 6,
                "symbol-placement": "point"
              },
              "paint": {
                "text-color": "#000"
              }
            })
        ],
        "feet" => vec![
            json!({
              "id": "feet",
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
                "text-max-width": 9,
                "text-size": 10,
                "text-padding": 6,
                "symbol-placement": "point"
              },
              "paint": {
                "text-color": "#fff",
                "text-halo-color": "#000",
                "text-halo-width": 1.5
              }
            })
        ],
        "meters" | _ => vec![
            json!({
              "id": "feet",
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
                "text-max-width": 9,
                "text-size": 10,
                "text-padding": 6,
                "symbol-placement": "point"
              },
              "paint": {
                "text-color": "#fff",
                "text-halo-color": "#000",
                "text-halo-width": 1.5
              }
            })
        ]
    }
}

fn style_layers(depth: &String, color: &String) -> Value {
    let colors = match color.as_str() {
        "day" => color_map_day(),
        _ => color_map_day() // todo: (WK
    };
    let mut value = json!([
    {
      "id": "background",
      "type": "background",
      "paint": {
        "background-color": "#000",
        "background-opacity": 1
      }
    },
    {
      "id": "SEAARE_fill",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "SEAARE",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ]
      ],
      "paint": {
        "fill-color": "#CEEAEE"
      }
    },
    {
      "id": "SEAARE_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "SEAARE",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ],
        [
          "==",
          "$type",
          "LineString"
        ]
      ],
      "paint": {
        "line-color": &colors["CHMGF"], //todo: (this is not correct) this is the color for "achare"
        "line-dasharray": [ 4, 2 ],
        "line-width": 1.5
      }
    },
    {
      "id": "DEPARE_fill_2",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], ["<=", "DRVAL1", 9.0]],
      "paint": {
        "fill-color": "#B4D6E3"
      }
    },
    {
      "id": "DEPARE_fill_1",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], ["<=", "DRVAL1", 3.0]],
      "paint": {
        "fill-color": "#5EB7F4"
      }
    },
    {
      "id": "DEPARE_fill_0",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], ["<", "DRVAL1", 0.0], ["<=", "DRVAL2", 0.0]],
      "paint": {
        "fill-color": "#75B493"
      }
    },
    {
      "id": "DEPARE_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], [">", "DRVAL2", 0.0]],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 0.5
      }
    },
    {
      "id": "SLCONS_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "SLCONS",
      "filter": [
        "all"
      ],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 1
      }
    },
    {
      "id": "PONTON_fill",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "PONTON",
      "filter": [ "all", [ "==", "$type", "Polygon" ] ],
      "paint": {
        "fill-color": "#B7911F"
      }
    },
    {
      "id": "PONTON_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "PONTON",
      "filter": [ "any", [ "==", "$type", "Polygon" ], [ "==", "$type", "LineString" ] ],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 1
      }
    },
    {
      "id": "HULKES_fill",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "HULKES",
      "filter": [ "any", [ "==", "$type", "Polygon" ] ],
      "paint": {
        "fill-color": "#B7911F"
      }
    },
    {
      "id": "HULKES_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "HULKES",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ],
        [
          "==",
          "$type",
          "LineString"
        ]
      ],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 1.5
      }
    },
    {
      "id": "LNDARE_fill",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "LNDARE",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ]
      ],
      "paint": {
        "fill-color": "#C9B97A"
      }
    },
    {
      "id": "LNDARE_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "LNDARE",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ],
        [
          "==",
          "$type",
          "LineString"
        ]
      ],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 2
      }
    },
    {
      "id": "BOYSPP_point",
      "type": "symbol",
      "source": "src_senc",
      "source-layer": "BOYSPP",
      "filter": [ "any", [ "==", "$type", "Point" ] ],
      "layout": {
        "text-font": [ "Roboto Bold" ],
        "text-anchor": "center",
        "text-justify": "center",
        "text-field": [ "get", "OBJNAM" ],
        "text-allow-overlap": true,
        "text-ignore-placement": true,
        "text-max-width": 9,
        "text-size": 10,
        "text-padding": 6,
        "symbol-placement": "point"
      },
      "paint": {
        "text-color": "#fff",
        "text-halo-color": "#000",
        "text-halo-width": 1.5
      }
    }
    ]);
    if let Value::Array(ref mut items) = value {
        let mut depth_rules = depth_layers(&depth);
        items.append(&mut depth_rules);
    };
    return value;
}
