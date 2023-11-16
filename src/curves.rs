use core::ops::RangeInclusive;

use euclid::default::Point2D;

pub trait Curve {
    fn domain(&self) -> RangeInclusive<f64>;
    fn evaluate(&self, t: f64) -> Point2D<f64>;
    fn evaluate_multiple_ordered(&self, times: impl Iterator<Item = f64>) -> Vec<Point2D<f64>>;
}
mod bezier;
mod polyline;

pub use bezier::CubicBezier;
pub use polyline::PolyLine;
