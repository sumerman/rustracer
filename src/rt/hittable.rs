use crate::math::*;

#[derive(PartialEq)]
pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub face: Face,
    pub t: f32,
}

impl Hit {
    pub fn new(point: Point3, t: f32) -> Self {
        Hit {
            point,
            t,
            face: Face::Front,
            normal: Vec3::ONE,
        }
    }

    pub fn orient_hit_normal(mut self, outward_normal: Vec3, r: &Ray) -> Self {
        self.face = if Vec3::dot(r.dir, outward_normal) < 0.0 {
            Face::Front
        } else {
            Face::Back
        };
        self.normal = if self.face == Face::Front {
            outward_normal
        } else {
            -outward_normal
        };

        self
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}