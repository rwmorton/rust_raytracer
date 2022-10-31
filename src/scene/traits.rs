use crate::math::point::Point;
use crate::math::ray::Ray;

pub trait Primitive {
    fn hit(&self,ray: &Ray,tmin: &mut f64) -> Option<Point>;
}
