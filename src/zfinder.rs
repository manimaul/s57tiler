use gdal::spatial_ref::{SpatialRef, CoordTransform};

#[derive(Copy, Clone)]
struct GeoPoint {
    lat: f64,
    lng: f64,
}

fn cartesian_distance(origin: GeoPoint, destination: GeoPoint) -> f64 {
    let wgs84 = SpatialRef::from_epsg(4326).unwrap();
    let spherical_merc = SpatialRef::from_epsg(3785).unwrap();
    let transform = CoordTransform::new(&wgs84, &spherical_merc).unwrap();
    let mut ox = [origin.lng, 0.0];
    let mut oy = [origin.lat, 0.0];
    transform.transform_coords(&mut ox, &mut oy, &mut [0.0, 0.0]).expect("could not transform coord");

    let mut dx = [destination.lng, 0.0];
    let mut dy = [destination.lat, 0.0];
    transform.transform_coords(&mut dx, &mut dy, &mut [0.0, 0.0]).expect("could not transform coord");
    let dx = ox[0] - dx[0];
    let dy = oy[0] - dy[0];
    (dx.powi(2) + dy.powi(2)).sqrt()
}

fn haversine_distance(origin: GeoPoint, destination: GeoPoint) -> f64 {
    let earth_radius_km = 6371.0;
    let delta_lat = (destination.lat - origin.lat).to_radians();
    let delta_lng = (destination.lng - origin.lng).to_radians();
    let central_angle_inner = (delta_lat / 2.0).sin().powi(2)
        + origin.lat.to_radians().cos() * destination.lat.to_radians().cos()
        * (delta_lng / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();
    let distance = earth_radius_km * central_angle;
    distance * 1000_f64 // meters
}

fn latitude_distortion(latitude: f64) -> f64 {
    let origin = GeoPoint {
        lat: latitude,
        lng: 0.0,
    };
    let destination = GeoPoint {
        lat: latitude,
        lng: 1.0,
    };
    let hd = haversine_distance(origin, destination);
    let cd = cartesian_distance(origin, destination);
    return cd / hd;
}

fn get_zoom_for_true_scale(true_scale: f64) -> i64 {
    let mut z: i64 = 30; //max zoom
    let mut ts = true_scale;

    // SCAMIN values for features within an ENC should be set to either 1, 2, 3 or 4 steps smaller
    // scale than the compilation scale of the ENC.
    let step = 1_i64;

    while ts > 1.0 {
        ts /= 2.0;
        z -= 1;
    }
    z - step
}

//https://docs.mapbox.com/help/glossary/zoom-level/
pub fn find_zoom(scale: i64, latitude: f64) -> i64 {
    let distortion = latitude_distortion(latitude);
    //println!("distortion at latitude {} = {}", latitude, distortion);
    let true_scale = scale as f64 * distortion;
    get_zoom_for_true_scale(true_scale)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_zoom() {
        let scamin: i64 = 29999;
        let lat: f64 = 47.270827;
        let z = find_zoom(scamin, lat);
        assert_eq!(z, 13);
    }

    #[test]
    fn test_find_zoom_2() {
        let scamin: i64 = 29999;
        let lat: f64 = 0.0;
        let z = find_zoom(scamin, lat);
        assert_eq!(z, 14);
    }
}
