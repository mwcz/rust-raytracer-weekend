use crate::random;
use fmt::Display;
use num::traits::{Float, Num};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              VEC3                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

/// A vector with three components.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// Alias Point3 and Color to avoid accidental concept pollution.
pub type Point3<T> = Vec3<T>;
pub type Color<T> = Vec3<T>;

#[allow(dead_code)]
impl<T: Float> Vec3<T> {
    pub fn zero() -> Vec3<T> {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn one() -> Vec3<T> {
        Vec3 {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }

    /// Create a vector randomly seeded with values in the range [0..1)
    #[allow(dead_code)]
    pub fn random() -> Vec3<T> {
        Vec3 {
            x: random::random_float(),
            y: random::random_float(),
            z: random::random_float(),
        }
    }

    /// Create a vector randomly seeded with values in the given range.
    pub fn random_range(min: T, max: T) -> Vec3<T> {
        Vec3 {
            x: random::random_float_in_range(min, max),
            y: random::random_float_in_range(min, max),
            z: random::random_float_in_range(min, max),
        }
    }

    /// Create a vector randomly seeded with a point inside the unit sphere.
    pub fn random_in_unit_sphere() -> Vec3<T> {
        loop {
            let p = Vec3::random_range(-T::one(), T::one());

            if p.length_squared() >= T::one() {
                continue;
            }

            return p;
        }
    }

    /// Create a unit vector pointing in a random direction.
    pub fn random_unit_vector() -> Vec3<T> {
        Vec3::<T>::random_in_unit_sphere().unit()
    }

    /// Create a vector randomly seeded with a point inside the unit hemisphere occupied by the
    /// given normal.
    pub fn random_in_hemisphere(normal: &Vec3<T>) -> Vec3<T> {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > T::zero() {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Create a vector random seeded within a unit disk.  To be used as an origin point for
    /// casting rays from a virtual film plane.
    pub fn random_in_unit_disk() -> Vec3<T> {
        loop {
            let p = Vec3 {
                x: random::random_float_in_range(-T::one(), T::one()),
                y: random::random_float_in_range(-T::one(), T::one()),
                z: T::zero(),
            };

            if p.length_squared() >= T::one() {
                continue;
            }

            return p;
        }
    }

    /// Return true if the vector is very close to the zero vector.
    pub fn near_zero(&self) -> bool {
        let s = T::from(1e-8).unwrap();
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    /// Reflect vector off normal n.
    pub fn reflect(&self, n: Vec3<T>) -> Vec3<T> {
        *self - n * T::from(2.0).unwrap() * self.dot(&n)
        // n * v.dot(&n) * T::from(2.0).unwrap()
    }

    /// Refract vector entering surface with normal n.
    pub fn refract(&self, n: Vec3<T>, etai_over_etat: T) -> Vec3<T> {
        let dot_normal = (self * -T::one()).dot(&n);
        let cos_theta = dot_normal.min(T::one());
        let r_out_perp = (*self + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(-r_out_perp.length_squared() + T::one()).sqrt();

        r_out_parallel + r_out_perp
    }
}

// impl<T: Float> Clone for Vec3<T> {
//     fn clone(&self) -> Self {
//         Vec3 {
//             x: self.x,
//             y: self.y,
//             z: self.z,
//         }
//     }
//     fn clone_from(&mut self, source: &Self) {
//         self.x = source.x;
//         self.y = source.y;
//         self.z = source.z;
//     }
// }

///////////
//  ADD  //
///////////

// Vec3 * Vec3
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

// Vec * Float
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

// f64 * Vec
impl Add<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    #[inline]
    fn add(self, other: Vec3<f64>) -> Vec3<f64> {
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
impl<T: Num + Copy + AddAssign> AddAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Vec3 += n
impl<T: Num + Copy + AddAssign> AddAssign<T> for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

///////////
//  NEG  //
///////////

// -Vec3
impl<T: Num + Copy + Neg + Neg<Output = T>> Neg for Vec3<T> {
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

// Vec3 - n
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

// f64 - Vec3
impl Sub<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    #[inline]
    fn sub(self, other: Vec3<f64>) -> Vec3<f64> {
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

impl<'a, T: Num + Copy> Mul<T> for &'a Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn mul(self, other: T) -> Vec3<T> {
        Vec3::<T> {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// f64 * Vec3
impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    #[inline]
    fn mul(self, other: Vec3<f64>) -> Vec3<f64> {
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
    /// Get the magnitude squared of this vector.
    #[inline]
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Get the dot product of this vector and another vector.
    #[inline]
    pub fn dot(&self, other: &Self) -> T {
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
impl<T: Float> Vec3<T> {
    /// Get the magnitude of this vector.
    #[inline]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    /// Create a new vector that's this vector reduced to length 1.
    #[inline]
    pub fn unit(&self) -> Vec3<T> {
        *self / (self.length())
    }

    /// Normalize this vector; reduce it to length 1.
    #[inline]
    pub fn self_unit(&mut self) -> &Vec3<T> {
        let length = self.length();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
        self
    }
}

impl<T: Display + Num + Copy> Display for Vec3<T> {
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
