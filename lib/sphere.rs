use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn hit_sphere_simplified(&self, ray: Ray ) -> f64 {
        let oc = self.center.clone().remove(ray.origin.clone());
        let a = ray.direction.clone().lenght_squared();
        let h = ray.direction.clone().dot(oc.clone());
        let c = oc.lenght_squared() - self.radius * self.radius;
        let discriminent = h*h - a*c; 
        return if discriminent < 0.0 {
             -1.0
        } else {
            (h - discriminent.sqrt()) / a
        }
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
