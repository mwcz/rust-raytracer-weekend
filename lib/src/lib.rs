pub mod camera;
pub mod hit;
pub mod material;
pub mod objects;
pub mod random;
pub mod ray;
pub mod scenes;
pub mod vec;
pub mod write;

use num::traits::Float;
use std::rc::Rc;

/// Example render.
// pub fn render() -> Vec<vec::Vec3<f64>> {
pub fn render() -> Vec<u8> {
    // Configuration

    // let aspect_ratio = 3.0 / 2.0;
    // let width = 400.0;
    // let height = (width / aspect_ratio).floor();
    // let samples_per_pixel: i32 = 100;
    // let max_depth = 10;

    let aspect_ratio = 3.0 / 2.0;
    let width = 100.0;
    let height = (width / aspect_ratio).floor();
    let samples_per_pixel: i32 = 4;
    let max_depth = 2;

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
            z: 0.0
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

    let mut raw_pixels = vec![0u8; (3.0 * width * height) as usize];

    let mut i: usize = 0;
    for p in pixels {
        let color = write::png::get_color_u8(&p, samples_per_pixel);
        raw_pixels[i + 0] = color.x;
        raw_pixels[i + 1] = color.y;
        raw_pixels[i + 2] = color.z;
        i += 3;
    }

    raw_pixels
}
