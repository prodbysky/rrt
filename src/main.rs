use std::io::BufWriter;

use image::Pixel;
use vector3::{Point3, Vector3};

mod image;
mod ray;
mod vector3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

const FOCAL_LEN: f64 = 1.0;

fn hit_sphere(center: Point3, r: f64, ray: &ray::Ray) -> f64 {
    let oc = center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = ray.direction.dot(oc) * -2.0;
    let c = oc.dot(oc) - r * r;
    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 {
        -1.0
    } else {
        -b - disc.sqrt() / (2.0 * a)
    }
}

fn ray_color(ray: &ray::Ray) -> Pixel {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.4), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vector3::new(0.0, 0.0, -1.0)).unit();
        return Pixel::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_dir = ray.direction.unit();
    let a = 0.5 * (unit_dir.y + 1.0);
    Pixel::from_scalar(1.0) * (1.0 - a) + Pixel::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let camera_center = Point3::from_scalar(0.0);
    let viewport_u = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, FOCAL_LEN) - viewport_u / 2.0 - viewport_v / 2.0;
    let first_pixel_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut image = image::Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let pixel_center =
                first_pixel_loc + (pixel_delta_u * x as f64) + (pixel_delta_v * y as f64);
            let ray_dir = pixel_center - camera_center;

            let ray = ray::Ray::new(pixel_center, ray_dir);

            let color = ray_color(&ray);
            image.data[(x + y * IMAGE_WIDTH) as usize] = color;
        }
    }
    let file = std::fs::File::create("test.ppm").unwrap();
    let mut file = BufWriter::new(file);
    image.write_ppm(&mut file).unwrap();
}
