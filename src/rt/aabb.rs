use crate::math::*;

use super::Hittable;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub fn surrounding_box(self, other: Aabb) -> Aabb {
        Aabb {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = 1.0 / r.dir;
        let t00 = (self.min - r.orig) * inv_d;
        let t01 = (self.max - r.orig) * inv_d;

        // swap t0_i and t1_i if inv_d_i is negative and thus t0_i > t1_i
        let ltz = inv_d.cmplt(Vec3::ZERO);
        let t0 = Vec3::select(ltz, t01, t00);
        let t1 = Vec3::select(ltz, t00, t01);

        let t_min_1 = t_min.max(t0.max_element());
        let t_max_1 = t_max.min(t1.min_element());

        t_max_1 > t_min_1
    }
}

pub struct AabbCache<T: Hittable> {
    pub object: T,
    aabb: Option<Aabb>,
}

impl<T: Hittable> AabbCache<T> {
    pub fn new(object: T, time_interval: std::ops::Range<f32>) -> Self {
        let mut res = AabbCache { object, aabb: None };
        res.aabb = res.object.bounding_box(time_interval);
        res
    }
}

impl<T: Hittable> Hittable for AabbCache<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<super::Hit> {
        self.object.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time_interval: std::ops::Range<f32>) -> Option<Aabb> {
        self.aabb
    }
}
