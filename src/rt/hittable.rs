use crate::math::*;
use super::material::*;

#[derive(PartialEq)]
pub enum Face {
    Front,
    Back,
}

pub struct Hit<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub face: Face,
    pub t: f32,
    pub material: &'a Material,
}

impl Hit<'_> {
    pub fn new(point: Point3, normal: Vec3, material: &'_ Material, t: f32) -> Hit<'_> {
        Hit {
            point,
            t,
            face: Face::Front,
            normal,
            material,
        }
    }

    pub fn orient_hit_normal(mut self, r: &Ray) -> Self {
        if Vec3::dot(r.dir, self.normal) >= 0.0 {
            self.face = Face::Back;
            self.normal *= -1.0;
        }

        self
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        (**self).hit(r, t_min, t_max)
    }
}

impl<T: Hittable> Hittable for [T] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut final_hit = None;
        let mut closest_hit = t_max;

        for o in self {
            let current_hit = o.hit(r, t_min, closest_hit);
            if let Some(ref hit) = current_hit {
                closest_hit = hit.t;
                final_hit = current_hit;
            }
        }

        final_hit
    }
}