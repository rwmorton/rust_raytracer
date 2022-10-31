use super::traits::Primitive;

use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::math::ray::Ray;

struct World {
    primitives: Vec<Box<dyn Primitive>>
}

impl World {
    /// Construct world
    fn new(num_primitives: usize) -> World {
        World {
            primitives: vec![]
        }
    }

    /// Test
    fn test(&self) {
        let ray = Ray::new(&Point::new(0.,0.,0.),&Vector::new(0.,0.,1.));
        let mut thit: f64 = f64::INFINITY;

        for primitive in self.primitives.iter() {
            let result = primitive.hit(&ray,&mut thit);
            match result {
                Some(hit) => println!("Got a hit point!"),
                None => {}
            }
        }
    }
}
