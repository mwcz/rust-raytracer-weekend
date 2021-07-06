////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            HITTABLES                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};
use num::Float;
use std::fmt::Debug;
use std::rc::Rc;

pub struct HitRecord<T: Float> {
    pub p: Point3<T>,
    pub normal: Vec3<T>,
    pub material: Rc<dyn Material<T>>,
    pub t: T,
    pub front_face: bool,
}

impl<T: Float> HitRecord<T> {
    pub fn new(material: Rc<dyn Material<T>>) -> HitRecord<T> {
        HitRecord {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: T::zero(),
            front_face: false,
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray<T>, outward_normal: Vec3<T>) {
        self.front_face = ray.direction.dot(&outward_normal) < T::zero();
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -T::one()
        };
    }
}

pub trait Hittable<T: Float> {
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool;
}

/////////////////////
//  HITTABLE LIST  //
/////////////////////

pub struct HittableList<T: Float> {
    objects: Vec<Box<dyn Hittable<T>>>,
}

#[allow(dead_code)]
impl<T: Float + Debug> HittableList<T> {
    pub fn new(objects: Vec<Box<dyn Hittable<T>>>) -> HittableList<T> {
        HittableList::<T> { objects }
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Hittable<T>>) {
        self.objects.push(obj);
    }

    pub fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = rec;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
