use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::vec3::Point3;

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

    let material_ground = Material::Lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center = Material::Lambertian(Color::new(0.1, 0.2, 0.5));
    let material_left = Material::Dielectric(1.5);
    let material_right = Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0);

    world.add(sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left));
    world.add(sphere::Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}
