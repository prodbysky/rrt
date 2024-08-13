use crate::vector3::{Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
