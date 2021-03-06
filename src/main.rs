mod hittable;
mod ray;
mod sphere;

use std::f64::INFINITY;

use hittable::Hittable;
use image::{ImageBuffer, RgbImage};
use nalgebra::{Dot, Norm, Vec3};
use ray::{Point3, Ray};

use crate::{hittable::HittableObjects, sphere::Sphere};

type Color = Vec3<f64>;
type PixelBuf = Vec<u8>;

const IMAGE_WIDTH: u32 = 960;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    // Image
    let aspect_ratio: f64 = ASPECT_RATIO;
    let image_width = IMAGE_WIDTH;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // World
    let mut world = HittableObjects::default();
    let sphere1 = Sphere::new(Point3::new(0.5, 0.0, -1.0), 0.25);
    let sphere2 = Sphere::new(Point3::new(-0.5, 0.0, -1.0), 0.25);
    let ground = Sphere::new(Point3::new(0.,-100.5,-1.), 100.);
    world.add(sphere1);
    world.add(sphere2);
    world.add(ground);

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length: f64 = FOCAL_LENGTH;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut pixel_buf: Vec<u8> = Vec::new();

    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray: Ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let pixel_color: Color = ray_color(ray, &mut world);
            write_color(&mut pixel_buf, pixel_color);
        }
    }

    let buffer: RgbImage = ImageBuffer::from_raw(image_width, image_height, pixel_buf)
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

fn ray_color(ray: Ray, world: &mut HittableObjects) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, INFINITY) {
        return 0.5 * (hit.normal + Color::new(1.,1.,1.));
    }

    // Background color
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let half_b = oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
