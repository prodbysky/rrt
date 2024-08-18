use crate::hittable::{HitInfo, Hittable};
use crate::material::Material;
use crate::vector3::Vector3;

pub struct Sphere<M: Material> {
    center: Vector3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius),
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let oc = self.center - ray.origin;
        let a = ray.direction.sq_len();
        let h = ray.direction.dot(oc);
        let c = oc.sq_len() - self.radius * self.radius;
        let disc = h * h - a * c;

        if disc < 0.0 {
            return None;
        }

        let rooted = disc.sqrt();

        let mut root = (h - rooted) / a;
        if root <= t_min || t_max <= root {
            root = (h + rooted) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let mut info = HitInfo {
            t: root,
            point: ray.at(root),
            front: false,
            normal: (ray.at(root) - self.center) / self.radius,
            material: &self.material,
        };

        info.set_front(ray, (ray.at(root) - self.center) / self.radius);
        info.normal = (info.point - self.center) / self.radius;

        Some(info)
    }
}
