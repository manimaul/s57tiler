use serde_json::{Value, json};
use crate::constants;

pub fn tilejson() -> Value {
    let url = format!("http:/{}/v1/tile/{{z}}/{{x}}/{{y}}", constants::socket_address());
    json!(
    {
        "tiles": [
            url
        ],
        "scheme": "xyz",
        "format": "pbf",
        "minzoom": 0,
        "maxzoom": 30,
        "bounds": [
          -180,
          -85.05112877980659,
          180,
          85.0511287798066
        ],
        "tilejson": "2.2.0"
    }
    )
}