use std::f64::consts::PI;

use super::cartesian_coord::CartesianCoord;

/// Spherical point with a radial distance (r), polar angle(theta) and azimuthal angle (phi) **in radians**
///
/// All values of the struct are public
///
/// For more info read [wikipedia](https://en.wikipedia.org/wiki/Spherical_coordinate_system)
#[derive(Debug)]
pub struct SphericalCoord {
    pub r: f64,
    pub theta: f64,
    pub phi: f64,
}

impl SphericalCoord {
    pub fn new(r: f64, theta: f64, phi: f64) -> SphericalCoord {
        SphericalCoord {
            r: r,
            theta: theta,
            phi: phi,
        }
    }
}

impl From<CartesianCoord> for SphericalCoord {
    /// Converts itself into a spherical coordinate
    /// # Examples
    /// ```
    /// let cartesianPoint = CartesianPoint::new(x: 10, y: 10, z: 10);
    /// let sphericalPoint = cartesianPoint.toSpherical();
    /// ```
    fn from(cart_coord: CartesianCoord) -> Self {
        let r = (cart_coord.x.powi(2) + cart_coord.y.powi(2) + cart_coord.z.powi(2)).sqrt();
        let theta = (cart_coord.z / r).acos();
        let mut phi = PI / 2_f64;
        if cart_coord.x > 0_f64 {
            phi = (cart_coord.y / cart_coord.x).atan();
        } else if cart_coord.x < 0_f64 {
            phi = (cart_coord.y / cart_coord.x).atan() + PI;
        }
        SphericalCoord::new(r, theta, phi)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use assert_approx_eq::assert_approx_eq;

    use super::{CartesianCoord, SphericalCoord};

    #[test]
    fn test_to_cartesian() {
        let result = CartesianCoord::from(SphericalCoord::new(-10.0, 3.0 * PI, PI));
        let exp_result = CartesianCoord::new(0.0, 0.0, 10.0);
        assert_approx_eq!(exp_result.x, result.x);
        assert_approx_eq!(exp_result.y, result.y);
        assert_approx_eq!(exp_result.z, result.z);
    }
}
