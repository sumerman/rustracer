use std::io::{self, Write};

type ImagePixel = image::Rgb<u8>;
type OutputImage = image::RgbImage;

type Color = glam::Vec3;
type Point3 = glam::Vec3;

fn output_color(px: &mut ImagePixel, c: Color) {
    fn f32_to_u8(v: f32) -> u8 {
        (v * 255.999) as u8
    }

    *px = image::Rgb([f32_to_u8(c[0]), f32_to_u8(c[1]), f32_to_u8(c[2])])
}

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;
    let mut img_buf = OutputImage::new(image_width, image_height);

    for j in 0..image_height {
        let scanline = image_height - j - 1;
        eprint!("\rScanlines remaining: {}     ", scanline);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let color = Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.25,
            );
            output_color(img_buf.get_pixel_mut(i, scanline), color);
        }
    }

    img_buf.save("output.png").unwrap();
    eprintln!("\nDone!");
}
