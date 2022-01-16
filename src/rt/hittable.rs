use crate::math::*;

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32
}

pub trait Hittable {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}