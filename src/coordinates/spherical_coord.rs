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

    /// Converts spherical coordinates to [cartesian coordinates](super::cartesian_coord::CartesianCoord)
    /// More info on [wikipedia](https://en.wikipedia.org/wiki/Spherical_coordinate_system#Cartesian_coordinates)
    /// # Examples
    /// ```
    /// let spherical_point = SphericalCoord::new(1, 0, 0);
    /// let cartesian_point = spherical_point.to_cartesian();
    /// ```
    pub fn to_cartesian(&self) -> CartesianCoord {
        let x = self.r * self.phi.cos() * self.theta.sin();
        let y = self.r * self.phi.sin() * self.theta.cos();
        let z = self.r * self.theta.cos();
        CartesianCoord::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use assert_approx_eq::assert_approx_eq;

    use super::{CartesianCoord, SphericalCoord};
    
    #[test]
    fn test_to_cartesian() {
        let result = SphericalCoord::new(-10.0 , 3.0 * PI, PI).to_cartesian();
        let exp_result = CartesianCoord::new(0.0, 0.0, 10.0);
        assert_approx_eq!(exp_result.x, result.x);
        assert_approx_eq!(exp_result.y, result.y);
        assert_approx_eq!(exp_result.z, result.z);
    }
}