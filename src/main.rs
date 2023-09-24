use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::utility::{random_double, random_double_range};
use crate::vec3::{Point3, Vec3};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod utility;
mod material;

fn main() {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(sphere::Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Material::Lambertian(albedo)
                } else if choose_material < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    Material::Metal(albedo, fuzz)
                } else {
                    Material::Dielectric(1.5)
                };

                world.add(sphere::Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Material::Dielectric(1.5);
    world.add(sphere::Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(sphere::Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(sphere::Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.5,
        10.0,
    );
    camera.render(&world);
}
