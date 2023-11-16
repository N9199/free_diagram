#![feature(impl_trait_in_assoc_type, array_windows)]

use std::{fs::File, io::BufWriter};

use image::{ImageOutputFormat, RgbImage};

use crate::{
    curves::{CubicBezier, PolyLine},
    free_diagram::create_free_diagram,
};

pub mod curves;
pub mod free_diagram;

fn main() {
    let bezier = CubicBezier::new((0, 0), (10, 10), (20, 10), (30, 0));
    let polyline = PolyLine::new(vec![
        (0, 0),
        (5, 5),
        (7, 6),
        (15, 8),
        (23, 6),
        (25, 5),
        (30, 0),
    ]);
    let size = 2048;
    let mut buffer = RgbImage::new(size, size);
    create_free_diagram(bezier, polyline, 5., &mut buffer);
    let mut writer = BufWriter::new(File::create("temp.png").unwrap());
    buffer
        .write_to(&mut writer, ImageOutputFormat::Png)
        .unwrap();
}
