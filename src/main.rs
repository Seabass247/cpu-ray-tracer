
use std::io::Write;

type Color = (u8, u8, u8);

fn main() {
    let imgx = 256;
    let imgy = 256;

    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(imgx, imgy);
    
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let y = (imgy - 1) - y;
        let r = x as f32 / imgx as f32;
        let g = y as f32 / imgy as f32;
        let b = 0.25;

        let r = (r * 255.999) as u8;
        let g = (g * 255.999) as u8;
        let b = (b * 255.999) as u8;
        
        *pixel = image::Rgb([r, g, b]);
    }

    

    imgbuf.save("output.png").expect("Couldn't save image buffer");

    println!("Image finished." );
}