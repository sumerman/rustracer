use crate::math::*;
use super::hittable::*;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut final_hit = None;
        let mut closest_hit = t_max;

        for o in &self.objects {
            let current_hit = o.hit(r, t_min, closest_hit);
            if let Some(ref hit) = current_hit {
                closest_hit = hit.t;
                final_hit = current_hit;
            }
        }

        final_hit
    }
}
