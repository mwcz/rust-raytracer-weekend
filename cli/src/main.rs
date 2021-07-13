use num::traits::Float;
// use pbr::ProgressBar;
use rtw::random::random_float;

use rtw::camera::Camera;
use rtw::hit::HitRecord;
use rtw::material::Lambertian;
use rtw::scenes as Scenes;
use rtw::vec::{Color, Point3, Vec3};
use rtw::write::{png, FinalImage};

use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              MAIN                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

fn render() {
    // Configuration

    let aspect_ratio = 3.0 / 2.0;
    let width = 100.0;
    let height = (width / aspect_ratio).floor();
    let samples_per_pixel: i32 = 4;
    let max_depth = 2;

    // Progress bar
    // let mut pb = ProgressBar::new((width * height) as u64);

    // World

    // let world = Scenes::random_scene::scene();
    // let world = Scenes::ten_spheres::scene();
    let world = Scenes::glass_sphere_scene::scene();

    // Camera

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

            // pb.inc();
        }
    }

    png::write(FinalImage {
        width: width as u32,
        height: height as u32,
        pixels,
        samples_per_pixel,
    });

    // pb.finish_print("Done!");
}

fn main() {
    render();
}
