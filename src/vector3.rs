use std::ops;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// ### Helper for initializing a Vector3
pub fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x, y, z }
}

// Operators
impl ops::Add<Vector3> for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Vector3 {
    fn cross(self, other: Self) -> Vector3 {
        Vector3 {
            x: (self.y * other.z) - (self.y * other.z),
            y: (self.z * other.x) - (self.z * other.x),
            z: (self.x * other.y) - (self.x * other.y),
        }
    }

    fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y + other.y) + (self.z + other.z)
    }
}
