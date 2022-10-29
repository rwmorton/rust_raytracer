// # Point
// 4 dimensional affine point
//
// # Parameters
// * x
// * y
// * z
// * w
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

// helpful constant
pub const ZERO : Point = Point{x: 0.0,y: 0.0,z: 0.0,w: 0.0};

impl Default for Point {
    fn default() -> Self {
        ZERO
    }
}

impl Point {
    pub fn new() -> Point {
        ZERO
    }
}
