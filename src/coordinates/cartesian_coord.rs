use std::f64::consts::PI;

use super::spherical_coord::SphericalCoord;

/// Cartesian point with 3 coordinates, x, y and z
///
/// All values of the struct are public
#[derive(Debug)]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CartesianCoord {
    /// Creates a 3D cartesian coordinate
    /// # Arguments
    /// * ``x: f64`` - the x coordinate
    /// * ``y: f64`` - the y coordinate
    /// * ``z: f64`` - the z coordinate
    pub fn new(x: f64, y: f64, z: f64) -> CartesianCoord {
        CartesianCoord { x: x, y: y, z: z }
    }

    /// Converts itself into a spherical coordinate
    /// # Examples
    /// ```
    /// let cartesianPoint = CartesianPoint::new(x: 10, y: 10, z: 10);
    /// let sphericalPoint = cartesianPoint.toSpherical();
    /// ```
    pub fn to_spherical(&self) -> SphericalCoord {
        let r = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        let theta = (self.z / r).acos();
        let mut phi = PI / 2_f64;
        if self.x > 0_f64 {
            phi = (self.y / self.x).atan();
        } else if self.x < 0_f64 {
            phi = (self.y / self.x).atan() + PI;
        }
        SphericalCoord::new(r, theta, phi)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::CartesianCoord;
    use super::SphericalCoord;

    #[test]
    fn test_to_spherical() {
        let result = CartesianCoord::new(3.0, 4.0, 5.0).to_spherical();
        let exp_result = SphericalCoord::new(7.071067, 0.78539, 0.92729);
        assert_approx_eq!(exp_result.r, result.r);
        assert_approx_eq!(exp_result.theta, result.theta, 5_f64);
        assert_approx_eq!(exp_result.phi, result.phi, 5_f64);
    }
}
