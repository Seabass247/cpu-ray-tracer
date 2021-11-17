use nalgebra::Vec3;

pub type Point3 = Vec3<f64>;
pub type Dir3 = Vec3<f64>;

pub struct Ray {
    pub origin: Point3,
    pub direction: Dir3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t*self.direction
    }
}