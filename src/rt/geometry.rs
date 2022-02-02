use std::ops::Range;

use super::aabb::*;
use super::hittable::*;
use super::material::*;
use crate::math::*;

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
}

pub struct MovingSphere {
    center1: Point3,
    sphere: Sphere,
    time_interval: Range<f32>,
}

pub fn sphere(center: Point3, radius: f32, material: Material) -> Sphere {
    Sphere {
        center,
        radius,
        material,
    }
}

pub fn moving_sphere(sphere: Sphere, center1: Point3, time_interval: Range<f32>) -> MovingSphere {
    MovingSphere {
        sphere,
        center1,
        time_interval,
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        for &root_t in &[(-half_b - sqrtd) / a, (-half_b + sqrtd) / a] {
            if t_min <= root_t && root_t <= t_max {
                let point = r.at(root_t);
                let outward_normal = (point - self.center) / self.radius;
                let hit = Hit::new(point, outward_normal, &self.material, root_t);
                return Some(hit.orient_hit_normal(r));
            }
        }

        None
    }

    fn bounding_box(&self, _time_interval: Range<f32>) -> Aabb {
        Aabb::new(
            self.center - Vec3::splat(self.radius),
            self.center + Vec3::splat(self.radius),
        )
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let motion = match *self {
            MovingSphere {
                time_interval:
                    Range {
                        start: time0,
                        end: time1,
                    },
                sphere: Sphere { center, .. },
                center1,
            } => ((r.time - time0) / (time1 - time0)) * (center1 - center),
        };

        let r1 = Ray::new(r.orig - motion, r.dir, r.time);
        self.sphere
            .hit(&r1, t_min, t_max)
            .map(|h| Hit::new(h.point + motion, h.normal + motion, h.material, h.t))
    }

    fn bounding_box(&self, time_interval: Range<f32>) -> Aabb {
        let offest_aabb = Aabb::new(
            self.center1 - Vec3::splat(self.sphere.radius),
            self.center1 + Vec3::splat(self.sphere.radius),
        );
        self.sphere
            .bounding_box(time_interval)
            .surrounding_box(offest_aabb)
    }
}
