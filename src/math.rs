pub type Point3 = glam::Vec3A;
pub type Vec3 = glam::Vec3A;

mod ray;
pub use ray::Ray;

use rand::prelude::*;

pub fn random_vec3(rng: &mut impl Rng, min: f32, max: f32) -> Vec3 {
    let values: [f32; 3] = rng.sample(rand::distributions::Standard);
    (max - min) * Vec3::from_slice(&values) + Vec3::splat(min)
}

pub fn random_on_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    random_vec3(rng, -1.0, 1.0).normalize_or_zero()
}
