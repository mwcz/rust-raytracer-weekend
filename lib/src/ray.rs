use num::traits::Float;
use std::fmt::Debug;

use crate::hit::{HitRecord, HittableList};
use crate::vec::{Color, Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray<T: Float> {
    pub origin: Point3<T>,
    pub direction: Vec3<T>,
}

#[allow(dead_code)]
impl<T: Float + Debug> Ray<T> {
    #[inline]
    pub fn new() -> Ray<T> {
        Ray {
            origin: Point3::zero(),
            direction: Vec3::zero(),
        }
    }

    #[inline]
    pub fn at(&self, t: T) -> Vec3<T> {
        self.origin + self.direction * t
    }

    pub fn color(&self, rec: &mut HitRecord<T>, world: &HittableList<T>, depth: i32) -> Color<T> {
        rec.ray_count += 1;

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
