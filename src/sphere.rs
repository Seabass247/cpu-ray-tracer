use nalgebra::Dot;

use crate::{hittable::{HitRecord, Hittable}, ray::Point3};


struct Sphere {
    center: Point3,
    radius: f64,
    hit_record: HitRecord,
}

impl Hittable for Sphere {
    fn hit(&mut self, ray: crate::ray::Ray, t_min: f64, t_max: f64) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        self.hit_record.t = root;
        self.hit_record.p = ray.at(self.hit_record.t);
        self.hit_record.normal = (self.hit_record.p - self.center) / self.radius;
    
        true
    }
}