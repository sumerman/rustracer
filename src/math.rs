pub type Point3 = glam::Vec3A;
pub type Vec3 = glam::Vec3A;

mod ray;
pub use ray::Ray;

use rand::prelude::*;

#[inline]
pub fn random_vec3(rng: &mut impl Rng, min: f32, max: f32) -> Vec3 {
    let values: [f32; 3] = rng.sample(rand::distributions::Standard);
    (max - min) * Vec3::from_slice(&values) + Vec3::splat(min)
}

#[inline]
pub fn random_on_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    random_vec3(rng, -1.0, 1.0).normalize_or_zero()
}

#[inline]
pub fn attenuate(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}

#[inline]
pub fn near_zero(v: Vec3) -> bool {
    v.cmplt(Vec3::splat(1.0e-8)).all()
}
