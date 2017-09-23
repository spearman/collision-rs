//! Utilities
//!

use Aabb;
use cgmath::{BaseNum, Vector2, BaseFloat};
use cgmath::prelude::*;
use num::Float;

pub(crate) fn get_max_point<P, T>(vertices: &Vec<P>, direction: &P::Diff, transform: &T) -> P
where
    P: EuclideanSpace,
    P::Scalar: BaseFloat,
    T: Transform<P>,
{
    let direction = transform.inverse_transform().unwrap().transform_vector(
        *direction,
    );
    let (p, _) = vertices.iter().map(|v| (v, v.dot(direction))).fold(
        (P::from_value(P::Scalar::zero()), P::Scalar::neg_infinity()),
        |(max_p, max_dot), (v, dot)| if dot > max_dot {
            (v.clone(), dot)
        } else {
            (max_p, max_dot)
        },
    );
    transform.transform_point(p)
}

pub(crate) fn get_bound<A>(vertices: &Vec<A::Point>) -> A
where
    A: Aabb,
{
    vertices.iter().fold(A::zero(), |bound, p| bound.grow(*p))
}

#[allow(dead_code)]
#[inline]
pub(crate) fn triple_product<S>(a: &Vector2<S>, b: &Vector2<S>, c: &Vector2<S>) -> Vector2<S>
where
    S: BaseNum,
{
    let ac = a.x * c.x + a.y * c.y;
    let bc = b.x * c.x + b.y * c.y;
    Vector2::new(b.x * ac - a.x * bc, b.y * ac - a.y * bc)
}

#[allow(dead_code)]
pub(crate) fn barycentric_vector<V>(p: V, a: V, b: V, c: V) -> (V::Scalar, V::Scalar, V::Scalar)
where
    V: VectorSpace + InnerSpace,
    V::Scalar: BaseFloat,
{
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;
    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);
    let inv_denom = V::Scalar::one() / (d00 * d11 - d01 * d01);

    let v = (d11 * d20 - d01 * d21) * inv_denom;
    let w = (d00 * d21 - d01 * d20) * inv_denom;
    let u = V::Scalar::one() - v - w;
    (u, v, w)
}

#[allow(dead_code)]
pub(crate) fn barycentric_point<P>(p: P, a: P, b: P, c: P) -> (P::Scalar, P::Scalar, P::Scalar)
where
    P: EuclideanSpace,
    P::Diff: InnerSpace,
    P::Scalar: BaseFloat,
{
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;
    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);
    let inv_denom = P::Scalar::one() / (d00 * d11 - d01 * d01);

    let v = (d11 * d20 - d01 * d21) * inv_denom;
    let w = (d00 * d21 - d01 * d20) * inv_denom;
    let u = P::Scalar::one() - v - w;
    (u, v, w)
}

#[cfg(test)]
mod tests {
    use std;

    use cgmath::{Basis2, Point2, Rad, Rotation2, Vector2, Decomposed};

    use super::*;
    use Aabb2;

    #[test]
    fn test_get_bound() {
        let triangle = vec![
            Point2::new(-1., 1.),
            Point2::new(0., -1.),
            Point2::new(1., 0.),
        ];
        assert_eq!(
            Aabb2::new(Point2::new(-1., -1.), Point2::new(1., 1.)),
            get_bound(&triangle)
        );
    }

    fn test_max_point(dx: f32, dy: f32, px: f32, py: f32, rot_angle: f32) {
        let direction = Vector2::new(dx, dy);
        let point = Point2::new(px, py);
        let triangle = vec![
            Point2::new(-1., 1.),
            Point2::new(0., -1.),
            Point2::new(1., 0.),
        ];
        let t = transform(0., 0., rot_angle);
        let max_point = get_max_point(&triangle, &direction, &t);
        assert_ulps_eq!(point.x, max_point.x);
        assert_ulps_eq!(point.y, max_point.y);
    }

    #[test]
    fn test_max_point_1() {
        test_max_point(0., 1., -1., 1., 0.);
    }

    #[test]
    fn test_max_point_2() {
        test_max_point(-1., 0., -1., 1., 0.);
    }

    #[test]
    fn test_max_point_3() {
        test_max_point(0., -1., 0., -1., 0.);
    }

    #[test]
    fn test_max_point_4() {
        test_max_point(1., 0., 1., 0., 0.);
    }

    #[test]
    fn test_max_point_5() {
        test_max_point(10., 1., 1., 0., 0.);
    }

    #[test]
    fn test_max_point_6() {
        test_max_point(2., -100., 0., -1., 0.);
    }

    #[test]
    fn test_max_point_rot() {
        test_max_point(
            0.,
            1.,
            std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::PI / 4.,
        );
    }

    #[test]
    fn test_max_point_disp() {
        let direction = Vector2::new(0., 1.);
        let point = Point2::new(-1., 9.);
        let triangle = vec![
            Point2::new(-1., 1.),
            Point2::new(0., -1.),
            Point2::new(1., 0.),
        ];
        let t = transform(0., 8., 0.);
        assert_eq!(point, get_max_point(&triangle, &direction, &t));
    }

    fn transform(dx: f32, dy: f32, rot: f32) -> Decomposed<Vector2<f32>, Basis2<f32>> {
        Decomposed {
            scale: 1.,
            rot: Rotation2::from_angle(Rad(rot)),
            disp: Vector2::new(dx, dy),
        }
    }
}
