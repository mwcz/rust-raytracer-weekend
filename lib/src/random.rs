// A small collection of random number convenience functions.

use num::traits::Float;
use rand::prelude::*;

/// Generate a random number in the range [0..1).  Generic over Floats.
pub fn random_float<T: Float>() -> T {
    let mut rng = rand::thread_rng();
    T::from(rng.gen::<f64>()).unwrap()
}

/// Generate a random number in the range [min..max) based on the given min and max values.  Generic over Floats.
pub fn random_float_in_range<T: Float>(min: T, max: T) -> T {
    min + (max - min) * random_float()
}
