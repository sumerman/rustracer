mod color_output;
mod math;
mod rt;

use color_output::*;
use math::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use rt::geometry;
use rt::*;
use std::io::{self, Write};

fn report_progress(scanline: u32) {
    eprint!(
        "\rScanlines remaining: {value:>width$}",
        value = scanline,
        width = 6
    );
    io::stderr().flush().unwrap();
}

fn ray_color<T: Hittable + ?Sized>(r: &Ray, world: &T) -> Color {
    let dark_gray = Color::new(0.1, 0.1, 0.1);
    let skyblue = Color::new(0.5, 0.7, 1.0);

    if let Some(hit) = world.hit(r, 0.0, f32::INFINITY) {
        return 0.5 * Color::from(hit.normal + Vec3::ONE);
    }

    let unit_dir = r.dir.normalize_or_zero();
    let t = 0.5 * (unit_dir.y + 1.0);

    (1.0 - t) * dark_gray + t * skyblue
}

fn main() {
    let random_distribution = Uniform::from(0.0..1.0);
    let mut rng = SmallRng::from_entropy();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 64;
    let image_width = 1080u32;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let mut img_buf = OutputImage::new(image_width, image_height);

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(geometry::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(geometry::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    let camera = Camera::new(Point3::ZERO, aspect_ratio);

    // Render
    let color_scale = 1.0 / samples_per_pixel as f32;
    for j in 0..image_height {
        let scanline = image_height - j - 1;
        if scanline % 10 == 0 {
            report_progress(scanline);
        }

        for i in 0..image_width {
            let mut color = Color::ZERO;
            let mut r: Ray;

            for _ in 0..samples_per_pixel {
                let u =
                    (i as f32 + random_distribution.sample(&mut rng)) 
                    / (image_width - 1) as f32;
                let v =
                    (j as f32 + random_distribution.sample(&mut rng)) 
                    / (image_height - 1) as f32;
                r = camera.get_ray(u, v);
                color += ray_color(&r, world.as_slice());
            }
            output_color(img_buf.get_pixel_mut(i, scanline), color * color_scale);
        }
    }

    img_buf.save("output.png").unwrap();
    eprintln!("\nDone!");
}
