use nalgebra::Dot;

use crate::ray::{Dir3, Point3, Ray};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Dir3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Dir3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Dir3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
pub trait Hittable {
    // fn hit_record_mut(&mut self) -> &mut HitRecord;
    fn hit(&mut self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableObjects {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableObjects {
    pub fn add<H>(&mut self, object: H)
    where
        H: Hittable + 'static,
    {
        self.objects.push(Box::new(object) as Box<dyn Hittable>);
    }
}
impl Hittable for HittableObjects {
    fn hit(&mut self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_something: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &mut self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_something = Some(rec);
            }
        }

        return hit_something;
    }
}
