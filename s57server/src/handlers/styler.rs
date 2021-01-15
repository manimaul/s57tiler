use s57tiler::{colors, seaare, depare, depcnt, soundg, boyspp, lights};
use crate::constants;
use serde_json::json;
use serde_json::Value;

fn depths() -> Vec<String> {
    ["fathoms", "meters", "feet"].iter().map(|&ea| ea.into()).collect()
}


/// https://docs.mapbox.com/mapbox-gl-js/style-spec/
pub fn create_style(depth: &String, color: &String) -> Option<Value> {
    if !depths().contains(depth) {
        info!("style depth not found: {}", &depth);
        return None
    }
    if !colors::COLOR_KEYS.contains(&color) {
        info!("style color not found: {}", &color);
        return None
    }
    let base_url = constants::external_base_url();
    let json_style = json!({
      "version": 8,
      "name": format!("{}-{}", color, depth),
      "sources": {
        "src_senc": {
          "type": "vector",
          "url": format!("{}/v1/tile_json", base_url)
        }
      },
      "sprite": format!("{}/res/sprites/rastersymbols-{}", base_url, color),
      "glyphs": format!("{}/res/fonts/{{fontstack}}/{{range}}.pbf", base_url),
      "layers": style_layers(depth, color)} );
    return Some(json_style);
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
