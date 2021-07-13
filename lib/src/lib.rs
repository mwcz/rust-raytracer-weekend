pub mod camera;
pub mod hit;
pub mod material;
pub mod objects;
pub mod random;
pub mod ray;
pub mod scenes;
pub mod vec;
pub mod write;

use lazy_static::lazy_static;
use num::traits::Float;
use std::rc::Rc;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<u64> = Mutex::new(0xda942042e4dd58b5);
}
