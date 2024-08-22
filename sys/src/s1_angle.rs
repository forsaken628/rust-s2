use super::S1Angle;

impl S1Angle {
    pub fn from_degrees(degrees: f64) -> Self {
        use std::f64::consts::PI;
        S1Angle {
            radians_: (PI / 180.0) * degrees,
        }
    }
}
