use fmt::Display;
use num::clamp;
use num::traits::{Float, Num};
use rand::prelude::*;
use std::env;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::rc::Rc;
use std::time;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              VEC3                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T: Num + Copy> {
    x: T,
    y: T,
    z: T,
}

// Alias Point3 and Color to avoid accidental concept pollution.
type Point3<T> = Vec3<T>;
type Color<T> = Vec3<T>;

#[allow(dead_code)]
impl<T: Float> Vec3<T> {
    fn zero() -> Vec3<T> {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Create a vector randomly seeded with values in the range [0..1)
    #[allow(dead_code)]
    fn random() -> Vec3<T> {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: T::from(rng.gen::<f64>()).unwrap(),
            y: T::from(rng.gen::<f64>()).unwrap(),
            z: T::from(rng.gen::<f64>()).unwrap(),
        }
    }

    /// Create a vector randomly seeded with values in the given range.
    fn random_range(min: f64, max: f64) -> Vec3<T> {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: T::from(rng.gen_range(min..max)).unwrap(),
            y: T::from(rng.gen_range(min..max)).unwrap(),
            z: T::from(rng.gen_range(min..max)).unwrap(),
        }
    }

    /// Create a vector randomly seeded with a point inside the unit sphere.
    fn random_in_unit_sphere() -> Vec3<T> {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);

            if p.length_squared() >= T::one() {
                continue;
            }

            return p;
        }
    }

    /// Create a unit vector pointing in a random direction.
    fn random_unit_vector() -> Vec3<T> {
        Vec3::<T>::random_in_unit_sphere().unit()
    }

    /// Create a vector randomly seeded with a point inside the unit hemisphere occupied by the
    /// given normal.
    fn random_in_hemisphere(normal: &Vec3<T>) -> Vec3<T> {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > T::zero() {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Return true if the vector is very close to the zero vector.
    fn near_zero(&self) -> bool {
        let s = T::from(1e-8).unwrap();
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    // Reflect vector v off normal n.
    fn reflect(v: Vec3<T>, n: Vec3<T>) -> Vec3<T> {
        n * v.dot(&n) * T::from(2.0).unwrap()
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
//  NEG  //
///////////

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
    fn length_squared(&self) -> T {
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
    fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    /// Create a new vector that's this vector reduced to length 1.
    #[inline]
    fn unit(&self) -> Vec3<T> {
        *self / (self.length())
    }

    /// Normalize this vector; reduce it to length 1.
    #[inline]
    fn self_unit(&mut self) -> &Vec3<T> {
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
//                                             CAMERA                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
struct Camera<T: Float + Debug> {
    aspect_ratio: T,
    viewport_height: T,
    viewport_width: T,
    focal_length: T,

    origin: Point3<T>,
    lower_left_corner: Point3<T>,
    horizontal: Vec3<T>,
    vertical: Vec3<T>,
}

impl<T: Float + Debug> Camera<T> {
    /// Create a new Camera.
    fn new(aspect_ratio: T, viewport_height: T) -> Camera<T> {
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = T::one();

        let origin = Point3::zero();

        let horizontal = Point3 {
            x: viewport_width,
            y: T::zero(),
            z: T::zero(),
        };

        let vertical = Point3 {
            x: T::zero(),
            y: viewport_height,
            z: T::zero(),
        };

        let lower_left_corner = origin
            - horizontal / T::from(2.0).unwrap()
            - vertical / T::from(2.0).unwrap()
            - Vec3 {
                x: T::zero(),
                y: T::zero(),
                z: focal_length,
            };

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    /// Get a ray at (u,v).
    fn get_ray(&self, u: T, v: T) -> Ray<T> {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              RAY                                               //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
struct Ray<T: Float> {
    origin: Point3<T>,
    direction: Vec3<T>,
}

#[allow(dead_code)]
impl<T: Float> Ray<T> {
    fn new() -> Ray<T> {
        Ray {
            origin: Point3::zero(),
            direction: Vec3::zero(),
        }
    }

    #[inline]
    fn at(&self, t: T) -> Vec3<T> {
        self.origin + self.direction * t
    }
}

// Possibly implement this later for more efficient cloning.
// impl<T: Float + Copy> Clone for Ray<T> {
//     fn clone(&self) -> Self {
//         Ray {
//             origin: Vec3 {
//                 x: self.origin.x,
//                 y: self.origin.y,
//                 z: self.origin.z,
//             },
//             direction: Vec3 {
//                 x: self.direction.x,
//                 y: self.direction.y,
//                 z: self.direction.z,
//             },
//         }
//     }
//     fn clone_from(&mut self, source: &Self) {
//         self.origin.x = source.origin.x;
//         self.origin.y = source.origin.y;
//         self.origin.z = source.origin.z;
//         self.direction.x = source.direction.x;
//         self.direction.y = source.direction.y;
//         self.direction.z = source.direction.z;
//     }
// }

fn ray_color<T: Float + Debug>(
    r: &Ray<T>,
    rec: &mut HitRecord<T>,
    world: &HittableList<T>,
    depth: i32,
) -> Color<T> {
    if depth <= 0 {
        return Color::zero();
    }

    if world.hit(r, T::from(0.001).unwrap(), T::infinity(), rec) {
        let mut scattered = Ray::<T>::new();
        let mut attenuation = Color::<T>::zero();

        let is_scattered = rec
            .material
            .scatter(&r, rec, &mut attenuation, &mut scattered);

        if is_scattered {
            return ray_color(&scattered, rec, world, depth - 1);
        } else {
            return Color::<T>::zero();
        }

        // let target = diffuse_renderer(rec.p.clone(), rec.normal.clone());

        // let new_ray = Ray {
        //     origin: rec.p.clone(),
        //     direction: target - rec.p.clone(),
        // };
        // return ray_color(&new_ray, rec, world, depth - 1, diffuse_renderer)
        //     * T::from(0.5).unwrap();
    }

    let unit_direction = r.direction.unit();

    let t = T::from(0.5).unwrap() * (unit_direction.y + T::from(1.0).unwrap());

    let color = Vec3 {
        x: T::one(),
        y: T::one(),
        z: T::one(),
    } * (T::one() - t)
        + Vec3 {
            x: T::from(0.5).unwrap(),
            y: T::from(0.7).unwrap(),
            z: T::from(1.0).unwrap(),
        } * t;

    color
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            HITTABLES                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

struct HitRecord<T: Float> {
    p: Point3<T>,
    normal: Vec3<T>,
    material: Rc<dyn Material<T>>,
    t: T,
    front_face: bool,
}

impl<T: Float> HitRecord<T> {
    fn new(material: Rc<dyn Material<T>>) -> HitRecord<T> {
        HitRecord {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: T::zero(),
            front_face: false,
            material,
        }
    }

    fn set_face_normal(&mut self, ray: &Ray<T>, outward_normal: Vec3<T>) {
        self.front_face = ray.direction.dot(&outward_normal) < T::zero();
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -T::one()
        };
    }
}

trait Hittable<T: Float> {
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool;
}

/////////////////////
//  HITTABLE LIST  //
/////////////////////

struct HittableList<T: Float> {
    objects: Vec<Box<dyn Hittable<T>>>,
}

#[allow(dead_code)]
impl<T: Float + Debug> HittableList<T> {
    #[allow(dead_code)]
    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, obj: Box<dyn Hittable<T>>) {
        self.objects.push(obj);
    }

    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = rec;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}

//////////////
//  SPHERE  //
//////////////

struct Sphere<T: Float> {
    center: Point3<T>,
    radius: T,
    material: Rc<dyn Material<T>>,
}

impl<T: Float> Hittable<T> for Sphere<T> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < T::zero() {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;

        rec.set_face_normal(&r, outward_normal);
        rec.material = self.material.clone();

        true
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           MATERIALS                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

trait Material<T: Float> {
    fn scatter(
        &self,
        r_in: &Ray<T>,
        rec: &mut HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool;
}

struct MatLambertian<T: Float> {
    albedo: Color<T>,
}

impl<T: Float> Material<T> for MatLambertian<T> {
    fn scatter(
        &self,
        r_in: &Ray<T>,
        rec: &mut HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::<T>::random_unit_vector();

        // avoid scattering when near zero
        if scatter_direction.near_zero() {
            // scatter_direction.clone_from(&rec.normal);
            scatter_direction = rec.normal;
        }

        let scatter_ray = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };

        // scattered.clone_from(&scatter_ray);
        *scattered = scatter_ray;

        // attenuation.clone_from(&self.albedo);
        *attenuation = self.albedo;

        true
    }
}

struct MatMetal<T: Float> {
    albedo: Color<T>,
}

impl<T: Float> Material<T> for MatMetal<T> {
    fn scatter(
        &self,
        r_in: &Ray<T>,
        rec: &mut HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction, rec.normal);

        *scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };

        *attenuation = self.albedo;

        scattered.direction.dot(&rec.normal) > T::zero()
    }
}

/// Raytracing in one weekend hack
#[allow(dead_code)]
fn rtiowh_hack<T: Float>(p: Vec3<T>, normal: Vec3<T>) -> Vec3<T> {
    p + normal + Vec3::<T>::random_in_unit_sphere()
}

/// True lambertian reflection
#[allow(dead_code)]
fn true_lambert<T: Float>(p: Vec3<T>, normal: Vec3<T>) -> Vec3<T> {
    p + normal + Vec3::<T>::random_unit_vector()
}

/// The most intuitive approach to diffuse rendering; cast a random bounce ray in the normal hemisphere
#[allow(dead_code)]
fn naive_hemisphere<T: Float>(p: Vec3<T>, normal: Vec3<T>) -> Vec3<T> {
    p + Vec3::<T>::random_in_hemisphere(&normal)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              PPM                                               //
////////////////////////////////////////////////////////////////////////////////////////////////////

struct FinalImage {
    pixels: Vec<Color<f64>>,
    width: i32,
    height: i32,
    samples_per_pixel: i32,
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
        write_color(&mut file, rgb, image_data.samples_per_pixel);
    }
}

fn write_color(file: &mut File, pixel_color: &Color<f64>, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f64);
    // sqrt applies gamma 2, ie raising the color to the power of 1/gamma, where gamma is 2.
    let r = 256.0 * clamp(pixel_color.x * scale, 0.0, 0.999).sqrt();
    let g = 256.0 * clamp(pixel_color.y * scale, 0.0, 0.999).sqrt();
    let b = 256.0 * clamp(pixel_color.z * scale, 0.0, 0.999).sqrt();
    writeln!(file, "{} {} {}   ", r as u8, g as u8, b as u8).unwrap();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              MAIN                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // Configuration

    let aspect_ratio = 16.0 / 10.0;
    let width = 400.0;
    let height = width / aspect_ratio;
    let samples_per_pixel: i32 = 100;
    let max_depth = 10;
    let diffuse_renderer = &true_lambert;

    // RNG

    let mut rng = rand::thread_rng();

    // World

    let mut world: HittableList<f64> = HittableList {
        objects: Vec::new(),
    };

    // Materials

    let default_material = Rc::new(MatLambertian {
        albedo: Color {
            x: 0.8,
            y: 0.0,
            z: 0.0,
        },
    });

    let ground_material = Rc::new(MatLambertian {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });

    let metal_material = Rc::new(MatMetal {
        albedo: Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    });

    // Center sphere

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: default_material.clone(),
    }));

    // Metal sphere to the left
    world.add(Box::new(Sphere {
        center: Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: metal_material.clone(),
    }));

    // Metal sphere to the right
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: metal_material.clone(),
    }));

    // Add some random spheres
    // for i in 0..100 {
    //     world.add(Box::new(Sphere {
    //         center: Point3 {
    //             x: 5.0 * (rng.gen::<f64>() - 0.5),
    //             y: 5.0 * (rng.gen::<f64>() - 0.5),
    //             z: 2.0 * (rng.gen::<f64>() - 1.0),
    //         },
    //         radius: 0.2,
    //     }));
    // }

    // "World" sphere
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -31.45,
            z: -1.0,
        },
        radius: 31.0,
        material: ground_material.clone(),
    }));

    // Camera

    let viewport_height = 2.0;
    let cam = Camera::new(aspect_ratio, viewport_height);

    // Render

    let mut pixels = vec![
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };
        (width * height) as usize
    ];

    let mut i: usize = 0;
    for y in (0..(height as i32)).rev() {
        for x in 0..(width as i32) {
            let p = &mut pixels[i];

            for _ in 0..samples_per_pixel {
                // don't use RNG if there's only one sample per pixel
                let u_rand = if samples_per_pixel > 1 {
                    rng.gen::<f64>()
                } else {
                    1.0
                };

                let v_rand = if samples_per_pixel > 1 {
                    rng.gen::<f64>()
                } else {
                    1.0
                };

                let u = (u_rand + x as f64) / (width - 1.0);
                let v = (v_rand + y as f64) / (height - 1.0);

                let ray = cam.get_ray(u, v);

                let mut rec = HitRecord::new(default_material.clone());

                *p += ray_color(&ray, &mut rec, &world, max_depth);
            }

            i += 1;
        }
    }

    write_ppm(FinalImage {
        width: width as i32,
        height: height as i32,
        pixels,
        samples_per_pixel,
    });

    println!("done");
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
