use crate::{
    hittable::{HitInfo, Hittable},
    hittable_list::HittableList,
    image::{Image, Pixel},
    ray,
    vector3::{Point3, Vector3},
};

use rand::{rngs::ThreadRng, Rng};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    pos: Point3,
    pixel00_pos: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    samples_pp: u32,
    pixel_sample_scale: f64,
    rng: ThreadRng,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_w: u32) -> Self {
        let image_h = (image_w as f64 / aspect_ratio) as u32;
        let focal_len = 1.0;
        let viewport_h = 2.0;
        let samples_pp = 64;
        let viewport_w = viewport_h * (image_w as f64 / image_h as f64);
        let viewport_u = Vector3::new(viewport_w, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_h, 0.0);
        let pixel_delta_u = viewport_u / image_w as f64;
        let pixel_delta_v = viewport_v / image_h as f64;
        let pos = Point3::from_scalar(0.0);
        let viewport_upper_left =
            pos - Vector3::new(0.0, 0.0, focal_len) - viewport_u / 2.0 - viewport_v / 2.0;

        Self {
            aspect_ratio,
            image_width: image_w,
            image_height: image_h,
            pos,
            pixel00_pos: viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5,
            pixel_delta_u,
            pixel_delta_v,
            rng: rand::thread_rng(),
            pixel_sample_scale: 1.0 / samples_pp as f64,
            samples_pp,
        }
    }

    pub fn render(&mut self, world: &HittableList) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);
        println!("Started rendering image: {}x{}", image.w, image.h);

        for y in 0..image.h {
            println!("Progress: {}%", (100 * y) / image.h);
            if y != image.h - 1 {
                print!("\x1B[F");
                print!("\x1B[K");
            }

            for x in 0..image.w {
                let mut result = Pixel::from_scalar(0.0);
                for _ in 0..self.samples_pp {
                    let r = self.get_ray(x, y);
                    result += Camera::ray_color(&r, world);
                }
                result = result * self.pixel_sample_scale;
                image.data[(x + y * image.w) as usize] = result;
            }
        }

        image
    }

    fn ray_color(ray: &ray::Ray, world: &HittableList) -> Pixel {
        let mut info: HitInfo = HitInfo::default();
        if world.hit(ray, 0.0, f64::INFINITY, &mut info) {
            return (info.normal + Pixel::from_scalar(1.0)) * 0.5;
        }

        let unit_dir = ray.direction.unit();
        let a = 0.5 * (unit_dir.y + 1.0);
        Pixel::from_scalar(1.0) * (1.0 - a) + Pixel::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&mut self, i: u32, j: u32) -> ray::Ray {
        // Say my grace
        let offset = self.sample_square();
        let sample = self.pixel00_pos
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));
        ray::Ray::new(self.pos, sample - self.pos)
    }

    fn sample_square(&mut self) -> Vector3 {
        Vector3::new(
            self.rng.gen_range(-0.5..0.5),
            self.rng.gen_range(-0.5..0.5),
            0.0,
        )
    }
}
