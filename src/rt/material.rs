use super::*;
use crate::color_output::Color;
use crate::math::*;

use rand::prelude::*;

pub struct MaterialResponse<'a> {
    pub new_ray: Ray,
    pub attenuation: &'a Color,
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
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = hit.normal + random_on_unit_sphere(rng);
                if near_zero(scatter_dir) {
                    scatter_dir = hit.normal;
                }
                Some(MaterialResponse {
                    new_ray: Ray::new(hit.point, scatter_dir),
                    attenuation: albedo,
                })
            }
            Material::Metallic { albedo, roughness } => {
                let reflected = reflect(r_in.dir.normalize_or_zero(), hit.normal)
                    + *roughness * random_on_unit_sphere(rng);
                if Vec3::dot(reflected, hit.normal) > 0.0 {
                    Some(MaterialResponse {
                        new_ray: Ray::new(hit.point, reflected),
                        attenuation: albedo,
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
                let scattered = if !valid || reflectance > rng.sample(rand::distributions::Standard)
                {
                    reflect(r_in.dir.normalize_or_zero(), hit.normal)
                        + *roughness * random_on_unit_sphere(rng)
                } else if *roughness > 0.0 {
                    refracted + *roughness * random_on_unit_sphere(rng)
                } else {
                    refracted
                };
                Some(MaterialResponse {
                    new_ray: Ray::new(hit.point, scattered),
                    attenuation: albedo,
                })
            }
        }
    }
}
