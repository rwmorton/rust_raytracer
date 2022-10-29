// # Vector
// 3 dimensional column vector (w = 0)
// 
// # Parameters
// * x
// * y
// * z
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

// Helpful constants
pub const ZERO  : Vector = Vector{x: 0.0,y: 0.0,z: 0.0};
pub const X_AXIS: Vector = Vector{x: 1.0,y: 0.0,z: 0.0};
pub const Y_AXIS: Vector = Vector{x: 0.0,y: 1.0,z: 0.0};
pub const Z_AXIS: Vector = Vector{x: 0.0,y: 0.0,z: 1.0};

impl Default for Vector {
    // default
    fn default() -> Self {
        ZERO
    }
}

impl Vector {
    // Defaults to zero vector
    pub fn new(x: f64,y: f64,z: f64) -> Vector {
        Vector {x,y,z}
    }

    /*pub fn new(
        x: Option<f64>,
        y: Option<f64>,
        z: Option<f64>
    ) -> Vector {
        let mut _x: f64 = 0.0;
        let mut _y: f64 = 0.0;
        let mut _z: f64 = 0.0;

        match (x,y,z) {
            Some(x,y,z) => Vector{x: _x,y: _y,z: _z},
            None => Vector{0.,0.,0.}
        }

        // match (x,y,z) {
        //     Some(x,y,z) => Vector{x: x,y: y,z: z},
        //     None => ZERO
        // }
    }*/

    // Print vector
    pub fn print(&self) {
        println!("({},{},{})",self.x,self.y,self.z);
    }

    // Sum with another vector (immutable)
    pub fn add(&self,v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }

    // Cubtract another vector from self (immutable)
    pub fn sub(&self,v: &Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }

    // Scaled copy
    pub fn scale(&self,scale: f64) -> Vector {
        Vector {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }

    // Dot product
    pub fn dot(&self,v: &Vector) -> f64 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }

    // Cross product
    pub fn cross(&self,v: &Vector) -> Vector {
        Vector {
            x: self.y*v.z - self.z*v.y,
            y: self.z*v.x - self.x*v.z,
            z: self.x*v.y - self.y*v.x
        }
    }

    // Vector length squared
    pub fn len_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    // Vector length
    pub fn len(&self) -> f64 {
        f64::sqrt(Vector::len_sq(self))
    }

    // Normalized copy
    pub fn normalize(&self) -> Vector {
        let v_len = Vector::len(&self);
    
        assert!(v_len != 0.0);
    
        Vector {
            x: self.x / v_len,
            y: self.y / v_len,
            z: self.z / v_len
        }
    }
}

// Normalize a vector (mutable)
pub fn normalize(v: &mut Vector) {
    let len: f64 = v.len();

    assert!(len != 0.0);
    
    let recip: f64 = 1.0 / len;

    v.x *= recip;
    v.y *= recip;
    v.z *= recip;
}
