use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

/// Shorthand initializer for Color
pub fn color(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
}

// One line per pixel in PPM format
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.r, self.g, self.b)
    }
}

impl Color {
    /// Writes color in PPM format
    ///
    /// One line with:
    ///
    /// `r g b`
    pub fn to_ppm(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}
