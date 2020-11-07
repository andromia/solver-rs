/*
* 1. distances
*/
use common;

pub fn cacluate_haversine(
    origin: [f32; 2],
    dest: [f32; 2],
    unit: &common::DistanceUnit,
) -> f32 {
    /*
     * Caculates distance between two geo coordinate pairs using haversine method.
     *
     * NOTE: can use tuples for lat, lon = loc unpacking
     */
    const AVG_EARTH_RADIUS: f32 = 6371.0088; // in km

    let radius = match unit {
        common::DistanceUnit::KM => AVG_EARTH_RADIUS * 1.0,
        common::DistanceUnit::MI => AVG_EARTH_RADIUS * 1000.0,
        common::DistanceUnit::M => AVG_EARTH_RADIUS * 0.621371192,
        common::DistanceUnit::NM => AVG_EARTH_RADIUS * 0.539956803,
        common::DistanceUnit::FT => AVG_EARTH_RADIUS * 3280.839895013,
        common::DistanceUnit::IN => AVG_EARTH_RADIUS * 39370.078740158,
    };

    let r_olat = origin[0].to_radians();
    let r_olon = origin[1].to_radians();
    let r_dlat = dest[0].to_radians();
    let r_dlon = dest[1].to_radians();

    let d = ((r_dlat - r_olat) * 0.5).sin().powi(2)
        + r_olat.cos() * r_dlat.cos() * ((r_dlon - r_olon) * 0.5).powi(2);

    return 2.0 * radius * d.sqrt().asin();
}
