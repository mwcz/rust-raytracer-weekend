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

// lazy_static! {
//     static ref RNG: Mutex<u128> = Mutex::new(0xda942042e4dd58b5);
// }

// __uint128_t g_lehmer64_state;

// uint64_t lehmer64() {
//   g_lehmer64_state *= 0xda942042e4dd58b5;
//   return g_lehmer64_state >> 64;
// }

// fn gen() -> Result<u128> {
//     let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
//     db.push(fruit.to_string());
//     Ok(())
// }

/// Example render.
// pub fn render() -> Vec<vec::Vec3<f64>> {
pub fn render() -> Vec<u8> {
    // let rng = RNG.lock().map_err(|_| "Failed to acquire RNG mutex");
    // rng.iter()
    //     .enumerate()
    //     .for_each(|(i, item)| println!("{}: {}", i, item));
    // match rng {
    //     Ok(n) => println!("{}", n),
    //     Err(e) => println!("nothing"),
    // }
    // Configuration

    // let aspect_ratio = 3.0 / 2.0;
    // let width = 400.0;
    // let height = (width / aspect_ratio).floor();
    // let samples_per_pixel: i32 = 100;
    // let max_depth = 10;

    let aspect_ratio = 3.0 / 2.0;
    let width = 300.0;
    let height = (width / aspect_ratio).floor();
    let samples_per_pixel: i32 = 10;
    let max_depth = 3;

    // World

    // let world = Scenes::random_scene::scene();
    // let world = Scenes::ten_spheres::scene();
    let world = scenes::glass_sphere_scene::scene();

    // Camera

    let lookfrom = vec::Point3 {
        x: 0.0,
        y: 0.5,
        z: 4.0,
    };
    let lookat = vec::Point3 {
        x: 0.0,
        y: 0.0,
        z: -3.0,
    };
    let vup = vec::Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = camera::Camera::new(
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
        vec::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        (width * height) as usize
    ];

    // Default material to clone into each HitRecord
    let default_material = Rc::new(material::Lambertian {
        albedo: vec::Color {
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
                    random::random_float()
                } else {
                    1.0
                };

                let v_rand = if samples_per_pixel > 1 {
                    random::random_float()
                } else {
                    1.0
                };

                let u = (u_rand + x as f64) / (width - 1.0);
                let v = (v_rand + y as f64) / (height - 1.0);

                let ray = cam.get_ray(u, v);

                let mut rec = hit::HitRecord::new(default_material.clone());

                *p += ray.color(&mut rec, &world, max_depth);
            }

            i += 1;
        }
    }

    let mut raw_pixels = vec![0u8; (4.0 * width * height) as usize];

    let mut i: usize = 0;
    for p in pixels {
        let color = write::png::get_color_u8(&p, samples_per_pixel);
        raw_pixels[i + 0] = color.x;
        raw_pixels[i + 1] = color.y;
        raw_pixels[i + 2] = color.z;
        raw_pixels[i + 3] = 255;
        i += 4;
    }

    raw_pixels
}
