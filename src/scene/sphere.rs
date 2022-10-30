use super::primitive::Primitive;
use super::traits::Hit;

pub struct Sphere {
    pub super_struct: Primitive
}

impl Hit for Sphere {
    fn hit() -> Point {
        Point {
            x: 0.,
            y: 0.,
            z: 0.
        }
    }
}
