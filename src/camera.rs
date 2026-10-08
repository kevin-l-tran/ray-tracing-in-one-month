use std::io::{self, Write};

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::{self, Ray},
    vec3::{Point3, Vec3, unit_vector},
};

pub struct Camera {
    pub aspect_ratio: f64,      // ratio of image height over width
    pub image_width: u32,       // rendered image width in pixel count
    pub samples_per_pixel: u32, // number of random samples per pixel

    image_height: u32,        // rendered image height
    center: Point3,           // camera center
    pixel00_loc: Point3,      // location of pixel (0,0)
    pixel_delta_u: Vec3,      // distance between adjacent horizontal pixels, pointed right
    pixel_delta_v: Vec3,      // distance between adjacent vertical pixels, pointed down
    pixel_samples_scale: f64, // color scale factor for a sum of pixel samples
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut record = HitRecord::default();
    if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut record) {
        return 0.5 * (Color::from(record.normal) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn sample_square() -> Vec3 {
    // returns the vector to a random point in [-0.5,0.5] X [-0.5,0.5]
    Vec3::new(
        rand::random_range(0.0..=1.0) - 0.5,
        rand::random_range(0.0..=1.0) - 0.5,
        0.0,
    )
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let raw_image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if raw_image_height > 1 {
            raw_image_height
        } else {
            1
        };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = Point3::new(0.0, 0.0, 0.0);

        // determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

        // calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let stdout = io::stdout();
        let mut out = stdout.lock();

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            io::stderr().flush().expect("failed to flush");

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, world);
                }

                write_color(&mut out, &(self.pixel_samples_scale * pixel_color))?;
            }
        }

        eprint!("\rDone.                 \n");

        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // construct a camera ray originating from the origin
        // and directed at randomly sampled points around the
        // pixel location i,j

        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray {
            origin: ray_origin,
            direction: ray_direction,
        }
    }
}
