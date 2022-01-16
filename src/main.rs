mod color_output;
mod math;
mod rt;

use color_output::*;
use math::*;
use rt::geometry;
use rt::Hittable;
use std::io::{self, Write};

fn report_progress(scanline: u32) {
    eprint!(
        "\rScanlines remaining: {value:>width$}",
        value = scanline,
        width = 6
    );
    io::stderr().flush().unwrap();
}

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
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
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080u32;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let mut img_buf = OutputImage::new(image_width, image_height);

    // World
    let world = rt::HittableList::new(vec![
        Box::new(geometry::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(geometry::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_len = 1.0;

    let origin = Point3::ZERO;
    let horizontal = Vec3::X * viewport_width;
    let vertical = Vec3::Y * viewport_height;
    let lower_left_corener = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::Z * focal_len;

    // Render
    for j in 0..image_height {
        let scanline = image_height - j - 1;
        report_progress(scanline);

        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corener + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(&r, &world);
            output_color(img_buf.get_pixel_mut(i, scanline), color);
        }
    }

    img_buf.save("output.png").unwrap();
    eprintln!("\nDone!");
}
