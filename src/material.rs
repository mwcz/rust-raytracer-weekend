use crate::random_float;
use crate::Color;
use crate::HitRecord;
use crate::Ray;
use crate::Vec3;
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

pub struct Dielectric<T: Float> {
    pub ir: T,
    pub albedo: Color<T>,
}

trait DielectricReflectance<T> {
    fn reflectance(&self, cosine: T, ref_idx: T) -> T;
}

impl<T: Float> Material<T> for Dielectric<T> {
    fn scatter(
        &self,
        r_in: &Ray<T>,
        rec: &HitRecord<T>,
        attenuation: &mut Color<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        *attenuation = self.albedo;

        let refraction_ratio = if rec.front_face {
            T::one() / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.unit();

        let cos_theta = -unit_direction.dot(&rec.normal).min(T::one());
        let sin_theta = (T::one() - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > T::one();
        let should_reflect = self.reflectance(cos_theta, refraction_ratio) > random_float();

        let direction = if cannot_refract || should_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        *scattered = Ray {
            origin: rec.p,
            direction,
        };

        true
    }
}

impl<T: Float> DielectricReflectance<T> for Dielectric<T> {
    fn reflectance(&self, cosine: T, ref_idx: T) -> T {
        let r0 = (T::one() - ref_idx) / (T::one() + ref_idx);
        let r0 = r0 * r0;

        r0 + (T::one() - r0) * (T::one() - cosine).powi(5)
    }
}
