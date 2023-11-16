use core::{
    cmp::Ordering,
    ops::{DerefMut, RangeInclusive},
};
use image::{ImageBuffer, Rgb};

use crate::curves::Curve;

fn range_size(range: RangeInclusive<f64>) -> f64 {
    let (start, end) = range.into_inner();
    end - start
}

pub fn create_free_diagram<Container>(
    curve1: impl Curve,
    curve2: impl Curve,
    epsilon: f64,
    buffer: &mut ImageBuffer<Rgb<u8>, Container>,
) where
    Container: DerefMut<Target = [u8]>,
{
    // curve1 domain space is the vertical axis
    // curve2 domain space is the horizontal axis
    let (width, height) = buffer.dimensions();

    let domain1 = curve1.domain();
    let domain2 = curve2.domain();

    let step_size1 = range_size(domain1.clone()) / ((height - 1) as f64);
    let step_size2 = range_size(domain2.clone()) / ((width - 1) as f64);

    buffer.enumerate_rows_mut().for_each(|(i, buf)| {
        let p1 = curve1.evaluate((i as f64) * step_size1);
        let points = curve2.evaluate_multiple_ordered((0..width).map(|v| (v as f64) * step_size2));

        buf.zip(points).for_each(|((_i, _j, val), p2)| {
            let dist = p1.distance_to(p2);

            match dist.total_cmp(&epsilon) {
                Ordering::Less => *val = Rgb([u8::MAX, u8::MAX, u8::MAX]),
                Ordering::Equal => *val = Rgb([0, 0, 0]),
                Ordering::Greater => *val = Rgb([153, 153, 153]),
            }
        });
    });
}
