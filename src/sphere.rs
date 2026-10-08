use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{Point3, dot},
};

struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;

        let a = dot(r.direction, r.direction);
        let h = dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;
        if root <= tmin || root >= tmax {
            root = (h + sqrt_d) / a;
        }
        if root <= tmin || root >= tmax {
            return false;
        }

        record.t = root;
        record.p = r.at(record.t);

        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);

        true
    }
}
