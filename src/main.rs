use std::env;
use std::fs::File;
use std::io::Write;
use std::time;

use num::traits::Float;
use num::traits::Num;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T: Num + Copy> {
    x: T,
    y: T,
    z: T,
}

// impl<T: Num + Copy> Clone for Vec3<T> {
//     /// Clone this vector.
//     fn clone(&self) -> Vec3<T> {
//         Vec3 {
//             x: self.x,
//             y: self.y,
//             z: self.z,
//         }
//     }
// }

impl<T: Num + Copy> Vec3<T> {
    ///  Add a vector into this vector.
    #[inline]
    fn add(&mut self, other: &Self) -> &Vec3<T> {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        self
    }

    ///  Add a scalar into this vector.
    #[inline]
    fn add_s(&mut self, s: T) -> &Vec3<T> {
        self.x = self.x + s;
        self.y = self.y + s;
        self.z = self.z + s;
        self
    }

    ///  Subtract a vector into this vector.
    #[inline]
    fn sub(&mut self, other: &Self) -> &Vec3<T> {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
        self
    }

    ///  Subtract a scalar into this vector.
    #[inline]
    fn sub_s(&mut self, s: T) -> &Vec3<T> {
        self.x = self.x - s;
        self.y = self.y - s;
        self.z = self.z - s;
        self
    }

    ///  Divide this vector by a scalar.
    #[inline]
    fn div(&mut self, other: Self) -> &Vec3<T> {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
        self
    }

    ///  Divide this vector by a scalar.
    #[inline]
    fn div_s(&mut self, s: T) -> &Vec3<T> {
        self.x = self.x / s;
        self.y = self.y / s;
        self.z = self.z / s;
        self
    }

    ///  Multiply this vector by a vector.
    #[inline]
    fn mul(&mut self, other: Self) -> &Vec3<T> {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
        self
    }

    ///  Multiply this vector by a scalar.
    #[inline]
    fn mul_s(&mut self, s: T) -> &Vec3<T> {
        self.x = self.x * s;
        self.y = self.y * s;
        self.z = self.z * s;
        self
    }

    /// Get the magnitude squared of this vector.
    #[inline]
    fn mag_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Get the dot product of this vector and another vector.
    #[inline]
    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Get the cross product of this vector and another vector.
    #[inline]
    fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: Float> Vec3<T> {
    /// Get the magnitude of this vector.
    #[inline]
    fn mag(&self) -> T {
        self.mag_squared().sqrt()
    }

    /// Normalize the vector; reduce it to length 1.
    #[inline]
    fn normalize(&mut self) -> &Vec3<T> {
        self.div_s(self.mag())
    }
}

impl<T: fmt::Display + Num + Copy> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct FinalImage {
    pixels: Vec<Vec3<u8>>,
    width: i32,
    height: i32,
}

/// Write a PPM image to a temp directory.  Image size and contents are passed in a FinalImage.
fn write_ppm(image_data: FinalImage) {
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
        writeln!(&mut file, "{} {} {}   ", rgb.x, rgb.y, rgb.z).unwrap();
    }
}

fn main() {
    let width: i32 = 256;
    let height: i32 = 256;

    // Generate a fake image to test ppm output
    let mut pixels = vec![Vec3 { x: 0, y: 0, z: 0 }; (width * height) as usize];

    let mut i: usize = 0;
    for y in (0..height).rev() {
        for x in 0..width {
            let v = &mut pixels[i];
            v.x = x as u8;
            v.y = y as u8;
            v.z = 64 as u8;

            i += 1;
        }
    }

    write_ppm(FinalImage {
        width,
        height,
        pixels,
    });

    // for i in 0..10 {
    // let mut a = Vec3::<f32> {
    //     x: 1.0,
    //     y: 2.0,
    //     z: 3.0,
    // };
    // let b = Vec3::<f32> {
    //     x: 4.0,
    //     y: 5.0,
    //     z: 6.0,
    // };
    // a += b;
    // a.add(&b);
    // a.add(&b);
    // a.add_s(10.0);
    // a.mul_s(10.0);
    // a.sub_s(10.0);
    // println!("");
    // println!("a     = {}", a);
    // println!("b     = {}", b);
    // println!("|a|   = {}", a.mag());
    // println!("|b|   = {}", b.mag());
    // println!("a · b = {}", a.dot(&b));
    // println!("a.cross(b) = {}", a.cross(&b));
    // }
}
