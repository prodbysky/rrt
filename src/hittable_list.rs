use crate::hittable::{HitInfo, Hittable};

#[derive(Default)]
pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.0.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let mut hit = None;
        let mut closest = t_max;

        for obj in &self.0 {
            if let Some(info) = obj.hit(ray, t_min, closest) {
                closest = info.t;
                hit = Some(info);
            }
        }
        hit
    }
}
