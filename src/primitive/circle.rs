//! Circle primitive

use cgmath::{Point2, Vector2, BaseFloat};
use cgmath::prelude::*;

use {Aabb2, Ray2};
use prelude::*;
use traits::{ContinuousTransformed, DiscreteTransformed, HasAABB, SupportFunction};

/// Circle primitive
#[derive(Debug, Clone)]
pub struct Circle<S> {
    /// Radius of the circle
    pub radius: S,
}

impl<S> Circle<S> {
    /// Create a new circle primitive
    pub fn new(radius: S) -> Self {
        Self { radius }
    }
}

impl<S> SupportFunction for Circle<S>
where
    S: BaseFloat,
{
    type Point = Point2<S>;

    fn support_point<T>(&self, direction: &Vector2<S>, transform: &T) -> Point2<S>
    where
        T: Transform<Point2<S>>,
    {
        let direction = transform.inverse_transform().unwrap().transform_vector(
            *direction,
        );
        transform.transform_point(Point2::from_vec(direction.normalize_to(self.radius)))
    }
}

impl<S> HasAABB for Circle<S>
where
    S: BaseFloat,
{
    type Aabb = Aabb2<S>;

    fn get_bound(&self) -> Aabb2<S> {
        Aabb2::new(
            Point2::from_value(-self.radius),
            Point2::from_value(self.radius),
        )
    }
}

impl<S> DiscreteTransformed<Ray2<S>> for Circle<S>
where
    S: BaseFloat,
{
    type Point = Point2<S>;

    fn intersects_transformed<T>(&self, ray: &Ray2<S>, transform: &T) -> bool
    where
        T: Transform<Point2<S>>,
    {
        self.intersects(&(
            *ray,
            transform.transform_point(Point2::from_value(S::zero())),
        ))
    }
}

impl<S> Discrete<(Ray2<S>, Point2<S>)> for Circle<S>
where
    S: BaseFloat,
{
    fn intersects(&self, &(ref r, ref center): &(Ray2<S>, Point2<S>)) -> bool {
        let s = self;
        let l = center - r.origin;
        let tca = l.dot(r.direction);
        if tca < S::zero() {
            return false;
        }
        let d2 = l.dot(l) - tca * tca;
        if d2 > s.radius * s.radius {
            return false;
        }
        return true;
    }
}

impl<S> ContinuousTransformed<Ray2<S>> for Circle<S>
where
    S: BaseFloat,
{
    type Point = Point2<S>;
    type Result = Point2<S>;

    #[inline]
    fn intersection_transformed<T>(&self, ray: &Ray2<S>, transform: &T) -> Option<Point2<S>>
    where
        T: Transform<Point2<S>>,
    {
        self.intersection(&(
            *ray,
            transform.transform_point(Point2::from_value(S::zero())),
        ))
    }
}

impl<S> Continuous<(Ray2<S>, Point2<S>)> for Circle<S>
where
    S: BaseFloat,
{
    type Result = Point2<S>;

    fn intersection(&self, &(ref r, ref center): &(Ray2<S>, Point2<S>)) -> Option<Point2<S>> {
        let s = self;

        let l = center - r.origin;
        let tca = l.dot(r.direction);
        if tca < S::zero() {
            return None;
        }
        let d2 = l.dot(l) - tca * tca;
        if d2 > s.radius * s.radius {
            return None;
        }
        let thc = (s.radius * s.radius - d2).sqrt();
        Some(r.origin + r.direction * (tca - thc))
    }
}

#[cfg(test)]
mod tests {
    use std;

    use Ray2;
    use cgmath::{Basis2, Point2, Rad, Rotation2, Vector2, Decomposed};
    use prelude::*;

    use super::*;

    // circle
    #[test]
    fn test_circle_far_1() {
        test_circle(1., 0., 10., 0., 0.);
    }

    #[test]
    fn test_circle_far_2() {
        test_circle(1., 1., 7.0710677, 7.0710677, 0.);
    }

    #[test]
    fn test_circle_far_3() {
        test_circle(1., 0., 10., 0., -std::f32::consts::PI / 4.);
    }

    #[test]
    fn test_circle_far_4() {
        let circle = Circle::new(10.);
        let direction = Vector2::new(1., 0.);
        let transform = transform(0., 10., 0.);
        let point = circle.support_point(&direction, &transform);
        assert_eq!(Point2::new(10., 10.), point);
    }

    #[test]
    fn test_circle_bound() {
        let circle = Circle::new(10.);
        assert_eq!(bound(-10., -10., 10., 10.), circle.get_bound())
    }

    #[test]
    fn test_circle_ray_discrete() {
        let circle = Circle::new(10.);
        let ray = Ray2::new(Point2::new(25., 0.), Vector2::new(-1., 0.));
        let center = Point2::new(0., 0.);
        assert!(circle.intersects(&(ray, center)));
        let center = Point2::new(0., 11.);
        assert!(!circle.intersects(&(ray, center)));
    }

    #[test]
    fn test_circle_ray_discrete_transformed() {
        let circle = Circle::new(10.);
        let ray = Ray2::new(Point2::new(25., 0.), Vector2::new(-1., 0.));
        let t = transform(0., 0., 0.);
        assert!(circle.intersects_transformed(&ray, &t));
        let t = transform(0., 11., 0.);
        assert!(!circle.intersects_transformed(&ray, &t));
    }

    #[test]
    fn test_circle_ray_continuous() {
        let circle = Circle::new(10.);
        let ray = Ray2::new(Point2::new(25., 0.), Vector2::new(-1., 0.));
        let center = Point2::new(0., 0.);
        assert_eq!(
            Some(Point2::new(10., 0.)),
            circle.intersection(&(ray, center))
        );
        let center = Point2::new(0., 11.);
        assert_eq!(None, circle.intersection(&(ray, center)));
    }

    #[test]
    fn test_circle_ray_continuous_transformed() {
        let circle = Circle::new(10.);
        let ray = Ray2::new(Point2::new(25., 0.), Vector2::new(-1., 0.));
        let t = transform(0., 0., 0.);
        assert_eq!(
            Some(Point2::new(10., 0.)),
            circle.intersection_transformed(&ray, &t)
        );
        let t = transform(0., 11., 0.);
        assert_eq!(None, circle.intersection_transformed(&ray, &t));
    }

    fn test_circle(dx: f32, dy: f32, px: f32, py: f32, rot: f32) {
        let circle = Circle::new(10.);
        let direction = Vector2::new(dx, dy);
        let transform = transform(0., 0., rot);
        let point = circle.support_point(&direction, &transform);
        assert_ulps_eq!(px, point.x);
        assert_ulps_eq!(py, point.y);
    }

    // util
    fn transform(dx: f32, dy: f32, rot: f32) -> Decomposed<Vector2<f32>, Basis2<f32>> {
        Decomposed {
            scale: 1.,
            rot: Rotation2::from_angle(Rad(rot)),
            disp: Vector2::new(dx, dy),
        }
    }

    fn bound(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Aabb2<f32> {
        Aabb2::new(Point2::new(min_x, min_y), Point2::new(max_x, max_y))
    }
}
