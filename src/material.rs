use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{Vec3, dot, reflect_vector, unit_vector},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray {
            origin: hit_record.p,
            direction: scatter_direction,
        };
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect_vector(r_in.direction, hit_record.normal);
        let fuzz_reflected = unit_vector(reflected) + self.fuzz * Vec3::random_unit();

        *scattered = Ray {
            origin: hit_record.p,
            direction: fuzz_reflected,
        };
        *attenuation = self.albedo;

        dot(scattered.direction, hit_record.normal) > 0.0
    }
}
