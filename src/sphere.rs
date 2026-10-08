use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, dot},
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t: Interval, record: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;

        let a = dot(r.direction, r.direction);
        let h = dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;
        if root <= t.min || root >= t.max {
            root = (h + sqrt_d) / a;
        }
        if root <= t.min || root >= t.max {
            return false;
        }

        record.t = root;
        record.p = r.at(record.t);

        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);
        record.material = Some(self.material.clone());

        true
    }
}
