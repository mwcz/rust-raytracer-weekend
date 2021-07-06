mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::Color;
use crate::HitRecord;
use crate::Ray;
use num::traits::Float;

pub trait Material<T: Float> {
    fn scatter(
        &self,
        r_in: &Ray<T>,
        rec: &HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool;
}
