use crate::random;
use fmt::Display;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A vector with three components.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Alias Point3 and Color to avoid accidental concept pollution.
pub type Point3 = Vec3;
pub type Color = Vec3;

#[allow(dead_code)]
impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Create a vector randomly seeded with values in the range [0..1)
    #[allow(dead_code)]
    pub fn random() -> Vec3 {
        Vec3 {
            x: random::random_float(),
            y: random::random_float(),
            z: random::random_float(),
        }
    }

    /// Create a vector randomly seeded with values in the given range.
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: random::random_float_in_range(min, max),
            y: random::random_float_in_range(min, max),
            z: random::random_float_in_range(min, max),
        }
    }

    /// Create a vector randomly seeded with a point inside the unit sphere.
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);

            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    /// Create a unit vector pointing in a random direction.
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    /// Create a vector randomly seeded with a point inside the unit hemisphere occupied by the
    /// given normal.
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Create a vector random seeded within a unit disk.  To be used as an origin point for
    /// casting rays from a virtual film plane.
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3 {
                x: random::random_float_in_range(-1.0, 1.0),
                y: random::random_float_in_range(-1.0, 1.0),
                z: 0.0,
            };

            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    /// Return true if the vector is very close to the zero vector.
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    /// Reflect vector off normal n.
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0f32 * n * self.dot(&n)
        // n * v.dot(&n) * T::from(2.0).unwrap()
    }

    /// Refract vector entering surface with normal n.
    pub fn refract(&self, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let dot_normal = (self * -1.0).dot(&n);
        let cos_theta = dot_normal.min(1.0);
        let r_out_perp = (*self + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(-r_out_perp.length_squared() + 1.0).sqrt();

        r_out_parallel + r_out_perp
    }
}

///////////
//  ADD  //
///////////

// Vec3 * Vec3
impl Add<Vec3> for Vec3 {
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

// Vec * Float
impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

// f32 * Vec
impl Add<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

// I won't bother creating more Add impls for other numeric types, unless I start using this vector
// lib for more things, or I learn how to use macros to generate the variations. :)

//////////////////
//  ADD ASSIGN  //
//////////////////

// Vec3 += Vec3
impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Vec3 += n
impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: f32) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

///////////
//  NEG  //
///////////

// -Vec3
impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

///////////
//  SUB  //
///////////

// Vec3 - Vec3
impl Sub<Vec3> for Vec3 {
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

// Vec3 - n
impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

// f32 - Vec3
impl Sub<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

//////////////////
//  SUB ASSIGN  //
//////////////////

impl SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: f32) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

///////////
//  MUL  //
///////////

impl Mul<Vec3> for Vec3 {
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

impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<'a> Mul for &'a Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// f32 * Vec3
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

//////////////////
//  MUL ASSIGN  //
//////////////////

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

///////////
//  DIV  //
///////////

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<Vec3> for Vec3 {
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

impl Div for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, other: f32) -> Self {
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

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

#[allow(dead_code)]
impl Vec3 {
    /// Get the magnitude squared of this vector.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Get the dot product of this vector and another vector.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Get the cross product of this vector and another vector.
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[allow(dead_code)]
impl Vec3 {
    /// Get the magnitude of this vector.
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Create a new vector that's this vector reduced to length 1.
    #[inline]
    pub fn unit(&self) -> Vec3 {
        *self / (self.length())
    }

    /// Normalize this vector; reduce it to length 1.
    #[inline]
    pub fn self_unit(&mut self) -> &Vec3 {
        let length = self.length();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
        self
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             TESTS                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

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
    println!("{:?}", a);

    a -= 10.0;
    println!("{:?}", a);

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
