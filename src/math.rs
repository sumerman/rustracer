pub type Point3 = glam::Vec3A;
pub type Vec3 = glam::Vec3A;

mod ray;
pub use ray::Ray;

use rand::prelude::*;

use crate::color_output::Color;

#[inline(always)]
pub fn random_vec3(rng: &mut impl Rng, min: f32, max: f32) -> Vec3 {
    let values: [f32; 3] = rng.sample(rand::distributions::Standard);
    (max - min) * Vec3::from_slice(&values) + Vec3::splat(min)
}

#[inline(always)]
pub fn random_on_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    random_vec3(rng, -1.0, 1.0).normalize_or_zero()
}

#[inline(always)]
pub fn attenuate(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

#[inline(always)]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}

#[inline(always)]
pub fn near_zero(v: Vec3) -> bool {
    v.cmplt(Vec3::splat(1.0e-8)).all()
}

// Schlick reflectance approximation.
#[inline(always)]
pub fn reflectance(cosine: f32, f0: Vec3) -> Color {
    f0 + (Color::ONE - f0) * f32::powi(1.0 - cosine, 5)
}

#[inline(always)]
pub fn refract(r_in: Vec3, n: Vec3, etai_over_etat: f32) -> (Vec3, Color, bool) {
    let cos_theta = f32::min(Vec3::dot(-r_in, n), 1.0);
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let r_out_perp = etai_over_etat * (r_in + cos_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * n;

    let r0 = (1.0 - etai_over_etat) / (1.0 + etai_over_etat);
    let r0v = Vec3::splat(r0 * r0);
    (
        r_out_perp + r_out_parallel,
        reflectance(cos_theta, r0v),
        etai_over_etat * sin_theta <= 1.0,
    )
}
