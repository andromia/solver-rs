pub mod geo;
mod tests;
use std::vec::Vec;

pub type NodeType = [f32; 2];

// playing with enums
#[allow(dead_code)]
pub enum IntPrecision {
    TEN,
    HUNDRED,
    THOUSAND,
}

// utils
pub fn get_int_precision(precision: &IntPrecision) -> i32 {
    return match precision {
        IntPrecision::TEN => 10,
        IntPrecision::HUNDRED => 100,
        IntPrecision::THOUSAND => 1000,
    };
}

pub fn convert_float_to_int(f: f32, precision: &IntPrecision) -> i32 {
    let p = get_int_precision(&precision);

    return (f * p as f32).round() as i32;
}

pub fn one_to_many_nodes_array(one: NodeType, many: Vec<NodeType>) -> Vec<NodeType> {
    /*
     * Takes NodeArray and NodesArray and creates NodesMatrix.
     *
     * [
     *    [f32, f32],
     *    [f32, f32],
     *    ... many.len() + 1
     * ]
     */
    let capacity: usize = many.len();
    let mut new_arr = Vec::with_capacity(capacity);

    new_arr[0] = one;
    for i in 1..new_arr.len() {
        new_arr[i] = [many[i - 1][0], many[i - 1][1]];
    }

    return new_arr;
}

pub fn nodes_arr_to_dist_int_matrix(
    nodes: Vec<NodeType>,
    dunit: geo::DistanceUnit,
    precision: IntPrecision,
) -> geo::DistanceMatrixType {
    /*
     * Takes a NodesArray a matrix of distances corresponding
     * to the order of the NodesArray.
     *
     * [
     *    [0, a_1, a_2, ... a_N],
     *    [b_0, 0, b_2, ... b_N],
     *    ... N
     * ]
     *
     */
    let capacity: usize = nodes.len();
    let mut new_matrix: geo::DistanceMatrixType = Vec::with_capacity(capacity);

    for i in 0..new_matrix.len() {
        for j in 0..new_matrix.len() {
            let node0 = nodes[i];
            let node1 = nodes[j];
            let dist = geo::cacluate_haversine_between_nodes(node0, node1, &dunit);

            new_matrix[i][j] = convert_float_to_int(dist, &precision);
        }
    }

    return new_matrix;
}
