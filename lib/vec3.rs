#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

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

    pub fn mul(&mut self, a: f64) -> Self {
        Self {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }

    pub fn div(&mut self, a: f64) -> Self {
        Self {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }

    pub fn length(self) -> f64 {
        self.lenght_squared().sqrt()
    }

    pub fn lenght_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vec(self) -> Self {
        self.clone().div(self.length())
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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
