use num::traits::Float;
use pbr::ProgressBar;
use rayon::prelude::*;
use std::sync::Arc;

use rtw_lib::camera::Camera;
use rtw_lib::hit::HitRecord;
use rtw_lib::material::Lambertian;
use rtw_lib::random::random_float;
use rtw_lib::scenes as Scenes;
use rtw_lib::vec::{Color, Point3, Vec3};
use rtw_lib::write::{png, FinalImage};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              MAIN                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

fn render() {
    // Configuration

    // let aspect_ratio = 3.0 / 2.0;
    // let width = 400.0;
    // let height = (width / aspect_ratio).floor();
    // let samples_per_pixel: i32 = 100;
    // let max_depth = 10;

    let aspect_ratio = 3.0 / 2.0;
    let width = 1600.0;
    let height = (width / aspect_ratio).floor();

    // Progress bar
    let mut pb = ProgressBar::new((width * height) as u64);

    // let samples_per_pixel: i32 = 100;
    // let max_depth = 25;

    let samples_per_pixel: i32 = 15;
    let max_depth = 8;

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
    let default_material = Arc::new(Lambertian {
        albedo: Color {
            x: 122.0 / 255.0,
            y: 175.0 / 255.0,
            z: 238.0 / 255.0,
        },
    });

    let mut total_rays: u64 = 0;

    // Original loops
    // for y in (0..(height as i32)).rev() {
    //     for x in 0..(width as i32) {
    //         let p = &mut pixels[i];

    //         for _ in 0..samples_per_pixel {
    //             // don't use RNG if there's only one sample per pixel
    //             let u_rand = if samples_per_pixel > 1 {
    //                 random_float()
    //             } else {
    //                 1.0
    //             };

    //             let v_rand = if samples_per_pixel > 1 {
    //                 random_float()
    //             } else {
    //                 1.0
    //             };

    //             let u = (u_rand + x as f64) / (width - 1.0);
    //             let v = (v_rand + y as f64) / (height - 1.0);

    //             let ray = cam.get_ray(u, v);

    //             let mut rec = HitRecord::new(default_material.clone());

    //             *p += ray.color(&mut rec, &world, max_depth);

    //             total_rays += rec.ray_count;
    //         }

    //         pb.inc();
    //         i += 1;
    //     }
    // }

    // Same loop structure but with ranges, for_each and closures
    // (0..(height as i32)).rev().for_each(|y| {
    //     (0..(width as i32)).for_each(|x| {
    //         let p = &mut pixels[i];

    //         (0..samples_per_pixel).for_each(|_| {
    //             // don't use RNG if there's only one sample per pixel
    //             let u_rand = if samples_per_pixel > 1 {
    //                 random_float()
    //             } else {
    //                 1.0
    //             };

    //             let v_rand = if samples_per_pixel > 1 {
    //                 random_float()
    //             } else {
    //                 1.0
    //             };

    //             let u = (u_rand + x as f64) / (width - 1.0);
    //             let v = (v_rand + y as f64) / (height - 1.0);

    //             let ray = cam.get_ray(u, v);

    //             let mut rec = HitRecord::new(default_material.clone());

    //             *p += ray.color(&mut rec, &world, max_depth);

    //             total_rays += rec.ray_count;
    //         });

    //         pb.inc();
    //         i += 1;
    //     });
    // });

    // pixels.iter() which will eventually (hopefully) lead to par_iter
    let total_rays = pixels
        .par_iter_mut()
        .rev()
        .enumerate()
        .map(|(i, p)| {
            let y = (i as f64 / width).floor();
            let x = i as f64 % width;

            let mut thread_ray_total = 0;

            (0..samples_per_pixel).for_each(|_| {
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

                *p += ray.color(&mut rec, world.clone(), max_depth);

                thread_ray_total += rec.ray_count;
            });

            // println!("yes");
            // pb.inc();

            // println!("thread_ray_total: {}", thread_ray_total);
            thread_ray_total
        })
        .sum();

    println!("Total rays: {}", total_rays);

    png::write(FinalImage {
        width: width as u32,
        height: height as u32,
        pixels,
        samples_per_pixel,
        total_rays,
    });

    pb.finish_print("Done!");
}

fn main() {
    render();
}
