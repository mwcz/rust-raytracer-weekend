use std::fmt::Debug;

use crate::ray::Ray;
use crate::Point3;
use crate::Vec3;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             CAMERA                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub struct Camera {
    aspect_ratio: f32,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    /// Create a new Camera.
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let two = 2.0;

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
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        }
    }
}
