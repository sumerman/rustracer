mod camera;
pub mod geometry;
mod hittable;
mod material;
mod aabb;
mod bvh;

pub use camera::*;
pub use hittable::*;
pub use material::*;
pub use aabb::*;
pub use bvh::*;

use crate::color_output::*;
use crate::math::*;
use rand::prelude::*;

pub fn ray_color<T: Hittable + ?Sized>(r: &mut Ray, world: &T, rng: &mut impl Rng) -> Color {
    let white = Color::splat(1.0);
    let skyblue = Color::new(0.5, 0.7, 1.0);
    let mut bounces = 0;

    while let Some(hit) = world.hit(r, 0.001, f32::INFINITY) {
        *r = hit.material.scatter(r, &hit, rng);

        bounces += 1;
        if bounces > 50 {
            break;
        }
    }

    let unit_dir = r.dir.normalize_or_zero();
    let t = 0.5 * (unit_dir.y + 1.0);
    let env_color = white.lerp(skyblue, t);

    attenuate(r.color, env_color)
}
