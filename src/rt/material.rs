use super::*;
use crate::color_output::Color;
use crate::math::*;

use rand::prelude::*;

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian {
        albedo: Color,
    },
    Metallic {
        albedo: Color,
        roughness: f32,
    },
    Dielectric {
        albedo: Color,
        roughness: f32,
        ior: f32,
    },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, hit: &Hit, rng: &mut impl Rng) -> Ray {
        let random_unit_vec = random_on_unit_sphere(rng);
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = hit.normal + random_unit_vec;
                if near_zero(scatter_dir) {
                    scatter_dir = hit.normal;
                }
                Ray {
                    orig: hit.point,
                    dir: scatter_dir,
                    ..*r_in
                }
                .attenuate(*albedo)
            }
            Material::Metallic { albedo, roughness } => {
                let reflected = reflect(r_in.dir.normalize_or_zero(), hit.normal);
                let cos_theta = f32::min(Vec3::dot(-r_in.dir, hit.normal), 1.0);
                let f = reflectance(cos_theta, *albedo);
                let r = Ray {
                    orig: hit.point,
                    dir: reflected + *roughness * random_unit_vec,
                    ..*r_in
                }
                .attenuate(f);

                if Vec3::dot(reflected, hit.normal) < 0.0 {
                    Ray { color: Color::ZERO, ..r }
                } else {
                    r
                }
            }
            Material::Dielectric {
                albedo,
                roughness,
                ior,
            } => {
                let refraction_ratio = if hit.face == Face::Front {
                    1.0 / ior
                } else {
                    *ior
                };
                let (refracted, reflectance, valid) =
                    refract(r_in.dir.normalize_or_zero(), hit.normal, refraction_ratio);
                let (scattered, color) = if !valid
                    || reflectance.max_element() > rng.sample(rand::distributions::Standard)
                {
                    let reflected = reflect(r_in.dir.normalize_or_zero(), hit.normal)
                        + *roughness * random_unit_vec;
                    (reflected, Color::ONE)
                } else {
                    (refracted + *roughness * random_unit_vec, *albedo)
                };

                Ray {
                    orig: hit.point,
                    dir: scattered,
                    ..*r_in
                }
                .attenuate(color)
            }
        }
    }
}
