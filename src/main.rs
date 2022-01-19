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

fn main() {
    let mut rng = SmallRng::from_entropy();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 16;
    let image_width = 1280u32;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let mut img_buf = OutputImage::new(image_width, image_height);

    // World
    let mat_ground = rt::Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let mat_center = rt::Material::Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    };
    let mat_left = rt::Material::Metallic {
        albedo: Color::new(0.8, 0.8, 0.8),
        roughness: 0.3,
    };
    let mat_right = rt::Material::Metallic {
        albedo: Color::new(0.8, 0.6, 0.2),
        roughness: 1.0,
    };
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(geometry::Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            mat_ground,
        )),
        Box::new(geometry::Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            mat_center,
        )),
        Box::new(geometry::Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            mat_left,
        )),
        Box::new(geometry::Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            mat_right,
        )),
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
