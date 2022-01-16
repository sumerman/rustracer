mod color_output;
mod math;

use color_output::*;
use math::*;
use std::io::{self, Write};

fn report_progress(scanline: u32) {
    eprint!(
        "\rScanlines remaining: {value:>width$}",
        value = scanline,
        width = 6
    );
    io::stderr().flush().unwrap();
}

fn ray_color(r: &Ray) -> Color {
    let white = Color::new(1.0, 1.0, 1.0);
    let skyblue = Color::new(0.5, 0.7, 1.0);

    let shpere_center = Point3::new(0.0, 0.0, -1.0);

    let t = hit_sphere(&shpere_center, 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - shpere_center).normalize_or_zero();
        return 0.5*Color::from(n + Vec3::ONE);
    }

    let unit_dir = r.dir.normalize_or_zero();
    let t = 0.5 * (unit_dir.y + 1.0);

    (1.0 - t) * white + t * skyblue
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.orig - *center;
    let a = r.dir.length_squared();
    let half_b = Vec3::dot(oc, r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080u32;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let mut img_buf = OutputImage::new(image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_len = 1.0;

    let origin = Point3::ZERO;
    let horizontal = Vec3::X * viewport_width;
    let vertical = Vec3::Y * viewport_height;
    let lower_left_corener = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::Z * focal_len;

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
            let color = ray_color(&r);
            output_color(img_buf.get_pixel_mut(i, scanline), color);
        }
    }

    img_buf.save("output.png").unwrap();
    eprintln!("\nDone!");
}
