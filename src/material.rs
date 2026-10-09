use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{Vec3, dot, reflect_vector, refract_vector, unit_vector},
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

pub struct Dielectric {
    // ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powf(2.0);

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let rel_ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction);
        let cos_theta = f64::min(dot(-1.0 * unit_direction, hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = rel_ri * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract {
            direction = reflect_vector(unit_direction, hit_record.normal);
        } else {
            direction = refract_vector(unit_direction, hit_record.normal, rel_ri);
        }

        *scattered = Ray {
            origin: hit_record.p,
            direction,
        };
        *attenuation = Color::new(1.0, 1.0, 1.0);

        true
    }
}
