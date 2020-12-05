use gdal;
use gdal::vector::FieldValue;
use serde_json::{json, Value, Map, Number};
use serde_json;
use gdal::spatial_ref::SpatialRef;

type JsonObject = Map<String, Value>;

static SOUNDG: &str = "SOUNDG";
static FEET: &str = "FEET";
static FATHOMS: &str = "FATHOMS";
static FATHOMS_FT: &str = "FATHOMS_FT";
static METERS: &str = "METERS";

struct Sounding {
    feet_display: i64,
    fathoms_display: i64,
    fathoms_feet_display: i64,
    meters_display: i64,
}

impl Sounding {

    fn from(depth_meters: f64) -> Sounding {
        let meters_display = depth_meters as i64;
        let feet_display = (depth_meters * 3.28084_f64) as i64;
        let fathoms = depth_meters * 0.546807_f64;
        let fathoms_display = fathoms as i64;
        let fathoms_feet_display = ((fathoms - (fathoms_display as f64)) * 6_f64) as i64;
        Sounding {
            feet_display,
            fathoms_display,
            fathoms_feet_display,
            meters_display,
        }
    }

    fn insert_into(&self, properties: &mut JsonObject) {
        properties.insert(String::from(FEET), json!(self.feet_display));
        properties.insert(String::from(FATHOMS), json!(self.fathoms_display));
        properties.insert(String::from(FATHOMS_FT), json!(self.fathoms_feet_display));
        properties.insert(String::from(METERS), json!(self.meters_display));
    }
}

fn gdal_feature_to_geojson_feature(
    feature: &gdal::vector::Feature,
    target_sr: &SpatialRef,
    is_sounding: bool
) -> Option<geojson::Feature> {
    feature.geometry_by_index(0).ok().and_then(|g| {
        if g.has_gdal_ptr() {
            if g.spatial_ref().unwrap().auth_code() != target_sr.auth_code() {
                println!("performing coordinate transform");
                g.transform_to(target_sr).and_then(|tg| tg.json())
            } else {
                g.json()
            }.ok().and_then(|json_str| {
                let json_value = serde_json::from_str(json_str.as_str()).unwrap();
                let mut properties = gdal_feature_properties(&feature);
                let geometry = geojson::Geometry::from_json_object(json_value)
                    .map(|geojson_geom| {
                        if is_sounding {
                            if let geojson::Value::Point(position) = &geojson_geom.value {
                                let mut points = position.clone();
                                //https://iho.int/uploads/user/pubs/standards/s-57/20ApB1.pdf
                                let depth_meters = points[2];
                                Sounding::from(depth_meters).insert_into(&mut properties);
                                points.drain(2..3);
                                return geojson::Geometry::new(geojson::Value::Point(points))
                            }
                        }
                        geojson_geom
                    })
                    .ok();
                Some(geojson::Feature {
                    bbox: None,
                    geometry,
                    id: None,
                    properties: Some(properties),
                    foreign_members: None
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
        },
        FieldValue::RealValue(v) => Some(Value::Number(Number::from_f64(v.clone()).unwrap())),
        FieldValue::RealListValue(v) => Some(Value::Array(v.iter().map(|ea|Value::Number(Number::from_f64(ea.clone()).unwrap())).collect())),
        FieldValue::Integer64Value(v) => Some(Value::Number(Number::from(v.clone()))),
        FieldValue::IntegerListValue(v) => Some(Value::Array(v.iter().map(|ea|Value::Number(Number::from(ea.clone()))).collect())),
        FieldValue::Integer64ListValue(v) => Some(Value::Array(v.iter().map(|ea|Value::Number(Number::from(ea.clone()))).collect())),
        FieldValue::StringListValue(v) => Some(Value::Array(v.iter().map(|ea|Value::String(ea.clone())).collect())),
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
    let is_sounding = layer.name().eq(SOUNDG);
    let features: Vec<geojson::Feature> = layer.features().into_iter().map(|f| {
        gdal_feature_to_geojson_feature(&f, target_sr, is_sounding)
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

#[test]
fn test_soundings() {
    let mut subject = Sounding::from(0.9);
    assert_eq!(0_i64, subject.meters_display);
    assert_eq!(2_i64, subject.feet_display); //2.95276 ft
    assert_eq!(0_i64, subject.fathoms_display); //0.492126 fathoms
    assert_eq!(2_i64, subject.fathoms_feet_display); //0.492126 fathoms

    subject = Sounding::from(297.7);
    assert_eq!(297_i64, subject.meters_display);
    assert_eq!(976_i64, subject.feet_display); //976.70604 ft
    assert_eq!(162_i64, subject.fathoms_display); //162.78434 fathoms
    assert_eq!(4_i64, subject.fathoms_feet_display); //4.70604 fathoms
}
