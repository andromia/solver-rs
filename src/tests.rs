#[cfg(test)]
pub mod tests {
    use crate::common;

    #[test]
    fn test_valid_calculate_haversine() {
        let origin = [0.0, 0.0];
        let dest = [0.0, 0.0];
        let unit = common::DistanceUnit::MI;

        let result = crate::cacluate_haversine(origin, dest, &unit);
        assert_eq!(result, 0.0);
    }
}
