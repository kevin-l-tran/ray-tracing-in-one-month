use std::io;
use std::io::Write;

use crate::{
    color::{Color, write_color},
    ray::Ray,
    vec3::{Point3, Vec3, dot, unit_vector},
};

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = center - r.origin;

    let a = dot(r.direction, r.direction);
    let b = -2.0 * dot(r.direction, oc);
    let c = dot(oc, oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c; // from quadratic formula
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    /*
     **********************
     * IMAGE
     **********************
     */

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400i32;

    let raw_image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if raw_image_height > 1 {
        raw_image_height
    } else {
        1
    };

    /*
     **********************
     * CAMERA
     **********************
     */

    // viewport may not match the aspect ratio exactly since image_width and image_height are ints,
    // so we calculate it from image_width and image_height (the true ratio)
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    /*
     **********************
     * RENDER
     **********************
     */

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush().expect("failed to flush");

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color = ray_color(&r);

            write_color(&mut out, &pixel_color)?;
        }
    }

    Ok(())
}
