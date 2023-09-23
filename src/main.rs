use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

fn ray_color(r: Ray, world: &impl hittable::Hittable) -> Color {
    let mut rec = hittable::HitRecord::default();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let mut world = HittableList::new();
    world.add(sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_x = horizontal / image_width;
    let pixel_delta_y = vertical / image_height;

    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - horizontal / 2.0
        - vertical / 2.0;

    let pixel00_location = viewport_upper_left + (pixel_delta_x + pixel_delta_y) * 0.5;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_location + pixel_delta_x * i as f64 + pixel_delta_y * j as f64;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(ray, &world);
            pixel_color.write_color();
        }
    }

    eprintln!("Done.");
}
