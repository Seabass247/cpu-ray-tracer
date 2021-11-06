use image::{ImageBuffer, RgbImage};
use nalgebra::Vec3;
mod ray;

type Color = Vec3<f64>;
type PixelBuf = Vec<u8>;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut pixel_buf: Vec<u8> = Vec::new();

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            write_color(&mut pixel_buf, Vec3::new(r , g, b));
        }
    }

    let buffer: RgbImage = ImageBuffer::from_raw(IMAGE_WIDTH, IMAGE_HEIGHT, pixel_buf)
        .expect("Could not create image from buffer");
    buffer
        .save("output.png")
        .expect("Could not save image to file");

    println!("Done.")
}

fn write_color(pixel_buf: &mut PixelBuf, color: Color) {
    let ir = (255.99 * color.x) as u8;
    let ig = (255.99 * color.y) as u8;
    let ib = (255.99 * color.z) as u8;
    let mut pixel = vec![ir, ig, ib];
    pixel_buf.append(&mut pixel);
}