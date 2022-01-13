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

    /// Calculates the euclidean distance between 2 cartesian points
    /// # Examples
    /// ```
    /// let point1 = CartesianCoord::new(0,0,0);
    /// let point2 = CartesianCoord::new(2,2,2);
    /// let distance = point1.euc_distance(point2);
    /// ```
    pub fn euc_distance(&self, other: CartesianCoord) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

impl From<SphericalCoord> for CartesianCoord {
    /// Converts spherical coordinates to [cartesian coordinates](super::cartesian_coord::CartesianCoord)
    /// More info on [wikipedia](https://en.wikipedia.org/wiki/Spherical_coordinate_system#Cartesian_coordinates)
    /// # Examples
    /// ```
    /// let spherical_point = SphericalCoord::new(1, 0, 0);
    /// let cartesian_point = spherical_point.to_cartesian();
    /// ```
    fn from(s_coord: SphericalCoord) -> Self {
        let x = s_coord.r * s_coord.phi.cos() * s_coord.theta.sin();
        let y = s_coord.r * s_coord.phi.sin() * s_coord.theta.cos();
        let z = s_coord.r * s_coord.theta.cos();
        CartesianCoord::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::CartesianCoord;
    use super::SphericalCoord;

    #[test]
    fn test_to_spherical() {
        let result = SphericalCoord::from(CartesianCoord::new(3.0, 4.0, 5.0));
        let exp_result = SphericalCoord::new(7.071067, 0.78539, 0.92729);
        assert_approx_eq!(exp_result.r, result.r);
        assert_approx_eq!(exp_result.theta, result.theta, 5_f64);
        assert_approx_eq!(exp_result.phi, result.phi, 5_f64);
    }

    #[test]
    fn test_euc_distance() {
        let point1 = CartesianCoord::new(0.0, 0.0, 0.0);
        let point2 = CartesianCoord::new(2.0, 2.0, 2.0);
        let result = point1.euc_distance(point2);
        let exp_result = 3.464102;
        assert_approx_eq!(exp_result, result);

        let point1 = CartesianCoord::new(0.0, 0.0, 0.0);
        let point2 = CartesianCoord::new(0.0, 0.0, 0.0);
        let result = point1.euc_distance(point2);
        let exp_result = 0.0;
        assert_approx_eq!(exp_result, result);

        let point1 = CartesianCoord::new(0.0, 0.0, 0.0);
        let point2 = CartesianCoord::new(-2.0, -2.0, -2.0);
        let result = point1.euc_distance(point2);
        let exp_result = 3.464102;
        assert_approx_eq!(exp_result, result);
    }
}
