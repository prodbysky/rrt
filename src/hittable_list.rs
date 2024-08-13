use crate::{
    hittable::{HitInfo, Hittable},
    vector3::Vector3,
};

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
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, info: &mut HitInfo) -> bool {
        let mut temp_info = HitInfo {
            point: Vector3::from_scalar(0.0),
            normal: Vector3::from_scalar(0.0),
            t: 0.0,
            front: false,
        };
        let mut hit = false;
        let mut closest = t_max;

        for obj in &self.0 {
            if obj.hit(ray, t_min, closest, &mut temp_info) {
                hit = true;
                closest = temp_info.t;
                *info = temp_info;
            }
        }
        hit
    }
}
