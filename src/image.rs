use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;

#[derive(Clone, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b, a: 1.0}
    }
    pub fn black() -> Color {
        Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0}
    }
    pub fn mul(&self, other: &Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
    pub fn scale(&self, scaler: f64) -> Color {
        Color::new(self.r * scaler, self.g * scaler, self.b * scaler)
    }
    pub fn add(&self, other: &Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
    pub fn is_green(&self) -> bool {
        self.g > self.r && self.g > self.b
    }
}

pub struct Image {
    img: Vec<Color>,
    pub w: usize,
    pub h: usize
}

impl Image {

    pub fn new_rgba(w: usize, h: usize) -> Image {
        let img = vec![Color::black(); w * h];
        Image {img, w, h}
    }

    pub fn set_rgb(&mut self, i: usize, j: usize, c: &Color) {
        self.img[i * self.h + j] = c.clone();
    }

    pub fn at(&self, i: usize, j: usize) -> Color {
        self.img[i * self.w + j].clone()
    }

    fn clip2byte(x: f64) -> u8 {
        if x > 255.0 {
            255
        } else {
            if x < 0.0 {
                0
            } else {
                x as u8
            }
        }
    }

    pub fn flat(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for i in 0 .. self.h {
            for j in 0 .. self.w {
                let c = self.at(i, j);
                let ur = Image::clip2byte(c.r * 255.0);
                let ug = Image::clip2byte(c.g * 255.0);
                let ub = Image::clip2byte(c.b * 255.0);
                let ua = Image::clip2byte(c.a * 255.0);
                bytes.push(ur);
                bytes.push(ug);
                bytes.push(ub);
                bytes.push(ua);
            }
        }
        bytes
    }

    pub fn write_ppm(&self, file: String) {
        let path = Path::new(&file);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.h as u32, self.w as u32);  
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.flat()).unwrap();
    }

}