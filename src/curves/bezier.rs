use core::ops::RangeInclusive;

use euclid::default::{Point2D, Vector2D};
use num_traits::{NumOps, ToPrimitive};

use crate::curves::Curve;

#[derive(Debug, Clone)]
pub struct CubicBezier<T: Copy + ToPrimitive>([Point2D<T>; 4]);

impl<T> CubicBezier<T>
where
    T: Copy + ToPrimitive,
{
    pub fn new(
        p0: impl Into<Point2D<T>>,
        p1: impl Into<Point2D<T>>,
        p2: impl Into<Point2D<T>>,
        p3: impl Into<Point2D<T>>,
    ) -> Self {
        Self([p0.into(), p1.into(), p2.into(), p3.into()])
    }

}

impl<T> Curve for CubicBezier<T>
where
    T: Copy + ToPrimitive + NumOps,
{
    fn domain(&self) -> RangeInclusive<f64> {
        (0.)..=1.
    }

    fn evaluate(&self, t: f64) -> Point2D<f64> {
        assert!(self.domain().contains(&t));

        let t2 = t * t;
        let t3 = t2 * t;

        let minus_t = 1. - t;
        let minus_t2 = minus_t * minus_t;
        let minus_t3 = minus_t2 * minus_t;

        let b0 = minus_t3;
        let b1 = 3. * minus_t2 * t;
        let b2 = 3. * minus_t * t;
        let b3 = t3;

        let p0 = Vector2D::<f64>::from((
            self.0[0].x.to_f64().expect("WTF"),
            self.0[0].y.to_f64().expect("WTF"),
        ));
        let p1 = Vector2D::<f64>::from((
            self.0[1].x.to_f64().expect("WTF"),
            self.0[1].y.to_f64().expect("WTF"),
        ));
        let p2 = Vector2D::<f64>::from((
            self.0[2].x.to_f64().expect("WTF"),
            self.0[2].y.to_f64().expect("WTF"),
        ));
        let p3 = Vector2D::<f64>::from((
            self.0[3].x.to_f64().expect("WTF"),
            self.0[3].y.to_f64().expect("WTF"),
        ));

        let out = p0 * b0 + p1 * b1 + p2 * b2 + p3 * b3;

        out.to_point()
    }

    fn evaluate_multiple_ordered(&self, times: impl Iterator<Item = f64>) -> Vec<Point2D<f64>> {
        times.map(|t| self.evaluate(t)).collect()
    }
}