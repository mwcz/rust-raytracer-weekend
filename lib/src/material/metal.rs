use crate::material::Material;
use crate::Color;
use crate::HitRecord;
use crate::Ray;
use crate::Vec3;
use num::traits::Float;

pub struct Metal<T: Float> {
    pub albedo: Color<T>,
    pub fuzz: T,
}

impl<T: Float> Material<T> for Metal<T> {
    fn scatter(
        &self,
        r_in: &Ray<T>,
        rec: &HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        let reflected = r_in.direction.unit().reflect(rec.normal);

        *scattered = Ray {
            origin: rec.p,
            direction: reflected + Vec3::<T>::random_in_unit_sphere() * self.fuzz,
        };

        *attenuation = self.albedo;

        scattered.direction.dot(&rec.normal) > T::zero()
    }
}
