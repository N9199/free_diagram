#![feature(impl_trait_in_assoc_type, array_windows)]

use std::{
    fs::File,
    io::{BufReader, BufWriter, Read},
};

use image::{ImageOutputFormat, RgbImage};

use crate::{config::Config, free_diagram::create_free_diagram};

pub mod curves;
pub mod free_diagram;
pub mod config {
    use serde::Deserialize;

    use crate::curves::{CubicBezier, PolyLine};

    #[derive(Debug, Deserialize)]
    pub struct Config {
        bezier: [(f64, f64); 4],
        polyline: Vec<(f64, f64)>,
        width: u32,
        height: u32,
        epsilon: f64,
    }

    impl Config {
        pub fn get_bezier(&self) -> CubicBezier<f64> {
            self.bezier.into()
        }

        pub fn get_polyline(&self) -> PolyLine<f64> {
            self.polyline.clone().into()
        }

        pub fn get_dimensions(&self) -> (u32, u32) {
            (self.width, self.height)
        }

        pub fn get_epsilon(&self) -> f64 {
            self.epsilon
        }
    }
}
fn main() {
    let mut config_file = BufReader::new(File::open("stuff.toml").unwrap());
    let mut config = String::new();
    config_file.read_to_string(&mut config).unwrap();

    let config: Config = toml::from_str(&config).unwrap();

    let bezier = config.get_bezier();
    let polyline = config.get_polyline();
    let (width, height) = config.get_dimensions();
    let epsilon = config.get_epsilon();

    let mut buffer = RgbImage::new(width, height);
    create_free_diagram(bezier, polyline, epsilon, &mut buffer);
    let mut writer = BufWriter::new(File::create("temp.png").unwrap());
    buffer
        .write_to(&mut writer, ImageOutputFormat::Png)
        .unwrap();
}
