use super::*;
use crate::color_output::Color;
use crate::math::*;

use rand::prelude::*;

pub struct MaterialResponse {
    pub new_ray: Ray,
    pub attenuation: Color,
}

#[derive(Clone)]
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
    pub fn scatter(&self, r_in: &Ray, hit: &Hit, rng: &mut impl Rng) -> Option<MaterialResponse> {
        let random_unit_vec = random_on_unit_sphere(rng);
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = hit.normal + random_unit_vec;
                if near_zero(scatter_dir) {
                    scatter_dir = hit.normal;
                }
                Some(MaterialResponse {
                    new_ray: Ray::new(hit.point, scatter_dir, r_in.time),
                    attenuation: *albedo,
                })
            }
            Material::Metallic { albedo, roughness } => {
                let reflected = reflect(r_in.dir.normalize_or_zero(), hit.normal);
                let cos_theta = f32::min(Vec3::dot(-r_in.dir, hit.normal), 1.0);
                let f = reflectance(cos_theta, *albedo);
                if Vec3::dot(reflected, hit.normal) >= 0.0 {
                    Some(MaterialResponse {
                        new_ray: Ray::new(hit.point, reflected + *roughness * random_unit_vec, r_in.time),
                        attenuation: f
                    })
                } else {
                    None
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
                let (scattered, color) =
                    if !valid || reflectance.max_element() > rng.sample(rand::distributions::Standard) {
                        let reflected = reflect(r_in.dir.normalize_or_zero(), hit.normal)
                            + *roughness * random_unit_vec;
                        (reflected, Color::ONE)
                    } else {
                        (refracted + *roughness * random_unit_vec, *albedo)
                    };

                Some(MaterialResponse {
                    new_ray: Ray::new(hit.point, scattered, r_in.time),
                    attenuation: color,
                })
            }
        }
    }
}
