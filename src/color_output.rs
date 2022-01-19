use image::*;

pub type Subpixel = u16;
pub type ImagePixel = Rgb<Subpixel>;
pub type OutputImage = ImageBuffer<ImagePixel, Vec<Subpixel>>;

pub type Color = glam::Vec3A;

const GAMMA: f32 = 2.0;

#[inline(always)]
pub fn output_color(c: Color) -> Vec<Subpixel> {
    let gamma_corrected = c.powf(1.0 / GAMMA);
    let int_clr = gamma_corrected.clamp(Color::ZERO, Color::ONE - 0.001) * Subpixel::MAX as f32;

    Vec::from(image::Rgb([int_clr.x as u16, int_clr.y as u16, int_clr.z as u16]).channels())
}
