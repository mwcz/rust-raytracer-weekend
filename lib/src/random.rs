// A small collection of random number convenience functions.

use crate::RNG;
use num::traits::Float;

/// Generate a random number in the range [0..1).  Generic over Floats.
pub fn random_float<T: Float>() -> T {
    let mut num: u64;
    {
        // scope controls when the RNG mutex is released
        let mut rng = RNG
            .lock()
            .map_err(|_| "Failed to acquire RNG mutex")
            .unwrap();

        *rng *= 0xda942042e4dd58b5;
        num = *rng;
    }

    num >>= 32;

    let num = (num as f32) / 2f32.powi(32);

    T::from(num).unwrap()
}

/// Generate a random number in the range [min..max) based on the given min and max values.  Generic over Floats.
pub fn random_float_in_range<T: Float>(min: T, max: T) -> T {
    min + (max - min) * random_float()
}
