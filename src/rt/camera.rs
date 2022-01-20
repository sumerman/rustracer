use crate::math::*;

pub struct Camera {
    origin: Point3,
    upper_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point3, lookat: Point3, vup: Vec3, fov: f32, aspect_ratio: f32) -> Self {
        let h = f32::tan(f32::to_radians(fov / 2.0));
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::normalize_or_zero(origin - lookat);
        let u = Vec3::normalize_or_zero(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let horizontal = viewport_width * u;
        let vertical = -viewport_height * v;
        Camera {
            origin,
            horizontal,
            vertical,
            upper_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.upper_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
