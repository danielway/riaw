use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_location: Point3,
    pixel_delta_x: Vec3,
    pixel_delta_y: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let center = Point3::new(0.0, 0.0, 0.0);

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_x = horizontal / image_width;
        let pixel_delta_y = vertical / image_height;

        let viewport_upper_left = center
            - Vec3::new(0.0, 0.0, focal_length)
            - horizontal / 2.0
            - vertical / 2.0;

        let pixel00_location = viewport_upper_left + (pixel_delta_x + pixel_delta_y) * 0.5;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_location,
            pixel_delta_x,
            pixel_delta_y,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_location + self.pixel_delta_x * i + self.pixel_delta_y * j;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(ray, world);
                pixel_color.write_color();
            }
        }

        eprintln!("Done.");
    }

    fn ray_color(&self, r: Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}