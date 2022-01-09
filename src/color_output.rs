pub type ImagePixel = image::Rgb<u8>;
pub type OutputImage = image::RgbImage;

pub type Color = glam::Vec3;

pub fn output_color(px: &mut ImagePixel, c: Color) {
    fn f32_to_u8(v: f32) -> u8 {
        (v * 255.999) as u8
    }

    *px = image::Rgb([f32_to_u8(c.x), f32_to_u8(c.y), f32_to_u8(c.z)])
}