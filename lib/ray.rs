use crate::sphere;
use crate::sphere::Sphere;
use crate::vec3::Color;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.direction.clone().mul(t).add(self.origin)
    }

    pub fn get_color(self) -> Color {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
        let t = sphere.hit_sphere_simplified(self);
        if t > 0.0 {
            let n: Vec3 = self.at(t).clone().unit_vec().remove(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            });
            return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0).mul(0.5);
        }

        let a = (self.direction.clone().unit_vec().y + 1.0) * 0.5;

        Color::new(1.0, 1.0, 1.0)
            .mul(1.0 - a)
            .add(Color::new(0.5, 0.7, 1.0).mul(a))
    }
}
