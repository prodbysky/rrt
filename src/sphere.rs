use crate::hittable::Hittable;
use crate::vector3::Vector3;

pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius),
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        info: &mut crate::hittable::HitInfo,
    ) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.sq_len();
        let h = ray.direction.dot(oc);
        let c = oc.sq_len() - self.radius * self.radius;
        let disc = h * h - a * c;

        if disc < 0.0 {
            return false;
        }

        let rooted = disc.sqrt();

        let mut root = (h - rooted) / a;

        if root <= t_min || t_max <= root {
            root = (h + rooted) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        info.t = root;
        info.point = ray.at(info.t);
        let outward = (info.point - self.center) / self.radius;
        info.set_front(ray, outward);
        info.normal = (info.point - self.center) / self.radius;

        true
    }
}
