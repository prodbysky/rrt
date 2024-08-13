use std::io::BufWriter;

use hittable_list::HittableList;
use sphere::Sphere;
use vector3::Point3;

mod camera;
mod hittable;
mod hittable_list;
mod image;
mod ray;
mod sphere;
mod vector3;

fn main() {
    let mut world: HittableList = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -3.0), 1.0)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -101.5, -3.0), 100.0)));

    let mut cam = camera::Camera::new(16.0 / 9.0, 1920);

    let image = cam.render(&world);

    println!("Writing image to disk");
    let mut file = BufWriter::new(std::fs::File::create("test.ppm").unwrap());
    image.write_ppm(&mut file).unwrap();
}
