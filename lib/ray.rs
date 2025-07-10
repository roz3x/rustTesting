use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f32) -> Vec3 {
        self.direction.clone().mul(t).add(self.origin)
    }
}
