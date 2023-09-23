use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod vec3;
mod color;
mod ray;

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction()) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;
    discriminant > 0.0
}

fn ray_color(_r: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, _r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = _r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as u32;

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
            let ray = ray::Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(ray);
            pixel_color.write_color();
        }
    }

    eprintln!("Done.");
}
