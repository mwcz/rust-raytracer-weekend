mod camera;
mod hit;
mod material;
mod objects;
mod random;
mod ray;
mod scenes;
mod vec;
mod write;

use crate::camera::Camera;
use crate::hit::HitRecord;
use crate::material::Lambertian;
use crate::random::*;
use crate::ray::Ray;
use crate::scenes as Scenes;
use crate::vec::{Color, Point3, Vec3};
use crate::write::png;
use num::traits::Float;
use pbr::ProgressBar;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              MAIN                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

fn render() {
    // Configuration

    let aspect_ratio = 3.0 / 2.0;
    let width = 400.0;
    let height = (width / aspect_ratio).floor();
    let samples_per_pixel: i32 = 100;
    let max_depth = 10;

    // Progress bar
    let mut pb = ProgressBar::new((width * height) as u64);

    // World

    // let world = Scenes::random_scene::scene();
    // let world = Scenes::ten_spheres::scene();
    let world = Scenes::glass_sphere_scene::scene();

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

            pb.inc();
        }
    }

    png::write(write::FinalImage {
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
