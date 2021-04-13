pub mod png;
pub mod ppm;

use crate::Color;

pub struct FinalImage {
    pub pixels: Vec<Color<f64>>,
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: i32,
}
