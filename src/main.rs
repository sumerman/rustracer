mod color_output;
mod math;
mod rt;

use color_output::*;
use math::*;
use rand::distributions::Standard as StandardDist;
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

fn ray_color<T: Hittable + ?Sized>(mut r: Ray, world: &T, rng: &mut impl Rng) -> Color {
    let white = Color::splat(1.0);
    let skyblue = Color::new(0.5, 0.7, 1.0);
    let mut color = Color::ONE;
    let mut bounces = 0;

    while let Some(hit) = world.hit(&r, 0.001, f32::INFINITY) {
        let target = hit.normal + random_on_unit_sphere(rng);
        r = Ray::new(hit.point, target);
        color *= 0.5;
        bounces += 1;

        if bounces > 50 {
            return Color::ZERO;
        }
    }

    let unit_dir = r.dir.normalize_or_zero();
    let t = 0.5 * (unit_dir.y + 1.0);
    let env_color = white.lerp(skyblue, t);

    color * env_color
}

fn main() {
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
                let u_offset: f32 = rng.sample(StandardDist);
                let v_offset: f32 = rng.sample(StandardDist);

                let u = (i as f32 + u_offset) / (image_width - 1) as f32;
                let v = (j as f32 + v_offset) / (image_height - 1) as f32;

                r = camera.get_ray(u, v);
                color += ray_color(r, world.as_slice(), &mut rng);
            }
            output_color(img_buf.get_pixel_mut(i, scanline), color * color_scale);
        }
    }

    img_buf.save("output.png").unwrap();
    eprintln!("\nDone!");
}
