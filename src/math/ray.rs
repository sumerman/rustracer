use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f32,
    pub color: Color,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: f32) -> Ray {
        Ray {
            orig,
            dir,
            time,
            ..Default::default()
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn attenuate(mut self, incoming_color: Color) -> Self {
        self.color = attenuate(self.color, incoming_color);
        self
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            orig: Vec3::ZERO,
            dir: Vec3::ONE,
            time: 0.0,
            color: Color::ONE - Color::splat(0.001),
        }
    }
}
