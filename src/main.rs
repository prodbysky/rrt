mod image;
mod vector3;

fn main() {
    let image = image::Image::new(128, 128);
    let mut file = std::fs::File::create("test.ppm").unwrap();
    image.write_ppm(&mut file).unwrap();
}
