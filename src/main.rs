mod color_output;
mod math;
mod rt;

use color_output::*;
use math::*;
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

fn random_scene(rng: &mut impl Rng) -> Vec<Box<dyn Hittable>> {
    let n = 22;
    let mut world: Vec<Box<dyn Hittable>> = Vec::with_capacity(n);

    let mat_ground = rt::Material::Lambertian {
        albedo: Color::splat(0.5),
    };
    world.push(Box::new(geometry::sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    let upper_m: i64 = n as i64 / 2;
    let lower_m: i64 = -upper_m;
    for a in lower_m..upper_m {
        for b in lower_m..upper_m {
            let mat_prob: f32 = rng.sample(StandardDist);
            let center = Point3::new(
                a as f32 + 0.9 * rng.sample::<f32, _>(StandardDist),
                0.2,
                b as f32 + 0.9 * rng.sample::<f32, _>(StandardDist),
            );

            let mat = if mat_prob < 0.8 {
                rt::Material::Lambertian {
                    albedo: Color::from(random_vec3(rng, 0.0, 1.0)),
                }
            } else if mat_prob < 0.95 {
                rt::Material::Metallic {
                    albedo: Color::from(random_vec3(rng, 0.5, 1.0)),
                    roughness: 0.5 * rng.sample::<f32, _>(StandardDist),
                }
            } else {
                rt::Material::Dielectric {
                    albedo: Color::splat(0.95),
                    roughness: 0.01,
                    ior: 1.5,
                }
            };

            if mat_prob < 0.8 {
                let center2 =
                    center + Vec3::new(0.0, rng.sample::<f32, _>(StandardDist) * 0.5, 0.0);
                world.push(Box::new(AabbCache::new(
                    geometry::moving_sphere(geometry::sphere(center, 0.2, mat), center2, 0.0..1.0),
                    0.0..1.0,
                )));
            } else {
                world.push(Box::new(AabbCache::new(
                    geometry::sphere(center, 0.2, mat),
                    0.0..1.0,
                )));
            }
        }
    }

    let mat1 = rt::Material::Dielectric {
        albedo: Color::splat(0.98),
        roughness: 0.01,
        ior: 1.5,
    };
    world.push(Box::new(geometry::sphere(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        mat1,
    )));

    let mat2 = rt::Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    world.push(Box::new(geometry::sphere(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));

    let mat3 = rt::Material::Metallic {
        albedo: Color::new(0.7, 0.6, 0.5),
        roughness: 0.01,
    };
    world.push(Box::new(geometry::sphere(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        mat3,
    )));
    world
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 16;
    let image_width = 1280u32;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // World
    let mut world_rng = SmallRng::seed_from_u64(0xEDADBEEF);
    let world = random_scene(&mut world_rng);

    // Camera
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::ZERO,
        Point3::Y,
        20.0,
        aspect_ratio,
        0.0..1.0,
        0.1,
        Some(10.0),
    );

    // Render
    let color_scale = 1.0 / samples_per_pixel as f32;

    let j_step = 10;
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

                        r = camera.get_ray(u, v, &mut rng);
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
