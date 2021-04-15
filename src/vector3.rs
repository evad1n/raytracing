use std::ops;

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

// Operators
impl ops::Add<Vector3> for Vector3 {
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
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

impl Vector3 {}
