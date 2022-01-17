use crate::math::*;

pub struct Camera {
    origin: Point3,
    lower_left_corener: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point3, aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_len = 1.0;

        let h = Vec3::X * viewport_width;
        let v = Vec3::Y * viewport_height;
        Camera {
            origin,
            horizontal: h,
            vertical: v,
            lower_left_corener: origin - h / 2.0 - v / 2.0 - Vec3::Z * focal_len,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corener + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
