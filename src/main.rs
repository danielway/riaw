use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::vec3::Point3;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;

fn main() {
    let mut world = HittableList::new();
    world.add(sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);
}
