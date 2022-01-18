use image::*;

pub type ImagePixel = Rgb<u16>;
pub type OutputImage = ImageBuffer<ImagePixel, Vec<u16>>;

pub type Color = glam::Vec3A;

const GAMMA: f32 = 2.0;

pub fn output_color(px: &mut ImagePixel, c: Color) {
    let gamma_corrected = c.powf(1.0 / GAMMA);
    let int_clr = gamma_corrected.clamp(Color::ZERO, Color::ONE - 0.001) * u16::MAX as f32;

    *px = image::Rgb([int_clr.x as u16, int_clr.y as u16, int_clr.z as u16])
}
