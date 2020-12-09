use std::path::Path;
use crate::utils;
use crate::colors;
use serde_json::json;
use serde_json::Value;

fn depths() -> Vec<String> {
    ["fathoms", "meters", "feet"].iter().map(|&ea| ea.into()).collect()
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
        for color in colors::color_keys() {
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
        "dusk" => colors::colors()["DUSK"].clone(),
        "dark" => colors::colors()["NIGHT"].clone(),
        _ => colors::colors()["DAY_BRIGHT"].clone(),
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
        "line-color": colors["CHMGF"], //todo: (this is not correct) this is the color for "achare"
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
        "fill-color": colors["DEPVS"]
      //   blue 5EB7F4
      }
    },
    {
      "id": "DEPARE_fill_0",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], ["<", "DRVAL1", 0.0], ["<=", "DRVAL2", 0.0]],
      "paint": {
        "fill-color": colors["DEPIT"]
        // green 75B493
      }
    },
    {
      "id": "DEPARE_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], [">", "DRVAL2", 0.0]],
      "paint": {
        "line-color": colors["CSTLN"],
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
        "line-color": colors["CSTLN"],
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
        "line-color": colors["CSTLN"],
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
        "line-color": colors["CSTLN"],
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
        "fill-color": colors["LANDA"]
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
        "line-color": colors["CSTLN"],
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
