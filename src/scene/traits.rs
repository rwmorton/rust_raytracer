use crate::math::{
    point::Point,
    ray::Ray
};

pub trait Primitive {
    fn hit(&self,ray: &Ray,tmin: &mut f64) -> Option<Point>;
}
