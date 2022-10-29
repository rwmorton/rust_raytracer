//
// # Vector
// 4 dimensional column vector (w = 0) or point (w != 0)
// 
// # Parameters
// * x
// * y
// * z
// * w
//
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

pub fn print_vector(v: &Vector) {
    println!("({},{},{},{})",v.x,v.y,v.z,v.w);
}

pub fn add(a: &Vector,b: &Vector) -> Vector {
    Vector {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w
    }
}

pub fn dot(a: &Vector,b: &Vector) -> f64 {
    a.x*b.x + a.y*b.y + a.z*b.z + a.w*b.w
}

//
// Vector cross product (always takes w = 0)
//
pub fn cross(a: &Vector,b: &Vector) -> Vector {
    Vector {
        x: a.y*b.z - a.z*b.y,
        y: a.z*b.x - a.x*b.z,
        z: a.x*b.y - a.y*b.x,
        w: 0.0
    }
}

//
// Vector length squared
//
pub fn len_sq(v: &Vector) -> f64 {
    dot(&v,&v)
}

//
// Vector length
//
pub fn len(v: &Vector) -> f64 {
    f64::sqrt(dot(&v,&v))
}

//
// Normalize given vector and return a copy
//
pub fn normalize(v: &Vector) -> Vector {
    let v_len = len(&v);

    assert!(v_len != 0.0);

    Vector {
        x: v.x / v_len,
        y: v.y / v_len,
        z: v.z / v_len,
        w: 0.0
    }
}
