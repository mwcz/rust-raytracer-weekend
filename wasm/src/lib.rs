mod utils;

use rtw;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use num::traits::Float;
use rtw::random::random_float;

use rtw::camera::Camera;
use rtw::hit::HitRecord;
use rtw::material::Lambertian;
use rtw::scenes as Scenes;
use rtw::vec::{Color, Point3, Vec3};
use rtw::write::png::get_color_u8;

use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              MAIN                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Example render.
#[wasm_bindgen]
pub fn render() -> Vec<u8> {
    // Configuration

    // let aspect_ratio = 3.0 / 2.0;
    // let width = 400.0;
    // let height = (width / aspect_ratio).floor();
    // let samples_per_pixel: i32 = 100;
    // let max_depth = 10;

    let aspect_ratio = 3.0 / 2.0;
    let width = 500.0;
    let height = (width / aspect_ratio).floor();

    // let samples_per_pixel: i32 = 100;
    // let max_depth = 25;

    let samples_per_pixel: i32 = 10;
    let max_depth = 5;

    // World

    // let world = Scenes::random_scene::scene();
    // let world = Scenes::ten_spheres::scene();
    let world = Scenes::three_sphere_scene::scene();

    // Camera

    let lookfrom = Point3 {
        x: 0.0,
        y: 1.8,
        z: 1.4,
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
            z: 0.0,
        };
        (width * height) as usize
    ];

    // Default material to clone into each HitRecord
    let default_material = Rc::new(Lambertian {
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
        }
    }

    let mut raw_pixels = vec![0u8; (4.0 * width * height) as usize];

    let mut i: usize = 0;
    for p in pixels {
        let color = get_color_u8(&p, samples_per_pixel);
        raw_pixels[i + 0] = color.x;
        raw_pixels[i + 1] = color.y;
        raw_pixels[i + 2] = color.z;
        raw_pixels[i + 3] = 255;
        i += 4;
    }

    raw_pixels
}
