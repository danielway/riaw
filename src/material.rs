use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
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
            Material::Dielectric(ir) => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face { 1.0 / *ir } else { *ir };

                let unit_direction = r_in.direction().unit_vector();
                let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > rand::random()
                {
                    unit_direction.reflect(rec.normal)
                } else {
                    unit_direction.refract(rec.normal, refraction_ratio)
                };

                *scattered = Ray::new(rec.point, direction);
                true
            }
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
