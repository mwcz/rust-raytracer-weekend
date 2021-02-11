use num::traits::Float;
use num::traits::Num;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::time;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              VEC3                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Num + Copy> {
    x: T,
    y: T,
    z: T,
}

// Alias Point3 and Color to avoid accidental concept pollution.
type Point3<T> = Vec3<T>;
type Color<T> = Vec3<T>;

///////////
//  ADD  //
///////////

impl<T: Num + Copy> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Num + Copy> Add<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

//////////////////
//  ADD ASSIGN  //
//////////////////

impl<T: Num + Copy + AddAssign> AddAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Num + Copy + AddAssign> AddAssign<T> for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

///////////
//  SUB  //
///////////

impl<T: Num + Copy> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Num + Copy> Sub<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

//////////////////
//  SUB ASSIGN  //
//////////////////

impl<T: Num + Copy + SubAssign> SubAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Num + Copy + SubAssign> SubAssign<T> for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

///////////
//  MUL  //
///////////

impl<T: Num + Copy> Mul<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

//////////////////
//  MUL ASSIGN  //
//////////////////

impl<T: Num + Copy + MulAssign> MulAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: Num + Copy + MulAssign> MulAssign<T> for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

///////////
//  DIV  //
///////////

impl<T: Num + Copy> Div<Vec3<T>> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Num + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

//////////////////
//  DIV ASSIGN  //
//////////////////

impl<T: Num + Copy + DivAssign> DivAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

// #[duplicate(
//     name; [ Vec3<T> ]; [ &Vec3<T> ]; [ &mut Vec3<T> ]
// )]
// impl<T: Num + Copy + DivAssign> DivAssign<T> for name {
//     #[inline]
//     fn div_assign(&mut self, other: T) {
//         self.x /= other;
//         self.y /= other;
//         self.z /= other;
//     }
// }

impl<T: Num + Copy + DivAssign> DivAssign<T> for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

#[allow(dead_code)]
impl<T: Num + Copy> Vec3<T> {
    // ///  Create a new vector that's the sum of this vector and another.
    // #[inline]
    // fn addv(&self, other: &Self) -> Vec3<T> {
    //     Vec3 {
    //         x: self.x + other.x,
    //         y: self.y + other.y,
    //         z: self.z + other.z,
    //     }
    // }

    // ///  Add a vector into this vector.
    // #[inline]
    // fn self_addv(&mut self, other: &Self) -> &Vec3<T> {
    //     self.x = self.x + other.x;
    //     self.y = self.y + other.y;
    //     self.z = self.z + other.z;
    //     self
    // }

    // ///  Create a new vector that's the sum of this vector and a scalar.
    // #[inline]
    // fn adds(&self, s: T) -> Vec3<T> {
    //     Vec3 {
    //         x: self.x + s,
    //         y: self.y + s,
    //         z: self.z + s,
    //     }
    // }

    // ///  Add a scalar into this vector.
    // #[inline]
    // fn self_adds(&mut self, s: T) -> &Vec3<T> {
    //     self.x = self.x + s;
    //     self.y = self.y + s;
    //     self.z = self.z + s;
    //     self
    // }

    // ///  Create a new vector that's the difference between this vector and another.
    // #[inline]
    // fn subv(&self, other: &Self) -> Vec3<T> {
    //     Vec3 {
    //         x: self.x - other.x,
    //         y: self.y - other.y,
    //         z: self.z - other.z,
    //     }
    // }

    // ///  Subtract a vector into this vector.
    // #[inline]
    // fn self_subv(&mut self, other: &Self) -> &Vec3<T> {
    //     self.x = self.x - other.x;
    //     self.y = self.y - other.y;
    //     self.z = self.z - other.z;
    //     self
    // }

    // ///  Create a new vector that's this vector with a scalar subtracted.
    // #[inline]
    // fn subs(&self, s: T) -> Vec3<T> {
    //     Vec3 {
    //         x: self.x - s,
    //         y: self.y - s,
    //         z: self.z - s,
    //     }
    // }

    // ///  Subtract a scalar into this vector.
    // #[inline]
    // fn self_subs(&mut self, s: T) -> &Vec3<T> {
    //     self.x = self.x - s;
    //     self.y = self.y - s;
    //     self.z = self.z - s;
    //     self
    // }

    // ///  Create a new vector that's this vector divided by another vector.
    // #[inline]
    // fn divv(&self, other: Self) -> Vec3<T> {
    //     Vec3 {
    //         x: self.x / other.x,
    //         y: self.y / other.y,
    //         z: self.z / other.z,
    //     }
    // }

    // ///  Divide this vector by a scalar.
    // #[inline]
    // fn self_divv(&mut self, other: Self) -> &Vec3<T> {
    //     self.x = self.x / other.x;
    //     self.y = self.y / other.y;
    //     self.z = self.z / other.z;
    //     self
    // }

    // ///  Create a new vector that's this vector divided by a scalar.
    // #[inline]
    // fn divs(&self, s: T) -> Vec3<T> {
    //     Vec3 {
    //         x: self.x / s,
    //         y: self.y / s,
    //         z: self.z / s,
    //     }
    // }

    // ///  Divide this vector by a scalar.
    // #[inline]
    // fn self_divs(&mut self, s: T) -> &Vec3<T> {
    //     self.x = self.x / s;
    //     self.y = self.y / s;
    //     self.z = self.z / s;
    //     self
    // }

    // ///  Create a new vector that's this vector multiplied by another vector.
    // #[inline]
    // fn mulv(&self, other: Self) -> Vec3<T> {
    //     Vec3 {
    //         x: self.x * other.x,
    //         y: self.y * other.y,
    //         z: self.z * other.z,
    //     }
    // }

    // ///  Multiply this vector by a vector.
    // #[inline]
    // fn self_mulv(&mut self, other: Self) -> &Vec3<T> {
    //     self.x = self.x * other.x;
    //     self.y = self.y * other.y;
    //     self.z = self.z * other.z;
    //     self
    // }

    // ///  Create a new vector that's this vector multiplied by a scalar.
    // #[inline]
    // fn muls(&self, s: T) -> Vec3<T> {
    //     Vec3 {
    //         x: self.x * s,
    //         y: self.y * s,
    //         z: self.z * s,
    //     }
    // }

    // ///  Multiply this vector by a scalar.
    // #[inline]
    // fn self_muls(&mut self, s: T) -> &Vec3<T> {
    //     self.x = self.x * s;
    //     self.y = self.y * s;
    //     self.z = self.z * s;
    //     self
    // }

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

#[allow(dead_code)]
impl<T: Float> Vec3<T> {
    /// Get the magnitude of this vector.
    #[inline]
    fn mag(&self) -> T {
        self.mag_squared().sqrt()
    }

    /// Create a new vector that's this vector reduced to length 1.
    #[inline]
    fn unit(&self) -> Vec3<T> {
        *self / (self.mag())
    }

    /// Normalize this vector; reduce it to length 1.
    #[inline]
    fn self_unit(&mut self) -> &Vec3<T> {
        let mag = self.mag();
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
        self
    }
}

impl<T: fmt::Display + Num + Copy> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              RAY                                               //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Ray<T: Float> {
    origin: Point3<T>,
    direction: Vec3<T>,
}

#[allow(dead_code)]
impl<T: Float> Ray<T> {
    fn at(&self, t: T) -> Vec3<T> {
        self.origin + self.direction * t
    }
}

// fn ray_color<T: Float, C: Num + Copy>(ray: &Ray<T>) -> Color<C> {
//     let unit_direction = ray.direction.unit();
//     let t = 0.5 * unit_direction.y + 1.0;
//     let mut color = Color::<C> {
//         x: 1.0,
//         y: 1.0,
//         z: 1.0,
//     };

//     color.muls(1.0 - t)
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              PPM                                               //
////////////////////////////////////////////////////////////////////////////////////////////////////

struct FinalImage {
    pixels: Vec<Color<u8>>,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              MAIN                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // Image

    let aspect_ratio = 16.0 / 10.0;
    let width: i32 = 400;
    let height: i32 = width / (aspect_ratio as i32);

    // Camera

    let viewport_height = 2.0;
    let _viewport_width = aspect_ratio * viewport_height;
    let _focal_length = 1.0;

    // Render

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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             TESTS                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ray_at() {
    let ray = Ray {
        origin: Point3 {
            x: 3.0,
            y: 5.0,
            z: 2.0,
        },
        direction: Vec3 {
            x: 8.0,
            y: -2.0,
            z: 1.0,
        },
    };

    let expected = Vec3 {
        x: 83.0,
        y: -15.0,
        z: 12.0,
    };

    assert_eq!(ray.at(10.0), expected);
}

#[test]
fn vec3_add_vector_operator() {
    let a = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let c = a + b;

    let expected = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };

    assert_eq!(expected, c);
}

#[test]
fn vec3_sub_vector_operator() {
    let a = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let c = a - b;

    let expected = Vec3 {
        x: 0.0,
        y: -1.0,
        z: -2.0,
    };

    assert_eq!(expected, c);
}

#[test]
fn vec3_mul_vector_operator() {
    let a = Vec3 {
        x: -1.0,
        y: 4.0,
        z: 3.0,
    };

    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let c = a * b;

    let expected = Vec3 {
        x: -1.0,
        y: 8.0,
        z: 9.0,
    };

    assert_eq!(expected, c);
}

#[test]
fn vec3_div_vector_operator() {
    let a = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 4.0,
    };

    let c = a / b;

    let expected = Vec3 {
        x: 1.0,
        y: 0.5,
        z: 0.25,
    };

    assert_eq!(expected, c);
}

#[test]
fn vec3_add_assign_vector_operator() {
    let mut a = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    a += b;

    let expected = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };

    assert_eq!(expected, a);
}

#[test]
fn vec3_sub_assign_vector_operator() {
    let mut a = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    a -= b;

    let expected = Vec3 {
        x: 0.0,
        y: -1.0,
        z: -2.0,
    };

    assert_eq!(expected, a);
}

#[test]
fn vec3_mul_assign_vector_operator() {
    let mut a = Vec3 {
        x: 9.0,
        y: 2.0,
        z: 1.0,
    };

    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    a *= b;

    let expected = Vec3 {
        x: 9.0,
        y: 4.0,
        z: 3.0,
    };

    assert_eq!(expected, a);
}

#[test]
fn vec3_div_assign_vector_operator() {
    let mut a = Vec3 {
        x: 9.0,
        y: 8.0,
        z: 6.0,
    };

    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    a /= b;

    let expected = Vec3 {
        x: 9.0,
        y: 4.0,
        z: 2.0,
    };

    assert_eq!(expected, a);
}

#[test]
fn vec3_add_assign_scalar_operator() {
    let mut a = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };

    a += 10.0;

    let expected = Vec3 {
        x: 12.0,
        y: 13.0,
        z: 14.0,
    };

    assert_eq!(expected, a);
}

#[test]
fn vec3_sub_assign_scalar_operator() {
    let mut a = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };

    a -= 10.0;

    let expected = Vec3 {
        x: -8.0,
        y: -7.0,
        z: -6.0,
    };

    assert_eq!(expected, a);
}

#[test]
fn vec3_mul_assign_scalar_operator() {
    let mut a = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };

    a *= 10.0;

    let expected = Vec3 {
        x: 20.0,
        y: 30.0,
        z: 40.0,
    };

    assert_eq!(expected, a);
}

#[test]
fn vec3_div_assign_scalar_operator() {
    let mut a = Vec3 {
        x: 2.0,
        y: 4.0,
        z: 6.0,
    };

    a /= 2.0;

    let expected = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    assert_eq!(expected, a);
}
