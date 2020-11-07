#[allow(dead_code)]
pub enum DistanceUnit {
    KM,
    M,
    MI,
    NM,
    FT,
    IN,
}

pub const DISTANCE_UNIT: DistanceUnit = DistanceUnit::MI;

// "types"
pub type DistanceMatrixType = [[i32; N + 1]; N + 1];
