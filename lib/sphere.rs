use crate::ray::Ray;
use crate::vec3::{HitRecord, Hittable, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn hit_sphere_simplified(&self, ray: Ray) -> f64 {
        let oc = self.center.clone().remove(ray.origin.clone());
        let a = ray.direction.clone().lenght_squared();
        let h = ray.direction.clone().dot(oc.clone());
        let c = oc.lenght_squared() - self.radius * self.radius;
        let discriminent = h * h - a * c;
        return if discriminent < 0.0 {
            -1.0
        } else {
            (h - discriminent.sqrt()) / a
        };
    }

    // old implementation
    // dont use
    pub fn hit_sphere(&self, ray: Ray) -> f64 {
        let oc = self.center.clone().remove(ray.origin.clone());
        let a = ray.direction.clone().dot(ray.direction.clone());
        let b = -2.0 * ray.direction.clone().dot(oc.clone());
        let c = oc.clone().dot(oc.clone()) - self.radius * self.radius;
        let discriminent = b * b - 4.0 * a * c;

        return if discriminent < 0.0 {
            -1.0
        } else {
            (-b - discriminent.sqrt()) / (2.0 * a)
        };
    }
}

impl Hittable for Sphere {
    fn hit(self, ray: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center.clone().remove(ray.origin.clone());
        let a = ray.direction.clone().lenght_squared();
        let h = ray.direction.clone().dot(oc.clone());
        let c = oc.clone().lenght_squared() - self.radius * self.radius;

        let dis = h * h - a * c;
        if dis < 0. {
            return None;
        }
        let sqrtd = dis.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let mut res = HitRecord {
            p: ray.at(root.clone()),
            t: root.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0), // tmp value
            front_face: false,                // tmp value
        };
        res.normal = res.p.clone().remove(self.center).div(self.radius);
        let outward_normal = res.p.clone().remove(self.center.clone()).div(self.radius);
        res.set_face_normal(&ray, &outward_normal);
        Some(res)
    }
}
