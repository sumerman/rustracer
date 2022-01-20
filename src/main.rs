mod color_output;
mod math;
mod rt;

use color_output::*;
use math::*;
use rand::distributions::Standard as StandardDist;
use rand::prelude::*;
use rayon::prelude::*;
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
    // let mut rng = SmallRng::from_entropy();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 16;
    let image_width = 1280u32;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // World
    let mat_ground = rt::Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let mat_center = rt::Material::Lambertian {
        // albedo: Color::new(0.7, 0.3, 0.3), // red-ish
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    // let mat_left = rt::Material::Metallic {
    //     albedo: Color::new(0.8, 0.8, 0.8),
    //     roughness: 0.3,
    // };
    let mat_left = rt::Material::Dielectric {
        albedo: Color::splat(0.98),
        roughness: 0.005,
        ior: 1.5,
    };
    let mat_right = rt::Material::Metallic {
        albedo: Color::new(0.8, 0.6, 0.2),
        roughness: 0.03,
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
            mat_left.clone(),
        )),
        Box::new(geometry::Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.4,
            mat_left.clone(),
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

    let j_step = 50;
    let mut data_buf: Vec<Subpixel> = Vec::with_capacity((image_height * image_width) as usize);

    (0..image_height).step_by(j_step).for_each(|jst| {
        let jst_upper = u32::min(jst + j_step as u32, image_height);
        report_progress(image_height - jst_upper);

        data_buf.par_extend((jst..jst_upper).into_par_iter().flat_map(|j| {
            (0..image_width)
                .into_par_iter()
                .map(move |i| (i, j)) // move j and only j
                .flat_map_iter(|(i, j)| {
                    let mut color = Color::ZERO;
                    let mut r: Ray;
                    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();

                    for _ in 0..samples_per_pixel {
                        let u_offset: f32 = rng.sample(StandardDist);
                        let v_offset: f32 = rng.sample(StandardDist);

                        let u = (i as f32 + u_offset) / (image_width - 1) as f32;
                        let v = (j as f32 + v_offset) / (image_height - 1) as f32;

                        r = camera.get_ray(u, v);
                        color += ray_color(r, world.as_slice(), &mut rng);
                    }
                    output_color(color * color_scale)
                })
        }));
    });

    let image = OutputImage::from_vec(image_width, image_height, data_buf).unwrap();
    image.save("output.png").unwrap();
    eprintln!("\nDone!");
}
