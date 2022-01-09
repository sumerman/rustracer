use super::*;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(o: Point3, d: Vec3) -> Ray {
        Ray {orig: o, dir: d}
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t*self.dir
    }
}