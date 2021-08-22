//////////////
//  SPHERE  //
//////////////

use crate::hit::HitRecord;
use crate::hit::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Point3;
use num::Float;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Sphere<T: Float> {
    pub center: Point3<T>,
    pub radius: T,
    pub material: Rc<dyn Material<T>>,
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

        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();

        true
    }
}
