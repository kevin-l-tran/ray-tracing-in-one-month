use std::io;

use crate::{
    hittable_list::HittableList,
    sphere::Sphere,
    vec3::{Point3},
};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() -> io::Result<()> {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = camera::Camera::new(16.0 / 9.0, 400);
    cam.render(&world)
}
