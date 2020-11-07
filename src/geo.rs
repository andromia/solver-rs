/*
* 1. distances
*/
use crate::NodeType;
use std::vec::Vec;

pub type DistanceMatrixType = Vec<Vec<i32>>;

#[allow(dead_code)]
pub enum DistanceUnit {
    KM,
    ME, // meters
    MI,
    NM,
    FT,
    IN,
}

pub fn cacluate_haversine_between_nodes(
    node0: NodeType,
    node1: NodeType,
    dunit: &DistanceUnit,
) -> f32 {
    /*
     * Caculates distance between two geo coordinate pairs using haversine method.
     *
     * NOTE: can use tuples for lat, lon = loc unpacking
     */
    const AVG_EARTH_RADIUS: f32 = 6371.0088; // in km

    let radius = match dunit {
        DistanceUnit::KM => AVG_EARTH_RADIUS * 1.0,
        DistanceUnit::MI => AVG_EARTH_RADIUS * 1000.0,
        DistanceUnit::ME => AVG_EARTH_RADIUS * 0.621371192,
        DistanceUnit::NM => AVG_EARTH_RADIUS * 0.539956803,
        DistanceUnit::FT => AVG_EARTH_RADIUS * 3280.839895013,
        DistanceUnit::IN => AVG_EARTH_RADIUS * 39370.078740158,
    };

    let r_n00 = node0[0].to_radians();
    let r_n01 = node0[1].to_radians();
    let r_n10 = node1[0].to_radians();
    let r_n11 = node1[1].to_radians();

    let d = 2.0 * radius * ((r_n10 - r_n00) * 0.5).sin().powi(2)
        + r_n00.cos() * r_n10.cos() * ((r_n11 - r_n01) * 0.5).powi(2).sqrt().asin();

    return d;
}
