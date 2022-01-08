use std::io::{self, Write};

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;
    let mut img_buf = image::RgbImage::new(image_width, image_height);

    fn f64_to_u8(c: f64) -> u8 {
        (c * 255.999) as u8
    }

    for j in 0..image_height {
        let scanline = image_height - j - 1;
        eprint!("\rScanlines remaining: {}     ", scanline);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let px = img_buf.get_pixel_mut(i, scanline);
            *px = image::Rgb([f64_to_u8(r), f64_to_u8(g), f64_to_u8(b)])
        }
    }

    img_buf.save("output.png").unwrap();
    eprintln!("\nDone!");
}
