use std::fs::{File, OpenOptions};
use std::io::Write;

#[path = "../lib/ray.rs"]
mod ray;
#[path = "../lib/vec3.rs"]
mod vec3;

use vec3::Color;

struct Ppm {
    height: usize,
    width: usize,
    file_name: &'static str,

    maximum_color_value: usize,
    file: Option<File>,

    color_data: Vec<Vec<Color>>,
}

impl Ppm {
    pub fn new(file_name: &'static str, height: usize, width: usize) -> Self {
        Ppm {
            file_name: file_name,
            width: width,
            height: height,
            file: None,
            maximum_color_value: 255,
            color_data: vec![
                vec![
                    Color {
                        x: 0.,
                        y: 0.,
                        z: 0.
                    };
                    width
                ];
                height
            ],
        }
    }
    pub fn create_file(&mut self) -> &mut Self {
        self.file = Some(
            OpenOptions::new()
                .create(true)
                .append(false)
                .write(true)
                .open(self.file_name)
                .expect("meh"),
        );
        self
    }
    pub fn add_header(&mut self) -> &mut Self {
        match &mut self.file {
            Some(f) => {
                let header = format!(
                    "P3\n{} {}\n{}\n",
                    self.height, self.width, self.maximum_color_value
                );
                f.write_all(header.as_bytes()).expect("file write failed");
            }
            None => {
                panic!("file is none");
            }
        }
        self
    }

    pub fn set_color_data(&mut self, x: usize, y: usize, c: Color) -> &mut Self {
        assert!(x < self.height && y < self.width);
        self.color_data[x][y] = c;
        self
    }
    pub fn write_ppm(&mut self) {
        let file = match &mut self.file {
            Some(file) => file,
            None => panic!("file not there"),
        };

        for i in 0..self.height {
            for j in 0..self.width {
                file.write_all(self.color_data[i][j].serialize_color().as_bytes())
                    .expect("failed to write");
            }
        }
    }
}

pub fn main() {
    let mut ppm = Ppm::new("output.ppm", 256, 256);
    ppm.create_file().add_header();

    for i in 0..256 {
        for j in 0..256 {
            print!("\r[scanline remaining: {}]", (256 - i));
            let r = (i as f32) / 255.0;
            let g: f32 = (j as f32) / 255.0;

            ppm.set_color_data(i, j, Color { x: r, y: g, z: 0.0 });
        }
    }
    ppm.write_ppm();

    drop(ppm);
}
