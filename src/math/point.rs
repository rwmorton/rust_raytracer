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

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// UNIT TESTS //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // is ZERO constant correct
    fn test_zero_constant() {
        assert_eq!(ZERO.x,0.);
        assert_eq!(ZERO.y,0.);
        assert_eq!(ZERO.z,0.);
        assert_eq!(ZERO.w,0.);
    }

    #[test]
    // should construct to zero point
    fn test_new() {
        let p: Point = Point::new();
        assert_eq!(p.x,0.);
        assert_eq!(p.y,0.);
        assert_eq!(p.z,0.);
        assert_eq!(p.w,0.);
    }
}
