use std::rc::Rc;

use crate::{
    interval::Interval, material::Material, ray::Ray, vec3::{Point3, Vec3, dot},
};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal
        } else {
            self.normal = -1.0 * outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t: Interval, record: &mut HitRecord) -> bool;
}
