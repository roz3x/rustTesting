#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn negate(&mut self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn add(&mut self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn mul(&mut self, a: f32) -> Self {
        Self {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }

    pub fn div(&mut self, a: f32) -> Self {
        Self {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }

    pub fn length(self) -> f32 {
        self.lenght_squared().sqrt()
    }

    pub fn lenght_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

pub type Color = Vec3;

impl Color {
    pub fn serialize_color(self) -> String {
        let v = self as Vec3;
        format!(
            "{} {} {}\n",
            (v.x * 255.0) as u8,
            (v.y * 255.0) as u8,
            (v.z * 255.0) as u8,
        )
    }
}
