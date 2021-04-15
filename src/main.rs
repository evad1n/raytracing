use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static width: usize = 100;
static height: usize = 100;

#[derive(Debug, Default, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// One line per pixel in PPM format
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.r, self.g, self.b)
    }
}

impl Color {
    fn to_ppm(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}

fn main() {
    let c = Color { r: 1, g: 2, b: 3 };

    println!("{:?}", c);
    println!("{}", c);

    let pixels = create_data();

    write_ppm(pixels, "test.ppm");
}

fn create_data() -> Vec<Vec<Color>> {
    let mut pixels: Vec<Vec<Color>> = vec![vec![Color::default(); width as usize]; height as usize];

    for j in (0..height).rev() {
        for i in 0..width {
            let r: f32 = (i as f32) / (width as f32);
            let g: f32 = (j as f32) / (height as f32);
            let b: f32 = ((i * j) as f32) / ((width * height) as f32);

            pixels[i][j] = Color {
                r: (r * 256.0) as u8,
                g: (g * 256.0) as u8,
                b: (b * 256.0) as u8,
            }
        }
    }

    return pixels;
}

fn write_ppm(pixels: Vec<Vec<Color>>, file_name: &str) {
    let path = Path::new(file_name);
    let display = path.display();
    println!("Writing data as PPM to {}", display);

    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // PPM header
    let header = format!("P3\n{} {}\n255\n", width, height);

    let pixel_lines: Vec<String> = pixels
        .into_iter()
        .flat_map(|row| row.into_iter().map(|p| p.to_ppm()))
        .collect();

    let data = [
        header.into_bytes(),
        pixel_lines
            .into_iter()
            .flat_map(|line| line.into_bytes())
            .collect(),
    ]
    .concat();

    match file.write_all(&data) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
