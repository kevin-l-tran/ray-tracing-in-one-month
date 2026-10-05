fn main() {
    let image_width = 256i32;
    let image_height = 256i32;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0f64;

            let int_r = (255.999 * r) as u64;
            let int_g = (255.999 * g) as u64;
            let int_b = (255.999 * b) as u64;

            println!("{} {} {}", int_r, int_g, int_b);
        }
    }
}

