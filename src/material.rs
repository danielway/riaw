use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                *scattered = Ray::new(rec.point, scatter_direction);
                *attenuation = *albedo;
                true
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = r_in.direction().unit_vector().reflect(rec.normal);
                *scattered = Ray::new(rec.point, reflected + Vec3::random_unit_vector() * *fuzz);
                *attenuation = *albedo;
                scattered.direction().dot(rec.normal) > 0.0
            }
        }
    }
}