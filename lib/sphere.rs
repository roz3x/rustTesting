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
    pub fn hit_sphere(&self, ray: Ray) -> bool {
        let oc = self.center.clone().remove(ray.origin.clone());
        let a = ray.direction.clone().dot(ray.direction.clone());
        let b = -2.0 * ray.direction.clone().dot(oc.clone());
        let c = oc.clone().dot(oc.clone()) - self.radius * self.radius;
        let discriminent = b * b - 4.0 * a * c;
        discriminent >= 0.0
    }
}
