mod math;

use math::point::Point;

trait CanHit {
    fn hit(&self) -> Point;
}

pub struct Primitive {
    //
}
