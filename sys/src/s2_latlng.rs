use super::S1Angle;
use super::S2LatLng;

impl S2LatLng {
    pub fn from_angle(lat: S1Angle, lng: S1Angle) -> Self {
        S2LatLng {
            coords_: [lat.radians_.to_bits(), lng.radians_.to_bits()],
        }
    }

    pub fn from_degrees(lat: f64, lng: f64) -> Self {
        Self::from_angle(S1Angle::from_degrees(lat), S1Angle::from_degrees(lng))
    }

    pub fn lat(&self) -> S1Angle {
        S1Angle {
            radians_: f64::from_bits(self.coords_[0]),
        }
    }

    pub fn lng(&self) -> S1Angle {
        S1Angle {
            radians_: f64::from_bits(self.coords_[1]),
        }
    }
}

#[test]
fn test_latlng() {
    unsafe {
        let latlng = S2LatLng::from_degrees(40.0, 20.0);
        let x = latlng.Normalized();

        println!("{:?}", x);
        println!("{:?} {:?}", x.lat(), x.lng())
    }
}
