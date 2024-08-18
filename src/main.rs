use std::io::{BufWriter, Write};

use hittable_list::HittableList;
use image::Pixel;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vector3::Point3;

mod camera;
mod hittable;
mod hittable_list;
mod image;
mod material;
mod ray;
mod sphere;
mod vector3;

fn main() {
    let mut world: HittableList = HittableList::default();
    let metal = Metal {
        albedo: Pixel::from_scalar(0.5),
        fuzz: 0.25,
    };
    let lambertian = Lambertian {
        albedo: Pixel::new(0.12, 0.5, 0.0),
    };
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -0.65, -3.0),
        1.0,
        metal.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -101.5, -3.0),
        100.0,
        lambertian,
    )));

    let mut cam = camera::Camera::new(16.0 / 9.0, 1920);

    let image = cam.render(&world);

    println!("Writing image to disk");
    let mut file = BufWriter::new(std::fs::File::create("test.ppm").unwrap());
    write!(file, "{}", image).unwrap();
}
