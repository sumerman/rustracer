use crate::math::*;
use rand::prelude::*;

pub struct Camera {
    origin: Point3,
    upper_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        origin: Point3,
        lookat: Point3,
        vup: Vec3,
        fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: Option<f32>,
    ) -> Self {
        let h = f32::tan(f32::to_radians(fov / 2.0));
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focus_dist = focus_dist.unwrap_or_else(|| Vec3::length(origin - lookat));

        let w = Vec3::normalize_or_zero(origin - lookat);
        let u = Vec3::normalize_or_zero(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * -viewport_height * v;
        Camera {
            origin,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
            upper_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut impl Rng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.upper_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
