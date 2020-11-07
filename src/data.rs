/*
* custom data parsing and wrangling. includes csv parsing
*/
use common;

pub fn create_geo_matrix(
    olat: f32,
    olon: f32,
    dlats: [f32; common::N],
    dlons: [f32; common::N],
) -> [[f32; 2]; common::N + 1] {
    /*
     * Takes origin and dest lats and lons and creates matrix of geocode pairs.
     *
     * [
     *    [olat, olon],
     *    [dlat_0, dlon_0],
     *    ... N + 1
     * ]
     */
    let mut geo_matrix = [[0.0, 0.0]; common::N + 1];

    geo_matrix[0] = [olat, olon];
    for i in 1..geo_matrix.len() {
        geo_matrix[i] = [dlats[i - 1], dlons[i - 1]];
    }

    return geo_matrix;
}

pub fn create_distance_matrix(
    geo_matrix: [[f32; 2]; common::N + 1],
) -> common::DistanceMatrixType {
    /*
     * Takes origin lat and lon and destination lats and lons and returns
     * a matrix of distances.
     *
     * [
     *    [0, a_1, a_2, ... a_N],
     *    [b_0, 0, b_2, ... b_N],
     *    ... N
     * ]
     *
     * TODO:
     *   - Refactor to use new_arr.len() for iterator
     */

    // TODO: don't want 0 initial values
    let unit = common::DISTANCE_UNIT;
    let mut dist_matrix: common::DistanceMatrixType = [[0; common::N + 1]; common::N + 1];

    for i in 0..dist_matrix.len() {
        for j in 0..dist_matrix.len() {
            let origin = geo_matrix[i];
            let dest = geo_matrix[j];

            dist_matrix[i][j] = (cacluate_haversine(origin, dest, &unit)
                * common::INT_PRECISION as f32)
                .round() as i32;
        }
    }

    return dist_matrix;
}
