use crate::{
    ray,
    vector3::{Point3, Vector3},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct HitInfo {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front: bool,
}

impl HitInfo {
    pub fn set_front(&mut self, ray: &ray::Ray, normal: Vector3) {
        self.front = ray.direction.dot(normal) < 0.0;
        self.normal = if self.front { normal } else { -normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, info: &mut HitInfo) -> bool;
}
