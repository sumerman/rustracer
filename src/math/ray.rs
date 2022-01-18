use super::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {orig, dir}
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t*self.dir
    }
}