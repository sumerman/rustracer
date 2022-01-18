use image::*;

pub type ImagePixel = Rgb<u8>;
pub type OutputImage = RgbImage;

pub type Color = glam::Vec3A;

pub fn output_color(px: &mut ImagePixel, c: Color) {
    let int_clr = c.clamp(Color::ZERO, Color::ONE - 0.0001) * 256.0;

    *px = image::Rgb([int_clr.x as u8, int_clr.y as u8, int_clr.z as u8])
}
