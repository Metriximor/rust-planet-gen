//! Creates a evenly-ish distanced set of points in a spherical shape
use std::f64::consts::PI;

use super::{cartesian_coord::CartesianCoord, spherical_coord::SphericalCoord};

/// Set of points in a sphere
pub struct FibonacciSphere {
    seed: u64,
    jitter: f32,
    number_points: u16,
    points: Vec<SphericalCoord>,
}

impl FibonacciSphere {
    fn new(number_points: u16, jitter: f32, seed: u64) -> FibonacciSphere {
        FibonacciSphere {
            seed: seed,
            jitter: jitter,
            number_points: number_points,
            points: FibonacciSphere::generate_points(number_points),
        }
    }

    /// Get a reference to the fibonacci sphere's points.
    pub fn points(&self) -> &[SphericalCoord] {
        self.points.as_ref()
    }

    /// Generates the fibonnaci sphere points
    /// Using [golden selection](http://web.archive.org/web/20120421191837/http://www.cgafaq.info/wiki/Evenly_distributed_points_on_sphere)
    ///
    /// # Todo
    ///
    /// * Implement Jitter based on seed
    fn generate_points(number_points: u16) -> Vec<SphericalCoord> {
        let mut points = Vec::with_capacity(usize::from(number_points));
        let dlong = PI * (3_f64 - 5_f64.sqrt());
        let dz = 2.0 / f64::from(number_points);
        let mut long: f64 = 0.0;
        let mut z = 1.0 - dz / 2.0;
        for _k in 0..number_points - 1 {
            let r = (1.0 - z * z).sqrt();
            let cart = CartesianCoord::new(long.cos() * r, long.sin() * r, z);
            z -= dz;
            long += dlong;
            points.push(cart.into());
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::FibonacciSphere;

    #[test]
    fn test_create_sphere() {
        let sphere = FibonacciSphere::new(100, 0.0, 123);
        println!("{:?}", sphere.points());
    }
}
