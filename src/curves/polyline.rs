use core::ops::RangeInclusive;

use euclid::default::{Point2D, Vector2D};
use num_traits::{NumOps, ToPrimitive};

use crate::curves::Curve;

#[derive(Debug, Clone)]
pub struct PolyLine<T: Copy + ToPrimitive> {
    points: Vec<Point2D<T>>,
    length: f64,
}

impl<T> PolyLine<T>
where
    T: Copy + ToPrimitive + NumOps,
{
    pub fn new(points: Vec<impl Into<Point2D<T>>>) -> Self {
        let points: Vec<Point2D<T>> = points.into_iter().map(|v| v.into()).collect();
        let length = points
            .array_windows::<2>()
            .map(|[p1, p2]| (*p2 - *p1).square_length().to_f64().expect("WTF").sqrt())
            .sum();
        Self { points, length }
    }
}

impl<T> Curve for PolyLine<T>
where
    T: Copy + ToPrimitive + NumOps,
{
    fn domain(&self) -> RangeInclusive<f64> {
        (0.)..=self.length
    }

    fn evaluate(&self, t: f64) -> Point2D<f64> {
        assert!(self.domain().contains(&t));
        let mut length = 0.;
        for [p1, p2] in self.points.array_windows() {
            let (p1, p2) = (*p1, *p2);
            let delta = p2 - p1;
            let delta_length = delta
                .dot(delta)
                .to_f64()
                .expect("Some fucked conversion stuff")
                .sqrt();

            if length + delta_length >= t {
                let t = t - length;
                let minus_t = 1. - t;
                let p1 = Vector2D::<f64>::from((
                    p1.x.to_f64().expect("WTF"),
                    p1.y.to_f64().expect("WTF"),
                ));
                let p2 = Vector2D::<f64>::from((
                    p2.x.to_f64().expect("WTF"),
                    p2.y.to_f64().expect("WTF"),
                ));
                return (p1 * minus_t + p2 * t).to_point();
            }

            length += delta_length;
        }
        panic!("t out of domain")
    }

    fn evaluate_multiple_ordered(&self, times: impl Iterator<Item = f64>) -> Vec<Point2D<f64>> {
        let mut length = 0.;
        let mut out = Vec::new();

        let mut times = times.peekable();

        for [p1, p2] in self.points.array_windows() {
            let (p1, p2) = (*p1, *p2);
            let delta = p2 - p1;
            let delta_length = delta
                .dot(delta)
                .to_f64()
                .expect("Some fucked conversion stuff")
                .sqrt();

            while let Some(t) = times.peek() {
                let t = *t;
                if length <= t && t <= length + delta_length {
                    let t = t - length;
                    let t = t / delta_length;
                    let minus_t = 1. - t;
                    let p1 = Vector2D::<f64>::from((
                        p1.x.to_f64().expect("WTF"),
                        p1.y.to_f64().expect("WTF"),
                    ));
                    let p2 = Vector2D::<f64>::from((
                        p2.x.to_f64().expect("WTF"),
                        p2.y.to_f64().expect("WTF"),
                    ));
                    out.push((p1 * minus_t + p2 * t).to_point());
                    times.next();
                } else {
                    break;
                }
            }
            length += delta_length;
        }
        out
    }
}
