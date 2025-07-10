use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn At(self, t: f32) -> Vec3 {
        let x = self.direction.clone().mul(t);
        let y = self.origin;
        return x.clone().add(y);
    }
}
