use crate::ray::{Dir3, Point3, Ray};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Dir3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&mut self, ray: Ray, t_min: f64, t_max: f64) -> bool;
}