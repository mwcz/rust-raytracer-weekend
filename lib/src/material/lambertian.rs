use crate::material::Material;
use crate::Color;
use crate::HitRecord;
use crate::Ray;
use crate::Vec3;
use num::traits::Float;

pub struct Lambertian<T: Float> {
    pub albedo: Color<T>,
}

impl<T: Float> Material<T> for Lambertian<T> {
    fn scatter(
        &self,
        _r_in: &Ray<T>,
        rec: &HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::<T>::random_unit_vector();

        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        let scatter_ray = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };

        *scattered = scatter_ray;

        *attenuation = self.albedo;

        true
    }
}
