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

fn random_scene(rng: &mut impl Rng) -> Vec<geometry::Sphere> {
    let n = 22;
    let mut world = Vec::with_capacity(n);

    let mat_ground = rt::Material::Lambertian {
        albedo: Color::splat(0.5),
    };
    world.push(geometry::Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    ));

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

            world.push(geometry::Sphere::new(center, 0.2, mat));
        }
    }

    let mat1 = rt::Material::Dielectric {
        albedo: Color::splat(0.98),
        roughness: 0.01,
        ior: 1.5,
    };
    world.push(geometry::Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = rt::Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    world.push(geometry::Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    ));

    let mat3 = rt::Material::Metallic {
        albedo: Color::new(0.7, 0.6, 0.5),
        roughness: 0.01,
    };
    world.push(geometry::Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));
    world
}

fn crystal_ball(rng: &mut impl Rng) -> Vec<geometry::Sphere> {
    let mut world = Vec::new();

    let mat_ground = rt::Material::Lambertian {
        albedo: Color::splat(0.3),
    };
    world.push(geometry::Sphere::new(
        Point3::new(0.0, -1004.0, 0.0),
        1000.0,
        mat_ground,
    ));

    let mat1 = rt::Material::Dielectric {
        albedo: Color::new(0.7, 0.0, 0.9),
        roughness: 0.08,
        ior: 1.5,
    };
    world.push(geometry::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        4.0,
        mat1,
    ));

    let center = Point3::new(0.0, 0.0, -1.0);
    let n = 24;
    for _ in 0..2*(n * n) {
        let mat_prob: f32 = rng.sample(StandardDist);
        let random_unit_vec = random_on_unit_sphere(rng);

        let mat = if mat_prob < 0.8 {
            rt::Material::Metallic {
                albedo: Color::from(random_vec3(rng, 0.1, 1.0)),
                roughness: 0.5 * rng.sample::<f32, _>(StandardDist),
            }
        } else {
            rt::Material::Dielectric {
                albedo: Color::splat(0.95),
                roughness: 0.01,
                ior: 1.5,
            }
        };

        world.push(geometry::Sphere::new(
            center + random_unit_vec * 55.0,
            0.32,
            mat.clone(),
        ));
    }
    for _ in 0..5 * n {
        let mat_prob: f32 = rng.sample(StandardDist);
        let sphere_offset: f32 = 2.0 + rng.sample::<f32, _>(StandardDist) * 1.5;
        let random_unit_vec = random_on_unit_sphere(rng);

        let mat = if mat_prob < 0.5 {
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

        world.push(geometry::Sphere::new(
            center + random_unit_vec * sphere_offset,
            0.1,
            mat.clone(),
        ));
    }
    world
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 512;
    let image_width = 1280u32;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // World
    let mut world_rng = SmallRng::seed_from_u64(0xEDADBEEF);
    let world = crystal_ball(&mut world_rng);

    // Camera
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 6.0),
        Point3::ZERO,
        Point3::Y,
        75.0,
        aspect_ratio,
        0.25,
        Some(6.0),
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
