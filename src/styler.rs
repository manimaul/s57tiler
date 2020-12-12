use std::path::Path;
use crate::{utils, soundg, seaare, depare, depcnt, boyspp, lights};
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
        for color in colors::COLOR_KEYS.iter() {
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

fn style_layers(depth: &String, color: &String) -> Value {
    let colors = match color.as_str() {
        "dusk" => colors::COLORS["DUSK"].clone(),
        "dark" => colors::COLORS["NIGHT"].clone(),
        _ => colors::COLORS["DAY_BRIGHT"].clone(),
    };
    let mut value = json!([
    {
      "id": "background",
      "type": "background",
      "paint": {
        "background-color": *colors::BG,
        "background-opacity": 1
      }
    }
    ]);
    if let Value::Array(ref mut items) = value {
        items.append(&mut seaare::layers(&colors));
        items.append(&mut depare::layers(&colors));
        items.append(&mut depcnt::layers(&colors));
        items.append(&mut todo_layers(&colors));
        items.append(&mut soundg::layers(&colors, &depth));
        items.append(&mut boyspp::layers());
        items.append(&mut lights::layers());
    };
    return value;
}

fn todo_layers(colors: &Value) -> Vec<Value> {
    vec![
        json!({
            "id": "SLCONS_line",
            "type": "line",
            "source": "src_senc",
            "source-layer": "SLCONS",
            "filter": [ "all" ],
            "paint": {
                "line-color": colors["CSTLN"],
                "line-width": 1
            }
        }),
        json!({
            "id": "PONTON_fill",
            "type": "fill",
            "source": "src_senc",
            "source-layer": "PONTON",
            "filter": [ "all", [ "==", "$type", "Polygon" ] ],
            "paint": {
                "fill-color": "#B7911F"
            }
        }),
        json!({
            "id": "PONTON_line",
            "type": "line",
            "source": "src_senc",
            "source-layer": "PONTON",
            "filter": [ "any", [ "==", "$type", "Polygon" ], [ "==", "$type", "LineString" ] ],
            "paint": {
                "line-color": colors["CSTLN"],
                "line-width": 1
            }
        }),
        json!({
            "id": "HULKES_fill",
            "type": "fill",
            "source": "src_senc",
            "source-layer": "HULKES",
            "filter": [ "any", [ "==", "$type", "Polygon" ] ],
            "paint": {
                "fill-color": "#B7911F"
            }
        }),
        json!({
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
        }),
        json!({
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
        }),
        json!({
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
        })
    ]
}
