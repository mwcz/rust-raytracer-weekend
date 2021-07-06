use super::FinalImage;
use crate::vec::Color;
use num::clamp;
use std::env;
use std::fs::File;
use std::io::Write;
use std::time;

/// Write a PPM image to a temp directory.  Image size and contents are passed in a FinalImage.
#[allow(dead_code)]
pub fn write(image_data: FinalImage) {
    let now = time::SystemTime::now();
    let since = now
        .duration_since(time::UNIX_EPOCH)
        .expect("Time went backwards.");

    let filename = format!("raytrace-{:?}.ppm", since);

    let temp_dir = env::temp_dir();
    let temp_file = temp_dir.join(filename);

    let mut file = File::create(temp_file).unwrap();
    writeln!(&mut file, "P3").unwrap();
    writeln!(&mut file, "{} {}", image_data.width, image_data.height).unwrap();
    writeln!(&mut file, "255").unwrap(); // maximum RGB component value

    for rgb in image_data.pixels.iter() {
        write_color(&mut file, rgb, image_data.samples_per_pixel);
    }
}

#[allow(dead_code)]
fn write_color(file: &mut File, pixel_color: &Color<f64>, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f64);
    // sqrt applies gamma 2, ie raising the color to the power of 1/gamma, where gamma is 2.
    let r = 256.0 * clamp(pixel_color.x * scale, 0.0, 0.999).sqrt();
    let g = 256.0 * clamp(pixel_color.y * scale, 0.0, 0.999).sqrt();
    let b = 256.0 * clamp(pixel_color.z * scale, 0.0, 0.999).sqrt();
    writeln!(file, "{} {} {}   ", r as u8, g as u8, b as u8).unwrap();
}
