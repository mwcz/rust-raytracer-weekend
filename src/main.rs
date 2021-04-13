mod random;
mod vec;

use image;
use num::clamp;
use num::traits::Float;
use pbr::ProgressBar;
use random::*;
use vec::{Color, Point3, Vec3};

use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::time;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             CAMERA                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
struct Camera<T: Float + Debug> {
    aspect_ratio: T,
    origin: Point3<T>,
    lower_left_corner: Point3<T>,
    horizontal: Vec3<T>,
    vertical: Vec3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    w: Vec3<T>,
    lens_radius: T,
}

impl<T: Float + Debug> Camera<T> {
    /// Create a new Camera.
    fn new(
        lookfrom: Point3<T>,
        lookat: Point3<T>,
        vup: Vec3<T>,
        vfov: T,
        aspect_ratio: T,
        aperture: T,
        focus_dist: T,
    ) -> Camera<T> {
        let two = T::from(2.0).unwrap();

        let theta = vfov.to_radians();
        let theta_half = theta / two;
        let h = theta_half.tan();
        let viewport_height = h * two;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / two - vertical / two - w * focus_dist;
        let lens_radius = aperture / two;

        Camera {
            aspect_ratio,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    /// Get a ray at (u,v).
    fn get_ray(&self, s: T, t: T) -> Ray<T> {
        let rd = Vec3::<T>::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
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
impl<T: Float + Debug> Ray<T> {
    #[inline]
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

    fn color(&self, rec: &mut HitRecord<T>, world: &HittableList<T>, depth: i32) -> Color<T> {
        if depth <= 0 {
            return Color::zero();
        }

        if world.hit(&self, T::from(0.001).unwrap(), T::infinity(), rec) {
            let mut scattered = Ray::<T>::new();
            let mut attenuation = Color::<T>::zero();

            let is_scattered = rec
                .material
                .scatter(&self, &*rec, &mut attenuation, &mut scattered);

            if is_scattered {
                return attenuation * scattered.color(rec, world, depth - 1);
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

        let unit_direction = self.direction.unit();

        let t = T::from(0.5).unwrap() * (unit_direction.y + T::from(1.0).unwrap());

        let color = Vec3 {
            x: T::from(248.0 / 255.0).unwrap(),
            y: T::from(245.0 / 255.0).unwrap(),
            z: T::from(254.0 / 255.0).unwrap(),
        } * (T::one() - t)
            + Vec3 {
                x: T::from(139.0 / 255.0).unwrap(),
                y: T::from(179.0 / 255.0).unwrap(),
                z: T::from(237.0 / 255.0).unwrap(),
            } * t;

        color
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

impl<T: Float + Debug> Hittable<T> for Sphere<T> {
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
        rec: &HitRecord<T>,
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
        _r_in: &Ray<T>,
        rec: &HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::<T>::random_unit_vector();

        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        let scatter_ray = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };

        *scattered = scatter_ray;

        *attenuation = self.albedo;

        true
    }
}

struct MatMetal<T: Float> {
    albedo: Color<T>,
    fuzz: T,
}

impl<T: Float> Material<T> for MatMetal<T> {
    fn scatter(
        &self,
        r_in: &Ray<T>,
        rec: &HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        let reflected = r_in.direction.unit().reflect(rec.normal);

        *scattered = Ray {
            origin: rec.p,
            direction: reflected + Vec3::<T>::random_in_unit_sphere() * self.fuzz,
        };

        *attenuation = self.albedo;

        scattered.direction.dot(&rec.normal) > T::zero()
    }
}

struct MatDielectric<T: Float> {
    ir: T,
    albedo: Color<T>,
}

trait Dielectric<T> {
    fn reflectance(&self, cosine: T, ref_idx: T) -> T;
}

impl<T: Float> Material<T> for MatDielectric<T> {
    fn scatter(
        &self,
        r_in: &Ray<T>,
        rec: &HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        *attenuation = self.albedo;

        let refraction_ratio = if rec.front_face {
            T::one() / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.unit();

        let cos_theta = -unit_direction.dot(&rec.normal).min(T::one());
        let sin_theta = (T::one() - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > T::one();
        let should_reflect = self.reflectance(cos_theta, refraction_ratio) > random_float();

        let direction = if cannot_refract || should_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        *scattered = Ray {
            origin: rec.p,
            direction,
        };

        true
    }
}

impl<T: Float> Dielectric<T> for MatDielectric<T> {
    fn reflectance(&self, cosine: T, ref_idx: T) -> T {
        let r0 = (T::one() - ref_idx) / (T::one() + ref_idx);
        let r0 = r0 * r0;

        r0 + (T::one() - r0) * (T::one() - cosine).powi(5)
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
//                                             SCENES                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
fn random_scene() -> HittableList<f64> {
    let mut world = HittableList::<f64> {
        objects: Vec::new(),
    };

    // Ground

    let ground_material = Rc::new(MatLambertian {
        albedo: Color {
            x: 80.0 / 255.0,
            y: 144.0 / 255.0,
            z: 22.0 / 255.0,
        },
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: ground_material.clone(),
    }));

    // Sphere 1

    let material1 = Rc::new(MatDielectric {
        albedo: Color {
            x: 242.0 / 255.0,
            y: 111.0 / 255.0,
            z: 112.0 / 255.0,
        },
        ir: 1.5,
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material1.clone(),
    }));

    // Sphere 2

    let material2 = Rc::new(MatLambertian {
        albedo: Color {
            x: 111.0 / 255.0,
            y: 165.0 / 255.0,
            z: 242.0 / 255.0,
        },
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material2.clone(),
    }));

    // Sphere 3

    let material3 = Rc::new(MatMetal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material3.clone(),
    }));

    let boundary = Point3 {
        x: 4.0,
        y: 0.2,
        z: 0.0,
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_float();

            let center = Point3 {
                x: (a as f64) + 0.9 * random_float::<f64>(),
                y: 0.2,
                z: (b as f64) + 0.9 * random_float::<f64>(),
            };

            if (center - boundary).length() > 0.9 {
                if choose_mat < 0.66 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(MatLambertian { albedo });

                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material.clone(),
                    }));
                } else if choose_mat < 0.85 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_float_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(MatMetal { albedo, fuzz });

                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material.clone(),
                    }));
                } else {
                    let albedo = Color::random_range(0.8, 1.0);
                    let sphere_material = Rc::new(MatDielectric { albedo, ir: 1.5 });

                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material.clone(),
                    }));
                }
            }
        }
    }

    world
}

#[allow(dead_code)]
fn ten_spheres_scene() -> HittableList<f64> {
    let mut world = HittableList::<f64> {
        objects: Vec::new(),
    };

    // Materials

    let default_material = Rc::new(MatLambertian {
        albedo: Color {
            x: 122.0 / 255.0,
            y: 175.0 / 255.0,
            z: 238.0 / 255.0,
        },
    });

    let grey_material = Rc::new(MatLambertian {
        albedo: Color {
            x: 255.0 / 255.0,
            y: 255.0 / 255.0,
            z: 255.0 / 255.0,
        },
    });

    let ground_material = Rc::new(MatLambertian {
        albedo: Color {
            x: 72.0 / 255.0,
            y: 72.0 / 255.0,
            z: 72.0 / 255.0,
        },
    });

    let metal_material = Rc::new(MatMetal {
        albedo: Color {
            x: 64.0 / 255.0,
            y: 64.0 / 255.0,
            z: 64.0 / 255.0,
        },
        fuzz: 0.1,
    });

    let mirror_material = Rc::new(MatMetal {
        albedo: Color {
            x: 253.0 / 255.0,
            y: 253.0 / 255.0,
            z: 255.0 / 255.0,
        },
        fuzz: 0.0,
    });

    let metal_red_material = Rc::new(MatMetal {
        albedo: Color {
            x: 208.0 / 255.0,
            y: 66.0 / 255.0,
            z: 70.0 / 255.0,
        },
        fuzz: 0.3,
    });

    // Sphere 1
    world.add(Box::new(Sphere {
        center: Point3 {
            x: -3.363,
            y: 0.45,
            z: -2.705 - 0.5,
        },
        radius: 0.9,
        material: grey_material.clone(),
    }));

    // Sphere 2
    world.add(Box::new(Sphere {
        center: Point3 {
            x: -1.84,
            y: 0.45,
            z: -4.028 - 0.5,
        },
        radius: 0.9,
        material: metal_material.clone(),
    }));

    // Sphere 3 (center)
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.45,
            z: -4.3 - 0.5,
        },
        radius: 0.9,
        material: default_material.clone(),
    }));

    // Sphere 4
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 1.84,
            y: 0.45,
            z: -4.028 - 0.5,
        },
        radius: 0.9,
        material: mirror_material.clone(),
    }));

    // Sphere 5
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 3.363,
            y: 0.45,
            z: -2.705 - 0.5,
        },
        radius: 0.9,
        material: metal_red_material.clone(),
    }));

    for s in 0..5 {
        // Glass sphere
        let s = s as f64;
        world.add(Box::new(Sphere {
            center: Point3 {
                x: -0.7 + s * 0.35,
                y: -0.28,
                z: 1.0,
            },
            radius: 0.15,
            material: Rc::new(MatDielectric {
                ir: 1.5,
                albedo: Color {
                    x: random_float(),
                    y: random_float(),
                    z: random_float(),
                },
            }),
        }));
    }

    // "World" sphere
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.45,
            z: -1.2,
        },
        radius: 1000.0,
        material: ground_material.clone(),
    }));

    world
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          IMAGE FILES                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

struct FinalImage {
    pixels: Vec<Color<f64>>,
    width: u32,
    height: u32,
    samples_per_pixel: i32,
}

/// Write a PPM image to a temp directory.  Image size and contents are passed in a FinalImage.
#[allow(dead_code)]
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

#[allow(dead_code)]
fn write_color(file: &mut File, pixel_color: &Color<f64>, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f64);
    // sqrt applies gamma 2, ie raising the color to the power of 1/gamma, where gamma is 2.
    let r = 256.0 * clamp(pixel_color.x * scale, 0.0, 0.999).sqrt();
    let g = 256.0 * clamp(pixel_color.y * scale, 0.0, 0.999).sqrt();
    let b = 256.0 * clamp(pixel_color.z * scale, 0.0, 0.999).sqrt();
    writeln!(file, "{} {} {}   ", r as u8, g as u8, b as u8).unwrap();
}

/// Write an image file to a temp directory.  Image size and contents are passed in a FinalImage
/// struct.
fn write_image(image_data: FinalImage) {
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

fn get_color_u8(pixel_color: &Color<f64>, samples_per_pixel: i32) -> Color<u8> {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              MAIN                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

fn render() {
    // Configuration

    let aspect_ratio = 3.0 / 2.0;
    let width = 400.0;
    let height = (width / aspect_ratio).floor();
    let samples_per_pixel: i32 = 50;
    let max_depth = 10;

    // Progress bar
    let mut pb = ProgressBar::new((width * height) as u64);

    // World

    // let world = random_scene();
    let world = ten_spheres_scene();

    // Camera

    // let lookfrom = Point3 {
    //     x: 13.0,
    //     y: 2.0,
    //     z: 3.0,
    // };
    // let lookat = Point3 {
    //     x: 0.0,
    //     y: 0.0,
    //     z: 0.0,
    // };
    // let vup = Vec3 {
    //     x: 0.0,
    //     y: 1.0,
    //     z: 0.0,
    // };
    // let dist_to_focus = 10.0;
    // let aperture = 0.0;

    // let cam = Camera::new(
    //     lookfrom,
    //     lookat,
    //     vup,
    //     20.0,
    //     aspect_ratio,
    //     aperture,
    //     dist_to_focus,
    // );
    let lookfrom = Point3 {
        x: 0.0,
        y: 0.5,
        z: 4.0,
    };
    let lookat = Point3 {
        x: 0.0,
        y: 0.0,
        z: -3.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        45.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render

    let mut pixels = vec![
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };
        (width * height) as usize
    ];

    // Default material to clone into each HitRecord
    let default_material = Rc::new(MatLambertian {
        albedo: Color {
            x: 122.0 / 255.0,
            y: 175.0 / 255.0,
            z: 238.0 / 255.0,
        },
    });

    let mut i: usize = 0;
    for y in (0..(height as i32)).rev() {
        for x in 0..(width as i32) {
            let p = &mut pixels[i];

            for _ in 0..samples_per_pixel {
                // don't use RNG if there's only one sample per pixel
                let u_rand = if samples_per_pixel > 1 {
                    random_float()
                } else {
                    1.0
                };

                let v_rand = if samples_per_pixel > 1 {
                    random_float()
                } else {
                    1.0
                };

                let u = (u_rand + x as f64) / (width - 1.0);
                let v = (v_rand + y as f64) / (height - 1.0);

                let ray = cam.get_ray(u, v);

                let mut rec = HitRecord::new(default_material.clone());

                *p += ray.color(&mut rec, &world, max_depth);
            }

            i += 1;

            pb.inc();
        }
    }

    write_image(FinalImage {
        width: width as u32,
        height: height as u32,
        pixels,
        samples_per_pixel,
    });

    pb.finish_print("Done!");
}

fn main() {
    render();
}

// #[test]
// fn ray_at() {
//     let ray = Ray {
//         origin: Point3 {
//             x: 3.0,
//             y: 5.0,
//             z: 2.0,
//         },
//         direction: Vec3 {
//             x: 8.0,
//             y: -2.0,
//             z: 1.0,
//         },
//     };

//     let expected = Vec3 {
//         x: 83.0,
//         y: -15.0,
//         z: 12.0,
//     };

//     assert_eq!(ray.at(10.0), expected);
// }
