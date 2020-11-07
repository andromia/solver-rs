use ndarray::arr2;

fn get_nodes() {
    arr2(&[[0., 0.], [0., 0.], [0., 0.]])
}

#[cfg(test)]
pub mod tests {
    use crate::common;
    use crate::geo;

    #[test]
    fn test_valid_calculate_haversine() {
        let origin = [0.0, 0.0];
        let dest = [0.0, 0.0];
        let unit = common::DistanceUnit::MI;

        let result = geo::cacluate_haversine(origin, dest, &unit);
        assert_eq!(result, 0.0);
    }
}
