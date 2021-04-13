use num::traits::Float;
use std::fmt::Debug;

use crate::ray::Ray;
use crate::Point3;
use crate::Vec3;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             CAMERA                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub struct Camera<T: Float + Debug> {
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
    pub fn new(
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
    pub fn get_ray(&self, s: T, t: T) -> Ray<T> {
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
