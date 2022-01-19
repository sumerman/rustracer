mod camera;
pub mod geometry;
mod hittable;
mod material;

pub use camera::*;
pub use hittable::*;
pub use material::*;

use crate::color_output::*;
use crate::math::*;
use rand::prelude::*;

pub fn ray_color<T: Hittable + ?Sized>(mut r: Ray, world: &T, rng: &mut impl Rng) -> Color {
    let white = Color::splat(1.0);
    let skyblue = Color::new(0.5, 0.7, 1.0);
    let mut color = Color::ONE;
    let mut bounces = 0;

    while let Some(hit) = world.hit(&r, 0.001, f32::INFINITY) {
        if let Some(MaterialResponse {
            attenuation: a,
            new_ray,
        }) = hit.material.scatter(&r, &hit, rng)
        {
            color = attenuate(color, *a);
            r = new_ray;
        } else {
            color = Color::ZERO;
            break;
        }

        bounces += 1;
        if bounces > 50 {
            color = Color::ZERO;
            break;
        }
    }

    let unit_dir = r.dir.normalize_or_zero();
    let t = 0.5 * (unit_dir.y + 1.0);
    let env_color = white.lerp(skyblue, t);

    color * env_color
}
