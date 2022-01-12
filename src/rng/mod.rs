//! Simple glue module between rand crate and this project for rng methods
//! in case I want to switch them in the future
use std::hash::Hash;

use rand::{RngCore, Rng};
use rand_pcg::Pcg64Mcg;
use rand_seeder::Seeder;

/// Uses a seed to generate randomness
/// # Generics
/// * ``T`` - The type of the Hash, must implement From<T>
struct SeededRng<T> {
    rng: Pcg64Mcg,
    hash: T,
}

impl<T: Hash + Copy> SeededRng<T> {
    pub fn new(hash: T) -> SeededRng<T> {
        let rng: Pcg64Mcg = Seeder::from(hash).make_rng();
        SeededRng {
            rng: rng,
            hash: hash,
        }
    }

    /// Generates u32s from a seed
    pub fn gen_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    /// Get a reference to the seeded rng's hash.
    /// # Examples
    /// ```
    /// let rng = SeededRng::new("Hello");
    /// let result = rng.hash;
    /// let exp_result = "Hello";
    /// assert_eq!(exp_result, result);
    /// ```
    pub fn hash(&self) -> &T {
        &self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::SeededRng;

    #[test]
    fn test_hash_preserved() {
        let rng = SeededRng::new("Hello");
        let result = rng.hash;
        let exp_result = "Hello";
        assert_eq!(exp_result, result);
        let rng = SeededRng::new(123);
        let result = rng.hash;
        let exp_result = 123;
        assert_eq!(exp_result, result);
    }

    #[test]
    fn test_is_seeded_u32() {
        let mut rng = SeededRng::new(123);
        assert_eq!(rng.gen_u32(), 4237460055);
        assert_eq!(rng.gen_u32(), 2125758287);
        assert_eq!(rng.gen_u32(), 3616798402);
        assert_eq!(rng.gen_u32(), 1488451274);
    }
}