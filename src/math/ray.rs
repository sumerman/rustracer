use super::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: f32) -> Ray {
        Ray {orig, dir, time}
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t*self.dir
    }
}