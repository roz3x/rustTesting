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
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 0.5);

        if sphere.hit_sphere(self) {
            return Color::new(0.55, 0.4, 0.55);
        }

        let a = (self.direction.clone().unit_vec().y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0)
            .mul(1.0 - a)
            .add(Color::new(0.5, 0.7, 1.0).mul(a))
    }
}
