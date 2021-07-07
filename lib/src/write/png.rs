use super::FinalImage;
use crate::vec::Color;
use image;
use num::clamp;
use std::env;
use std::time;

/// Write an image file to a temp directory.  Image size and contents are passed in a FinalImage
/// struct.
pub fn write(image_data: FinalImage) {
    let now = time::SystemTime::now();
    let since = now
        .duration_since(time::UNIX_EPOCH)
        .expect("Time went backwards.");

    let filename = format!("raytrace-{:?}.png", since);

    let temp_dir = env::temp_dir();
    let temp_file = temp_dir.join(&filename);

    let mut buf = image::ImageBuffer::new(image_data.width, image_data.height);

    for (i, pixel) in image_data.pixels.iter().enumerate() {
        let x = i as u32 % image_data.width;
        let y = i as u32 / image_data.width;
        let color = get_color_u8(&&pixel, image_data.samples_per_pixel);
        buf.put_pixel(x, y, image::Rgb([color.x, color.y, color.z]));
    }

    match buf.save(temp_file) {
        Ok(_) => println!("Wrote {}", filename),
        Err(err) => println!("Error writing {}", err),
    }
}

pub fn get_color_u8(pixel_color: &Color<f64>, samples_per_pixel: i32) -> Color<u8> {
    let scale = 1.0 / (samples_per_pixel as f64);
    // sqrt applies gamma 2, ie raising the color to the power of 1/gamma, where gamma is 2.
    let r = 256.0 * clamp(pixel_color.x * scale, 0.0, 0.999).sqrt();
    let g = 256.0 * clamp(pixel_color.y * scale, 0.0, 0.999).sqrt();
    let b = 256.0 * clamp(pixel_color.z * scale, 0.0, 0.999).sqrt();

    Color {
        x: r as u8,
        y: g as u8,
        z: b as u8,
    }
}
