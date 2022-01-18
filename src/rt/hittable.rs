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
    pub fn new(point: Point3, normal: Vec3, t: f32) -> Self {
        Hit {
            point,
            t,
            face: Face::Front,
            normal: normal,
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

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

impl Hittable for [Box<dyn Hittable>] {
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