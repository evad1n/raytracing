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
        other * self
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        self * (1.0 / other)
    }
}

// Assignments
impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Vector3 {
    pub fn mag(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn normalized(self) -> Vector3 {
        self / self.mag()
    }

    /// Mutates the underlying vector
    pub fn normalize(&mut self) {
        *self /= self.mag()
    }

    pub fn cross(self, other: Self) -> Vector3 {
        Vector3 {
            x: (self.y * other.z) - (self.y * other.z),
            y: (self.z * other.x) - (self.z * other.x),
            z: (self.x * other.y) - (self.x * other.y),
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y + other.y) + (self.z + other.z)
    }
}
