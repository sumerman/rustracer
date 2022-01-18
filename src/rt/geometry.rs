use super::hittable::*;
use crate::math::*;

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
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
                let hit = Hit::new(point, outward_normal, root_t);
                return Some(hit.orient_hit_normal(r));
            }
        }

        return None;
    }
}
