use std::fs::remove_file;

use crate::{hittable::HitInfo, image::Pixel, ray::Ray, vector3::Vector3};

pub trait Material {
    fn scatter(&self, _ray: &Ray, _info: &HitInfo) -> Option<(Pixel, Ray)> {
        None
    }
}

#[derive(Default, Clone)]
pub struct Lambertian {
    pub albedo: Pixel,
}

#[derive(Default, Clone)]
pub struct Metal {
    pub albedo: Pixel,
    pub fuzz: f64,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, info: &HitInfo) -> Option<(Pixel, Ray)> {
        let mut dir = info.normal + Vector3::random_unit();

        if dir.near_zero() {
            dir = info.normal;
        }

        let scattered = Ray::new(info.point, dir);

        Some((self.albedo, scattered))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, info: &HitInfo) -> Option<(Pixel, Ray)> {
        let mut reflected = ray.direction.reflect(&info.normal);
        reflected = reflected.unit() + (Vector3::random_unit() * self.fuzz);
        let scattered = Ray::new(info.point, reflected);

        if scattered.direction.dot(info.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
