use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static width: i32 = 256;
static height: i32 = 256;

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.r, self.g, self.b)
    }
}

fn main() {
    // output_ppm();

    let c = Color { r: 1, g: 2, b: 3 };

    println!("{:?}", c);
    println!("{}", c);
}

fn output_ppm() {
    let (w, h) = (256, 256);

    println!("P3\n{} {}\n255", w, h);

    for j in (0..h).rev() {
        println!("{} remaining...", h - j);
        for i in 0..w {
            let r: f32 = (i as f32) / (w as f32);
            let g: f32 = (j as f32) / (h as f32);
            let b: f32 = ((i * j) as f32) / ((w * h) as f32);

            println!(
                "{} {} {}",
                (r * 256.0) as i32,
                (g * 256.0) as i32,
                (b * 256.0) as i32
            );
        }
    }
}

// fn create_data() -> Vec<Vec<u8>> {
//     let mut pixels = vec![vec![0; width as usize]; height as usize];

//     for j in (0..height).rev() {
//         for i in 0..width {
//             let r: f32 = (i as f32) / (width as f32);
//             let g: f32 = (j as f32) / (height as f32);
//             let b: f32 = ((i * j) as f32) / ((width * height) as f32);

//             pixels[i][j] = println!(
//                 "{} {} {}",
//                 (r * 256.0) as i32,
//                 (g * 256.0) as i32,
//                 (b * 256.0) as i32
//             );
//         }
//     }

//     return pixels;
// }

// fn write_ppm(width: i32, height: i32, pixels: Vec<u8>, file_name: &str) {
//     let path = Path::new(file_name);
//     let display = path.display();
//     println!("Writing data as PPM to {}", display);

//     let mut file = match File::create(path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why),
//         Ok(file) => file,
//     };

//     // PPM header
//     let header = format!("P3\n{} {}\n255", width, height);

//     let data = [header.into_bytes(), pixels].concat();

//     match file.write_all(&data) {
//         Err(why) => panic!("couldn't write to {}: {}", display, why),
//         Ok(_) => println!("successfully wrote to {}", display),
//     }
// }
