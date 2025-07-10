use std::fmt::Binary;
use std::fs::{File, OpenOptions};
use std::io::Write;

#[path = "../lib/ray.rs"]
mod ray;
#[path = "../lib/vec3.rs"]
mod vec3;

use ray::Ray;
use vec3::Color;
use vec3::Vec3;
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
    let aspect_ratio = 9.0 / 16.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let image_height = if image_height > 1 { image_height } else { 1 };

    let mut ppm = Ppm::new("output.ppm", image_height, image_width);
    ppm.create_file().add_header();

    let focal_length = 1.0 as f32;
    let viewport_height = 2.0 as f32;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

    let camera_center = Vec3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.clone().div(image_width as f32);
    let pixel_delta_v = viewport_v.clone().div(image_height as f32);

    let viewport_upper_left = camera_center
        .clone()
        .add(Vec3::new(0.0, 0.0, focal_length).negate())
        .add(viewport_u.clone().div(2.0).negate())
        .add(viewport_v.clone().div(2.0).negate());

    let pixel00_loc = viewport_upper_left
        .clone()
        .add(pixel_delta_u.clone().add(pixel_delta_v).mul(0.5));

    for i in 0..image_height {
        for j in 0..image_width {
            let pixel_center = pixel00_loc
                .clone()
                .add(pixel_delta_u.clone().mul(i as f32))
                .add(pixel_delta_v.clone().mul(j as f32));

            let ray_direction = pixel_center.clone().add(camera_center.clone().negate());

            let ray = Ray::new(camera_center.clone(), ray_direction.clone());
            let ray_color = ray.get_color() as Color;

            print!("\r[scanline remaining: {}]", (image_height - i));

            ppm.set_color_data(i, j, ray_color);
        }
    }
    ppm.write_ppm();

    drop(ppm);
}
