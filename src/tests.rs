#[allow(dead_code)]
#[cfg(test)]
pub mod tests {
    use crate::NodeType;
    use geo;

    const DISTANCE_UNIT: geo::DistanceUnit = geo::DistanceUnit::MI;

    // TODO: update with test data
    const N: usize = 10;
    const NODES: [NodeType; N] = [
        [0., 0.],
        [0., 0.],
        [0., 0.],
        [0., 0.],
        [0., 0.],
        [0., 0.],
        [0., 0.],
        [0., 0.],
        [0., 0.],
        [0., 0.],
    ];

    #[test]
    fn test_valid_calculate_haversine() {
        let node0 = [0.0, 0.0];
        let node1 = [0.0, 0.0];
        let dunit = DISTANCE_UNIT;

        let result = geo::cacluate_haversine_between_nodes(node0, node1, &dunit);
        assert_eq!(result, 0.0);
    }
}
