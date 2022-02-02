use crate::math::*;

use super::Hittable;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
    infinite: bool,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Self {
        Aabb {
            min,
            max,
            infinite: false,
        }
    }

    pub fn infinite() -> Self {
        Aabb {
            min: Vec3::splat(f32::NEG_INFINITY),
            max: Vec3::splat(f32::INFINITY),
            infinite: true,
        }
    }

    pub fn surrounding_box(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
            infinite: self.infinite || other.infinite,
        }
    }

    pub fn doubled_centroid(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn surface_area(&self) -> f32 {
        let measurements = self.max - self.min;
        2.0 * (measurements.x * measurements.y
            + measurements.y * measurements.z
            + measurements.z * measurements.x)
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        if self.infinite {
            return true;
        }

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
    aabb: Aabb,
}

impl<T: Hittable> AabbCache<T> {
    #[allow(dead_code)]
    pub fn new(object: T, time_interval: std::ops::Range<f32>) -> Self {
        let mut res = AabbCache {
            object,
            aabb: Aabb::infinite(),
        };
        res.aabb = res.object.bounding_box(time_interval);
        res
    }
}

impl<T: Hittable> Hittable for AabbCache<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<super::Hit> {
        self.object.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time_interval: std::ops::Range<f32>) -> Aabb {
        self.aabb
    }
}
