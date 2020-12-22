use gdal;
use gdal::vector::FieldValue;
use serde_json::{Value, Map, Number, json};
use serde_json;
use gdal::spatial_ref::SpatialRef;
use crate::{soundg, boyspp, lights};
use crate::zfinder::find_zoom;

pub type JsonObject = Map<String, Value>;

pub fn process_scamin(
    center_lat: f64,
    properties: &mut JsonObject,
) {
    match properties.get("SCAMIN").and_then(|value| {
        value.as_i64()
    }).filter(|scamin| *scamin >= 0).and_then(|scamin: i64| {
        let min_z = find_zoom(scamin, center_lat);
        Some(min_z)
    }) {
        None => {
            properties.insert(String::from("MINZ"), json!(30_i64));
        }
        Some(min_z) => {
            properties.insert(String::from("MINZ"), json!(min_z));
        }
    }
}

fn geometry_json_center(
    geometry: &gdal::vector::Geometry
) -> Option<(String, f64)> {
    geometry.json().ok().and_then(|json| {
        geometry.centroid().map(|c| {
            (json, c.get_point(0).1)
        })
    })
}

fn gdal_feature_to_geojson_feature(
    feature: &gdal::vector::Feature,
    target_sr: &SpatialRef,
    layer_name: &String,
) -> Option<geojson::Feature> {
    feature.geometry_by_index(0).ok().and_then(|g| {
        if g.has_gdal_ptr() {
            if g.spatial_ref().unwrap().auth_code() != target_sr.auth_code() {
                println!("performing coordinate transform");
                g.transform_to(target_sr).ok().and_then(|tg| geometry_json_center(&tg))
            } else {
                geometry_json_center(g)
            }.and_then(|(json_str, lat)| {
                let json_value = serde_json::from_str(json_str.as_str()).unwrap();
                let mut properties = gdal_feature_properties(&feature);
                process_scamin(lat, &mut properties);
                let geometry = geojson::Geometry::from_json_object(json_value)
                    .map(|geojson_geom| {
                        match layer_name.as_str() {
                            "SOUNDG" => soundg::process_sounding(geojson_geom, &mut properties),
                            "BOYSPP" => boyspp::process_boyspp(geojson_geom, &mut properties),
                            "LIGHTS" => lights::process_lights(geojson_geom, &mut properties),
                            _ => geojson_geom
                        }
                    })
                    .ok();
                Some(geojson::Feature {
                    bbox: None,
                    geometry,
                    id: None,
                    properties: Some(properties),
                    foreign_members: None,
                })
            })
        } else {
            None
        }
    })
}

fn field_value_to_json_value(fv: &FieldValue) -> Option<Value> {
    match fv {
        FieldValue::IntegerValue(v) => Some(Value::Number(Number::from(v.clone()))),
        FieldValue::StringValue(v) => {
            if v.is_empty() {
                None
            } else {
                Some(Value::String(v.clone()))
            }
        }
        FieldValue::RealValue(v) => Some(Value::Number(Number::from_f64(v.clone()).unwrap())),
        FieldValue::RealListValue(v) => Some(Value::Array(v.iter().map(|ea| Value::Number(Number::from_f64(ea.clone()).unwrap())).collect())),
        FieldValue::Integer64Value(v) => Some(Value::Number(Number::from(v.clone()))),
        FieldValue::IntegerListValue(v) => Some(Value::Array(v.iter().map(|ea| Value::Number(Number::from(ea.clone()))).collect())),
        FieldValue::Integer64ListValue(v) => Some(Value::Array(v.iter().map(|ea| Value::Number(Number::from(ea.clone()))).collect())),
        FieldValue::StringListValue(v) => Some(Value::Array(v.iter().map(|ea| Value::String(ea.clone())).collect())),
        // FieldValue::DateValue(_) => Value::String(String::from("")),
        // FieldValue::DateTimeValue(_) => Value::String(String::from(""))
    }
}

fn gdal_feature_properties(feature: &gdal::vector::Feature) -> JsonObject {
    let mut props = JsonObject::new();
    feature.fields().for_each(|each| {
        if let Some(v) = field_value_to_json_value(&each.1) {
            let name: String = each.0.clone();
            props.insert(name, v);
        };
    });
    props
}

pub fn feature_collection_from_layer(layer: &gdal::vector::Layer, target_sr: &SpatialRef) -> Option<geojson::FeatureCollection> {
    let features: Vec<geojson::Feature> = layer.features().into_iter().map(|f| {
        gdal_feature_to_geojson_feature(&f, target_sr, &layer.name())
    }).filter(|f| f.is_some()).map(|f| f.unwrap()).collect();

    if features.is_empty() {
        None
    } else {
        Some(geojson::FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;
    use std::env;

    #[test]
    fn test_gdal_feature_to_geojson_feature() {
        let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let chart = Path::new(&out_dir)
            .parent().unwrap()
            .join("data")
            .join("charts")
            .join("US5WA22M")
            .join("US5WA22M.000");
        let mut ds = gdal::Dataset::open(&chart).unwrap();
        let name = String::from("BOYSPP");
        let layer = ds.layer_by_name(name.as_str()).unwrap();
        let feat = layer.features().nth(0).unwrap();
        let sr = SpatialRef::from_epsg(4326).unwrap();
        let gf = gdal_feature_to_geojson_feature(&feat, &sr, &name);
        assert!(gf.is_some())
    }
}


