use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::{degrees_to_radians, random_double};
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_location: Point3,
    pixel_delta_x: Vec3,
    pixel_delta_y: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        look_from: Point3,
        look_to: Point3,
        vup: Vec3,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        let center = look_from;

        let focal_length = (look_from - look_to).length();
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = (image_width as f64 / image_height as f64) * viewport_height;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left = center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            image_width,
            image_height,
            center,
            pixel00_location,
            pixel_delta_x: pixel_delta_u,
            pixel_delta_y: pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += ray_color(ray, self.max_depth, world);
                }
                pixel_color.write_color(self.samples_per_pixel);
            }
        }

        eprintln!("Done.");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_location + self.pixel_delta_x * i + self.pixel_delta_y * j;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        self.pixel_delta_x * px + self.pixel_delta_y * py
    }
}

fn ray_color(r: Ray, depth: u32, world: &dyn Hittable) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    let mut rec = HitRecord::default();
    if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered) {
            return ray_color(scattered, depth - 1, world) * attenuation;
        }

        return Color::zero();
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
